use serenity::all::UserId;

#[derive(Clone, Copy)]
pub enum GameTurn {
    Player1,
    Player2,
}

pub struct TicTacToeGame {
    game_id: uuid::Uuid,
    
    player1_id: UserId,
    player2_id: UserId,

    owner: UserId,

    turn: GameTurn,

    board: [[CellState; 3]; 3],
}

#[derive(Default, Copy, Clone)]
enum CellState {
    #[default]
    Empty,
    P1,
    P2,
}


impl TicTacToeGame {
    pub fn new(player1_id: UserId, player2_id: UserId, owner: UserId) -> Self {
        Self {
            game_id: uuid::Uuid::new_v4(),
            player1_id,
            player2_id,
            owner,

            turn: if random::conflip() {
                GameTurn::Player1
            } else {
                GameTurn::Player2
            },

            board: Default::default(),
        }
    }

    pub fn board_emoji_display(&self) -> String {
        let mut out = String::new();

        self.board.iter().for_each(|row| {
            row.iter().for_each(|cell| {
                out.push_str(cell.emoji());
            });
            out.push('\n');
        });

        out
    }

    pub fn players(&self) -> Vec<&UserId> {
        vec![&self.player1_id, &self.player2_id]
    }
}

impl std::ops::Not for GameTurn {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Self::Player1 => Self::Player2,
            Self::Player2 => Self::Player1,
        }
    }
}

impl CellState {
    fn emoji(&self) -> &str {
        match self {
            CellState::Empty => ":white_large_square:",
            CellState::P1 => ":regional_indicator_x:",
            CellState::P2 => ":o2:",
        }
    }
}

impl From<&CellState> for char {
    fn from(state: &CellState) -> Self {
        match state {
            CellState::Empty => ' ',
            CellState::P1 => 'X',
            CellState::P2 => 'O',
        }
    }
}

impl std::fmt::Display for CellState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", char::from(self))
    }
}
