#[derive(Debug, thiserror::Error)]
pub enum TicTacToeError{
    #[error("The player is already in a game")]
    PlayerAlreadyInAGame,
    #[error("Could not retreive the storage data")]
    StorageFetch,
    #[error("Cannot play vs yourself")]
    CantPlayVsYourself,
    #[error("Cannot play alone")]
    CantPlayAlone
    
}
