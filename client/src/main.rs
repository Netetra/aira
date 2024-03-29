mod commands;
use commands::ping;
use poise::serenity_prelude::{self as serenity, GatewayIntents};
use std::env;
use tokio::signal::unix;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

struct Data;

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_BOT_TOKEN").expect("bot token not set.");
    let intents = GatewayIntents::non_privileged();
    let framework = poise::Framework::builder()
        .setup(move |ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .options(poise::FrameworkOptions {
            commands: vec![ping::ping()],
            ..Default::default()
        })
        .build();

    let mut client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await
        .unwrap();
    let mut sigterm = unix::signal(unix::SignalKind::terminate()).unwrap();

    tokio::select!(
        _ = client.start() => {},
        _ = sigterm.recv() => {}
    );
}
