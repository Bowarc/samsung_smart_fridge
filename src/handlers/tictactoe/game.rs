use serenity::all::{ButtonStyle, CreateActionRow, CreateButton, Mentionable};

use super::id::GridButtonId;

use {
    super::{cell, position::CellPos, turn::GameTurn},
    crate::{error::Error, handlers::TicTacToeError},
    serenity::all::{CacheHttp, ChannelId, CreateMessage, EditMessage, Message, UserId},
};

pub type GameId = uid::IdU16<Game>;

#[derive(Copy, Clone, PartialEq)]
pub enum GameState {
    Running,
    Tie,
    Won(UserId),
}

impl GameState {
    fn is_done(&self) -> bool {
        !matches!(self, Self::Running)
    }
}

#[derive(Clone)] // For the uid:IdU16 impl
pub struct Game {
    game_id: uid::IdU16<Self>,

    state: GameState,

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
        request_message: &Message,
        player1_id: UserId,
        player2_id: UserId,
    ) -> Result<Self, Error> {
        let game_id = uid::IdU16::new();
        let owner = player1_id; // Technically not usefull as we could use player1, but explicit is better

        let board_message = request_message.reply(
            cache_http,
            format!("Generating TicTacToe game. . .\nGameId: {game_id}\nPlayers: {player1_id} - {player2_id}\nOwner: {owner}")
            )
            .await?;

        let mut game = Self {
            game_id,
            state: GameState::Running,

            player1_id,
            player2_id,
            owner,

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

    pub fn turn(&self) -> &GameTurn {
        &self.turn
    }

    pub fn state(&self) -> &GameState {
        &self.state
    }

    pub fn current_player_id(&self) -> UserId {
        match self.turn {
            GameTurn::Player1 => self.player1_id,
            GameTurn::Player2 => self.player2_id,
        }
    }

    pub fn check_win(&self) -> Option<Vec<CellPos>> {
        let mut out = Vec::with_capacity(5);

        let mut push_unique = |p: CellPos| {
            if !out.contains(&p) {
                out.push(p);
            }
        };

        // Horizontal
        for (i, row) in self.board.iter().enumerate() {
            if row[0] == row[1] && row[0] == row[2] && row[0] != cell::State::Empty {
                push_unique(CellPos::new(0.try_into().unwrap(), i.try_into().unwrap()));
                push_unique(CellPos::new(1.try_into().unwrap(), i.try_into().unwrap()));
                push_unique(CellPos::new(2.try_into().unwrap(), i.try_into().unwrap()));
            }
        }

        // vertical
        for i in 0..3 {
            if self.board[0][i] == self.board[1][i]
                && self.board[0][i] == self.board[2][i]
                && self.board[0][i] != cell::State::Empty
            {
                push_unique(CellPos::new(i.try_into().unwrap(), 0.try_into().unwrap()));
                push_unique(CellPos::new(i.try_into().unwrap(), 1.try_into().unwrap()));
                push_unique(CellPos::new(i.try_into().unwrap(), 2.try_into().unwrap()));
            }
        }

        // Topleft botright diagonal
        if self.board[0][0] == self.board[1][1]
            && self.board[0][0] == self.board[2][2]
            && self.board[0][0] != cell::State::Empty
        {
            push_unique(CellPos::new(0.try_into().unwrap(), 0.try_into().unwrap()));
            push_unique(CellPos::new(1.try_into().unwrap(), 1.try_into().unwrap()));
            push_unique(CellPos::new(2.try_into().unwrap(), 2.try_into().unwrap()));
        }

        // Topright botleft diagonal
        if self.board[2][0] == self.board[1][1]
            && self.board[2][0] == self.board[0][2]
            && self.board[2][0] != cell::State::Empty
        {
            push_unique(CellPos::new(0.try_into().unwrap(), 2.try_into().unwrap()));
            push_unique(CellPos::new(1.try_into().unwrap(), 1.try_into().unwrap()));
            push_unique(CellPos::new(2.try_into().unwrap(), 0.try_into().unwrap()));
        }

        if out.is_empty() {
            None
        } else {
            Some(out)
        }
    }

    // The caller should make sure that the user is playing in this game
    pub async fn play(
        &mut self,
        cache_http: &impl CacheHttp,
        player: UserId,
        position: &CellPos,
    ) -> Result<(), TicTacToeError> {
        assert!(self.players().contains(&&player));

        if self.state.is_done() {
            return Err(TicTacToeError::GameEnded);
        }

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

        if !self
            .board
            .iter()
            .any(|row| row.iter().any(|cell| cell == &cell::State::Empty))
        {
            self.state = GameState::Tie
        }

        if self.check_win().is_some() {
            self.state = GameState::Won(self.current_player_id())
        }

        debug!("Updated board: {:?}", self.board);

        self.turn = !self.turn;

        self.render(cache_http).await;

        Ok(())
    }

    async fn render(&mut self, cache_http: &impl CacheHttp) {
        let player_name = async |userid: UserId| -> String {
            cache_http
                .http()
                .get_user(userid)
                .await
                .map(|user| {
                    user.global_name
                        .clone()
                        .unwrap_or(user.mention().to_string())
                })
                .unwrap_or(format!(
                    "Player {}",
                    match self.turn {
                        GameTurn::Player1 => 1,
                        GameTurn::Player2 => 2,
                    }
                ))
        };

        let win_cells = self.check_win().unwrap_or_default();

        let mut components = Vec::with_capacity(3);
        for (y, row) in self.board.iter().enumerate() {
            let mut action_row = Vec::with_capacity(3);
            for (x, state) in row.iter().enumerate() {
                let cell_pos = CellPos::new(x.try_into().unwrap(), y.try_into().unwrap());
                // Thoses unwrap are fine because the board the board is 3x3, so x and y will be contained in 0..=2
                let id = GridButtonId::new(self.game_id.clone(), cell_pos);

                let cb = CreateButton::new(id)
                    .label(state)
                    .disabled(self.state.is_done());

                let style = match self.state {
                    GameState::Running => ButtonStyle::Primary,
                    GameState::Tie => ButtonStyle::Secondary,
                    GameState::Won(_) => {
                        if win_cells.contains(&cell_pos) {
                            ButtonStyle::Success
                        } else {
                            ButtonStyle::Secondary
                        }
                    }
                };

                action_row.push(cb.style(style))
                // action_row.push(CreateButton::new(id).label(' '))
            }
            components.push(CreateActionRow::Buttons(action_row));
        }

        let global_message_content: String;

        match self.state {
            GameState::Running => {
                let mark_current = |id: UserId, name: String| -> String {
                    if self.current_player_id() == id {
                        format!("**__{name}__**")
                    } else {
                        name
                    }
                };
                let p1 = mark_current(self.player1_id, player_name(self.player1_id).await);
                let p2 = mark_current(self.player2_id, player_name(self.player2_id).await);

                global_message_content = format!("{p1} vs {p2}");
            }
            GameState::Tie => {
                global_message_content = format!(
                    "{} tied vs {}",
                    player_name(self.player1_id).await,
                    player_name(self.player2_id).await
                );
            }
            GameState::Won(winner_id) => {
                let winner = player_name(winner_id).await;
                let loser = player_name(match self.turn {
                    GameTurn::Player1 => self.player1_id,
                    GameTurn::Player2 => self.player2_id,
                })
                .await;
                global_message_content = format!("{winner} won vs {loser}");
            }
        }

        self.board_message
            .edit(
                cache_http,
                EditMessage::new()
                    .components(components)
                    .content(global_message_content),
            )
            .await
            .unwrap();
    }
}
