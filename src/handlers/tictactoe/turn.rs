#[derive(Clone, Copy, PartialEq)]
pub enum GameTurn {
    Player1,
    Player2,
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
