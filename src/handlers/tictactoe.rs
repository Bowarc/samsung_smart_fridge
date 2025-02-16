mod cell;
mod data;
mod error;
mod game;
mod id;
mod position;
mod turn;

use crate::command;
use data::TicTacToeData;
pub use error::TicTacToeError;

use game::GameState;
use id::GridButtonId;
use serenity::all::{
    Context, CreateInteractionResponseMessage, EventHandler, Interaction, Message, Ready,
};
use turn::GameTurn;

pub struct TicTacToe;

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
        'join: {
            let Some(args) = command::parse(
                &message,
                "Tplay",
                command::Case::Sensitive,
                command::Prefix::Yes,
            ) else {
                break 'join;
            };

            if args.len() != 1 {
                if let Err(why) = message
                    .reply(
                        &ctx,
                        format!(
                            "Tplay command received, but got unexpected arguments: {:?}",
                            &args[1..]
                        ),
                    )
                    .await
                {
                    error!("Failed to send error message due to: {why}");
                }
                break 'join;
            }

            if let Err(why) = join(&ctx, &message).await {
                error!("{why}")
            }
        };

        'help: {
            let Some(args) = command::parse(
                &message,
                "Thelp",
                command::Case::Sensitive,
                command::Prefix::Yes,
            ) else {
                break 'help;
            };

            if args.len() > 1 {
                error!(
                    "Thelp command received, but got unexpected arguments: {:?}",
                    &args[1..]
                );
                break 'help;
            }

            if let Err(why) = message
                .reply(&ctx, "TEMPORARY HELP MESSAGE,\nFUCK YOU")
                .await
            {
                error!("{why}")
            }
        };

        't: {
            let Some(_args) = command::parse(
                &message,
                "T",
                command::Case::Sensitive,
                command::Prefix::Yes,
            ) else {
                break 't;
            };
        }
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        let Some(data) = ctx.data.read().await.get::<TicTacToeData>().cloned() else {
            error!("Could not get TicTacToe data from storage");
            return;
        };

        let Interaction::Component(c) = interaction else {
            warn!("Ignored interaction: {interaction:?}");
            return;
        };

        debug!("Button interaction: {:?}", c.data.kind);

        // Needed to make sure the user's client doesn't display a small error message

        let Ok(button_id) = GridButtonId::try_from(&c.data.custom_id) else {
            if let Err(e) = c
                .create_response(&ctx, serenity::all::CreateInteractionResponse::Acknowledge)
                .await
            {
                error!(
                    "Failed to acknoledge component interaction {} due to: {e}",
                    c.id
                )
            }

            panic!(
                "Failed to convert custom id to a grid button id: '{}'",
                c.data.custom_id
            )
        };

        let mut data_writer = data.write().await;

        let Some(game) = data_writer
            .games
            .iter_mut()
            .find(|g| g.players().contains(&&c.user.id))
        else {
            if let Err(e) = c
                .create_response(&ctx, serenity::all::CreateInteractionResponse::Acknowledge)
                .await
            {
                error!(
                    "Failed to acknoledge component interaction {} due to: {e}",
                    c.id
                )
            }
            return;
        };

        if game.current_player_id() != c.user.id {
            warn!(
                "Player {} tried to play, but it wasn't their turn, gameid: {}",
                ctx.http
                    .get_user(c.user.id)
                    .await
                    .map(|user| user.global_name.clone().unwrap_or(user.name))
                    .unwrap_or(format!(
                        "Player {}",
                        match game.turn() {
                            GameTurn::Player1 => 2,
                            GameTurn::Player2 => 1,
                        }
                    )),
                game.id()
            );
            if let Err(e) = c
                .create_response(
                    ctx,
                    serenity::all::CreateInteractionResponse::Message(
                        CreateInteractionResponseMessage::new()
                            .ephemeral(true)
                            .content("Wait your turn"),
                    ),
                )
                .await
            {
                error!("Failed to send component inreraction message due to: {e}");
            }
            return;
        }

        if let Err(e) = game.play(&ctx, c.user.id, button_id.cell_position()).await {
            error!("{e}");
        };

        if let Err(e) = c
            .create_response(&ctx, serenity::all::CreateInteractionResponse::Acknowledge)
            .await
        {
            error!(
                "Failed to acknoledge component interaction {} due to: {e}",
                c.id
            )
        }

        // Remove all games that are done
        data_writer
            .games
            .retain(|game| matches!(game.state(), GameState::Running));
    }
}

async fn join(ctx: &Context, message: &Message) -> Result<(), crate::error::Error> {
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
    let player2 = message.mentions.first().unwrap();

    // TODO: Disabled for testing purpose, please don't leave this commented
    if player2.id == message.author.id {
        message
            .reply(&ctx.http, "You cannot play vs yourself")
            .await?;
        do yeet TicTacToeError::CantPlayVsYourself;
    }

    message
        .reply(
            &ctx.http,
            format!("You have been registered to play against {}", player2.name),
        )
        .await?;

    let game =
        match game::Game::init_new(ctx, message, message.author.id, player2.id).await {
            Ok(game) => game,
            Err(e) => {
                panic!("{e}")
            }
        };

    {
        data.write().await.games.push(game);
    }

    Ok(())
}
