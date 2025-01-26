mod data;
mod error;
mod game;
mod id;
mod position;

use crate::command;
use data::TicTacToeData;
pub use error::TicTacToeError;

use serenity::all::{
    Context, CreateActionRow, CreateButton, CreateInteractionResponseMessage, CreateMessage,
    EventHandler, Interaction, Message, Ready,
};

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
            let Some(bits) = command::parse(
                &message,
                "Tplay",
                command::Case::Sensitive,
                command::Prefix::Yes,
            ) else {
                break 'join;
            };

            if bits.len() > 2 {
                if let Err(why) = message
                    .reply(
                        &ctx,
                        format!(
                            "Tplay command received, but got unexpected arguments: {:?}",
                            &bits[1..]
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
            let Some(bits) = command::parse(
                &message,
                "Thelp",
                command::Case::Sensitive,
                command::Prefix::Yes,
            ) else {
                break 'help;
            };

            if bits.len() > 1 {
                error!(
                    "Thelp command received, but got unexpected arguments: {:?}",
                    &bits[1..]
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
            let Some(bits) = command::parse(
                &message,
                "T",
                command::Case::Sensitive,
                command::Prefix::Yes,
            ) else {
                break 't;
            };

            message
                .channel_id
                .send_message(
                    &ctx,
                    CreateMessage::new().components(vec![
                        CreateActionRow::Buttons(vec![
                            CreateButton::new(format!("00")).label(format!("00")),
                            CreateButton::new(format!("01")).label(format!("01")),
                            CreateButton::new(format!("02")).label(format!("02")),
                        ]),
                        CreateActionRow::Buttons(vec![
                            CreateButton::new(format!("10")).label(format!("10")),
                            CreateButton::new(format!("11")).label(format!("11")),
                            CreateButton::new(format!("12")).label(format!("12")),
                        ]),
                        CreateActionRow::Buttons(vec![
                            CreateButton::new(format!("20")).label(format!("20")),
                            CreateButton::new(format!("21")).label(format!("21")),
                            CreateButton::new(format!("22")).label(format!("22")),
                        ]),
                    ]),
                )
                .await
                .unwrap();
        }
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        let Interaction::Component(c) = interaction else {
            panic!()
        };


        // Needed to make sure the user's client doesn't display a small error message
        c.create_response(&ctx, serenity::all::CreateInteractionResponse::Acknowledge)
            .await
            .unwrap();

        debug!("Acknowledged {}", c.data.custom_id);

        c.create_response(
            &ctx,
            serenity::all::CreateInteractionResponse::UpdateMessage(
                CreateInteractionResponseMessage::new().components(vec![
                    CreateActionRow::Buttons(vec![
                        CreateButton::new(format!("00")).label(format!("00")),
                        CreateButton::new(format!("01")).label(format!("01")),
                        CreateButton::new(format!("02")).label(format!("02")),
                    ]),
                    CreateActionRow::Buttons(vec![
                        CreateButton::new(format!("10")).label(format!("10")),
                        CreateButton::new(format!("11")).label(format!("11")),
                        CreateButton::new(format!("12")).label(format!("12")),
                    ]),
                    CreateActionRow::Buttons(vec![
                        CreateButton::new(format!("20")).label(format!("20")),
                        CreateButton::new(format!("21")).label(format!("21")),
                        CreateButton::new(format!("22")).label(format!("22")),
                    ]),
                ]),
            ),
        )
        .await
        .unwrap();

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

    let game = game::TicTacToeGame::new(message.author.id, player2.id, message.author.id);

    {
        data.write().await.games.push(game);
    }

    Ok(())
}
