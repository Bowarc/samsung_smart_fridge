use serenity::all::{Context, EventHandler, Message};

pub struct Ping;

#[serenity::async_trait]
impl EventHandler for Ping {
    async fn message(&self, ctx: Context, message: Message) {
        if !message.content.to_lowercase().starts_with("ping") {
            return;
        }
        
        if let Err(why) = message.reply(&ctx.http, "Pong !").await {
            println!("Failed to respond to ping message due to: {why}");
        }
    }
}
