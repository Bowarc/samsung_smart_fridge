use serenity::{
    all::{Context, EventHandler, Message, Ready},
    prelude::TypeMapKey,
};

pub struct TicTacToe;

#[derive(Default)]
pub struct TicTacToeData {
}

impl TypeMapKey for TicTacToeData {
    type Value = std::sync::Arc<std::sync::RwLock<TicTacToeData>>;
}

#[serenity::async_trait]
impl EventHandler for TicTacToe {
    async fn ready(&self, ctx: Context, _data_about_bot: Ready) {
        ctx.data
            .write()
            .await
            .insert::<TicTacToeData>(std::sync::Arc::new(std::sync::RwLock::new(
                TicTacToeData::default(),
            )));
    }
    async fn message(&self, ctx: Context, _message: Message) {
        let Some(_data) = ctx.data.read_owned().await.get::<TicTacToeData>() else {
            error!("Could not get TicTacToe data from storage");
            return;
        };
    }
}
