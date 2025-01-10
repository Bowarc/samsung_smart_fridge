use crate::handlers::TicTacToeError;

#[derive(thiserror::Error, Debug)]
pub enum Error{
    #[error("Serenity error: {0}")]
    Serenity(#[from] serenity::Error),
    #[error("TicTacToe error: {0}")]
    TicTacToe(#[from] TicTacToeError)
}
