use {
    crate::command,
    serenity::all::{Context, EventHandler, Message},
};

pub struct Ping;

#[serenity::async_trait]
impl EventHandler for Ping {
    async fn message(&self, ctx: Context, message: Message) {
        let Some(args) = command::parse(&message, "ping", command::Case::Insensitive, command::Prefix::Yes) else {
            return;
        };

        if !args.is_empty() {
            error!(
                "Thelp command received, but got unexpected arguments: {:?}",
                &args
            );
            return;
        }

        if let Err(why) = message.reply(&ctx.http, "Pong !").await {
            println!("Failed to respond to ping message due to: {why}");
        }
    }
}
