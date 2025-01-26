use serenity::prelude::TypeMapKey;

#[derive(Default)]
pub struct TicTacToeData {
    pub games: Vec<super::game::TicTacToeGame>,
}

impl TypeMapKey for TicTacToeData {
    type Value = std::sync::Arc<tokio::sync::RwLock<TicTacToeData>>;
}
