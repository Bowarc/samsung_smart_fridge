use serenity::all::{Channel, Context, EventHandler, GetMessages, Message, MessageId};

pub struct Purge;

#[serenity::async_trait]
impl EventHandler for Purge {
    async fn message(&self, ctx: Context, message: Message) {
        if !message.content.to_lowercase().starts_with("purge") {
            return;
        }

        let count = {
            let split = message.content.split(" ").collect::<Vec<&str>>();

            if split.len() < 2 {
                if let Err(why) = message
                    .channel_id
                    .say(
                        &ctx.http,
                        "Expected 1 argument, please specify a number of message to purge",
                    )
                    .await
                {
                    error!("Could not send error message due to: {why}");
                }
                return;
            }

            let Ok(nbr) = split.get(1).unwrap().parse::<u8>() else {
                if let Err(why) = message
                    .channel_id
                    .say(
                        &ctx.http,
                        "Could not parse count argument, make sure it's a positive integer",
                    )
                    .await
                {
                    error!("Could not send error message due to: {why}");
                }
                return;
            };

            nbr
        };

        let Ok(channel) = message.channel(&ctx.http).await else {
            if let Err(why) = message.channel_id.say(&ctx.http, "Error").await {
                error!("Could not send error message due to: {why}");
            }
            return;
        };

        let Channel::Guild(guild_channel) = channel else {
            if let Err(why) = message
                .channel_id
                .say(&ctx.http, "Private channels are not supported yet")
                .await
            {
                error!("Could not send error message due to: {why}");
            }
            return;
        };

        let Ok(messages) = guild_channel
            .messages(
                &ctx.http,
                GetMessages::new().before(message.id).limit(count),
            )
            .await
        else {
            if let Err(why) = message
                .channel_id
                .say(
                    &ctx.http,
                    "Failed to fetch the recent messages for this channel",
                )
                .await
            {
                error!("Could not send error message due to: {why}");
            }
            return;
        };

        // Delete the purge request message
        if let Err(why) = message.delete(&ctx.http).await {
            error!("Failed to delete purge request message due to: {why}");
            return;
        }

        if let Err(why) = message
            .channel_id
            .delete_messages(
                &ctx.http,
                messages
                    .iter()
                    .map(|msg| msg.id)
                    .collect::<Vec<MessageId>>(),
            )
            .await
        {
            error!("Failed to delete messages due to: {why}");
            return;
        }

        trace!("Purged {count} messages");
    }
}
