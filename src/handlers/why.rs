use {
    crate::command,
    serenity::all::{Context, EventHandler, Message},
};

pub struct Why;

#[serenity::async_trait]
impl EventHandler for Why {
    async fn message(&self, ctx: Context, message: Message) {
        let Some(_args) = command::parse(
            &message,
            "why",
            command::Case::Insensitive,
            command::Prefix::No,
        ) else {
            return;
        };

        if let Err(why) = message.reply(&ctx.http, "Because fuck you").await {
            println!("Failed to respond to ping message due to: {why}");
        }
    }
}
