#![feature(yeet_expr)]
#![feature(duration_constructors)]

#[macro_use]
extern crate log;

mod error;
mod handlers;
mod command;

const INVISIBLE_CHARACTER: &str = "\u{200e}";
// const INVISIBLE_CHARACTER: &str = "\u{18b5}";

#[tokio::main]
async fn main() {
    use serenity::prelude::*;
    use std::env;
    dotenv::dotenv().ok();

    let filters = vec![
        ("serenity", log::LevelFilter::Warn),
        ("h2", log::LevelFilter::Error),
        ("tokio", log::LevelFilter::Warn),
        ("hyper", log::LevelFilter::Warn),
        ("tungstenite", log::LevelFilter::Warn),
        ("reqwest", log::LevelFilter::Warn),
        ("rustls", log::LevelFilter::Warn),
    ];

    logger::init([
        logger::Config::default()
            .level(log::LevelFilter::Trace)
            .output(logger::Output::Stdout)
            .colored(true)
            .filters(&filters),
        logger::Config::default()
            .level(log::LevelFilter::Info)
            .output(logger::Output::new_timed_file(
                "./log/.log",
                std::time::Duration::from_hours(1),
            ))
            .colored(false)
            .filters(&filters),
    ]);

    // Login with a bot token from the environment
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

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
