mod game;
mod error;

pub use error::TicTacToeError;

use serenity::{
    all::{Context, EventHandler, Message, Ready},
    prelude::TypeMapKey,
};

pub struct TicTacToe;

#[derive(Default)]
pub struct TicTacToeData {
    games: Vec<game::TicTacToeGame>,
}

impl TypeMapKey for TicTacToeData {
    type Value = std::sync::Arc<tokio::sync::RwLock<TicTacToeData>>;
}

impl TicTacToe {}

#[serenity::async_trait]
impl EventHandler for TicTacToe {
    async fn ready(&self, ctx: Context, _data_about_bot: Ready) {
        ctx.data
            .write()
            .await
            .insert::<TicTacToeData>(std::sync::Arc::new(tokio::sync::RwLock::new(
                TicTacToeData::default(),
            )));
    }
    async fn message(&self, ctx: Context, message: Message) {
        if super::is_command(&message, "Tplay", super::Case::Sensitive) {
            if let Err(why) = join(ctx, message).await {
                error!("{why}")
            }
        }
    }
}

async fn join(ctx: Context, message: Message) -> Result<(), crate::error::Error> {
    let Some(data) = ctx.data.read().await.get::<TicTacToeData>().cloned() else {
        error!("Could not get TicTacToe data from storage");
        do yeet TicTacToeError::StorageFetch;
    };

    let player_in_a_game = {
        let data_view = data.read().await;

        data_view
            .games
            .iter()
            .any(|tttgame| tttgame.players().contains(&&message.author.id))
    };

    if player_in_a_game {
        message
            .reply(&ctx.http, "You are already in a game !")
            .await?;
        do yeet TicTacToeError::PlayerAlreadyInAGame;
    }

    if message.mentions.len() != 1 {
        message
            .reply(
                &ctx.http,
                "Expected 1 argument, please specify a player you wanna player against",
            )
            .await?;
        do yeet TicTacToeError::CantPlayAlone;
    }

    // The unwrap is fine, as we checked the len right above
    let enemy = message.mentions.first().unwrap();

    if enemy.id == message.author.id {
        message
            .reply(&ctx.http, "You cannot play vs yourself")
            .await?;
        do yeet TicTacToeError::CantPlayVsYourself;
    }

    message
        .reply(
            &ctx.http,
            format!("You have been registered to play against {}", enemy.name),
        )
        .await?;

    let game = game::TicTacToeGame::new(message.author.id, enemy.id, message.author.id);

    {
        data.write().await.games.push(game);
    }

    Ok(())
}
