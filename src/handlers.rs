mod basic;
mod logger;
mod ping;
mod purge;
mod tictactoe;
mod why;

pub use basic::Basic;
pub use logger::Logger;
pub use ping::Ping;
pub use purge::Purge;
pub use tictactoe::{TicTacToe, TicTacToeError};
pub use why::Why;

#[derive(PartialEq)]
pub enum Case {
    Sensitive,
    Insensitive,
}

pub fn is_command<'a>(message: &'a serenity::all::Message, command: &str, case: Case) -> bool {
    message
        .content
        .split(' ')
        .next()
        .map(ToString::to_string)
        .map(|c| {
            if case == Case::Insensitive {
                c.to_lowercase()
            } else {
                c
            }
        })
        == if case == Case::Insensitive {
            Some(command.to_lowercase().to_string())
        } else {
            Some(command.to_string())
        }
}
