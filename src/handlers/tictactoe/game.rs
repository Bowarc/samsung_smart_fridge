use serenity::all::{CreateActionRow, CreateButton, Mentionable};

use super::id::GridButtonId;

use {
    super::{cell, position::CellPos, turn::GameTurn},
    crate::{error::Error, handlers::TicTacToeError},
    serenity::all::{CacheHttp, ChannelId, CreateMessage, EditMessage, Message, UserId},
};


pub type GameId = uid::IdU16<Game>;

#[derive(Clone)] // For the uid:IdU16 impl
pub struct Game {
    game_id: uid::IdU16<Self>,

    player1_id: UserId,
    player2_id: UserId,

    owner: UserId,

    turn: GameTurn,

    board: [[cell::State; 3]; 3],

    board_message: Message,
}

impl Game {
    pub async fn init_new(
        cache_http: &impl CacheHttp,
        channel_id: ChannelId,
        player1_id: UserId,
        player2_id: UserId,
    ) -> Result<Self, Error> {
        let game_id = uid::IdU16::new();

        let board_message = channel_id
            .send_message(
                cache_http,
                CreateMessage::new().content("Generating TicTacToe game. . ."),
            )
            .await?;

        let mut game = Self {
            game_id,
            player1_id,
            player2_id,
            owner: player1_id, // Technically not usefull as we could use player1, but it's better imo

            turn: if random::conflip() {
                GameTurn::Player1
            } else {
                GameTurn::Player2
            },

            board: Default::default(),
            board_message,
        };

        game.render(cache_http).await;

        Ok(game)
    }
    pub fn id(&self) -> &uid::IdU16<Self> {
        &self.game_id
    }

    pub fn owner_id(&self) -> UserId {
        self.owner
    }

    pub fn players(&self) -> [&UserId; 2] {
        [&self.player1_id, &self.player2_id]
    }

    // The caller should make sure that the user is playing in this game
    pub async fn play(
        &mut self,
        cache_http: &impl CacheHttp,
        player: UserId,
        position: &CellPos,
    ) -> Result<(), TicTacToeError> {
        assert!(self.players().contains(&&player));

        if match self.turn {
            GameTurn::Player1 => self.player1_id,
            GameTurn::Player2 => self.player2_id,
        } != player
        {
            return Err(TicTacToeError::NotYourTurn);
        }

        let row = self.board.get_mut(usize::from(position.y())).unwrap();
        let cell = row.get_mut(usize::from(position.x())).unwrap();

        *cell = match self.turn {
            GameTurn::Player1 => cell::State::P1,
            GameTurn::Player2 => cell::State::P2,
        };

        debug!("Updated board: {:?}", self.board);

        self.turn = !self.turn;

        self.render(cache_http).await;

        Ok(())
    }

    async fn render(&mut self, cache_http: &impl CacheHttp) {
        let mut components = Vec::with_capacity(3);
        for (y, row) in self.board.iter().enumerate() {
            let mut action_row = Vec::with_capacity(3);
            for (x, state) in row.iter().enumerate() {
                // Thoses unwrap are fine because the board the board is 3x3, so x and y will be contained in 0..=2
                let id = GridButtonId::new(
                    self.game_id.clone(),
                    CellPos::new(x.try_into().unwrap(), y.try_into().unwrap()),
                );
                action_row.push(CreateButton::new(id).label(state))
                // action_row.push(CreateButton::new(id).label(' '))
            }
            components.push(CreateActionRow::Buttons(action_row));
        }

        self.board_message
            .edit(
                cache_http,
                EditMessage::new().components(components).content({
                    let get_label = async |user: UserId, fallback: &str| -> String {
                        let base = cache_http
                            .http()
                            .get_user(user)
                            .await
                            .map(|user| user.mention().to_string())
                            .unwrap_or(fallback.to_string());

                        if match self.turn {
                            GameTurn::Player1 => self.player1_id,
                            GameTurn::Player2 => self.player2_id,
                        } == user
                        {
                            format!("({base})")
                        } else {
                            base
                        }
                    };

                    let p1_label = get_label(self.player1_id, "Player 1").await;
                    let p2_label = get_label(self.player2_id, "Player 2").await;

                    format!("{p1_label} - {p2_label}")
                }),
            )
            .await
            .unwrap();
    }
}
