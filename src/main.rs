#[macro_use]
extern crate log;

mod handlers;

#[tokio::main]
async fn main() {
    use serenity::prelude::*;
    use std::env;
    dotenv::dotenv().ok();

    let logger_config = logger::LoggerConfig::new()
        .set_level(log::LevelFilter::Off) // Serenity is weird
        .add_filter("serenity", log::LevelFilter::Error)
        .add_filter("samsung_smart_fridge", log::LevelFilter::Trace);

    logger::init(logger_config, None);

    // Login with a bot token from the environment
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    // Create a new instance of the Client, logging in as a bot.
    let mut client = Client::builder(&token, intents)
        .event_handler(handlers::Basic)
        .event_handler(handlers::Ping)
        .event_handler(handlers::TicTacToe)
        .event_handler(handlers::Logger)
        .event_handler(handlers::Why)
        .event_handler(handlers::Purge)
        .await
        .expect("Err creating client");

    // Start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}
