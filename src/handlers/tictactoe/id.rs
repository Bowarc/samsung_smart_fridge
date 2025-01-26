pub struct TicTacToeButtonMessageId([String; 1]);

impl TicTacToeButtonMessageId{
    pub fn new(gameid: uuid::Uuid, grid_position: super::position::CellPos) -> Self {
        
        Self([gameid.hyphenated().to_string()])
    }
}

