use std::process::exit;

use once_cell::sync::Lazy;
use serenity::framework::StandardFramework;
use serenity::prelude::Client;

use crate::config::Config;

mod config;
mod ready_handler;

pub const CONFIG: Lazy<Config> = Lazy::new(|| match config::load_config() {
    Ok(conf) => conf,
    Err(err) => {
        println!("Failed to load config.json: {:?}", err);
        exit(1)
    }
});

#[tokio::main]
async fn main() {
    match config::copy_config_if_not_exists() {
        Ok(()) => (),
        Err(err) => {
            println!("Failed to copy config.json: {:?}", err);
            exit(1)
        }
    }

    let framework = StandardFramework::new().configure(|command| command.prefix(&*CONFIG.prefix));

    let mut client = Client::builder(&*CONFIG.token)
        .event_handler(ready_handler::Handler)
        .framework(framework)
        .await
        .expect("Failed to create a client.");

    if let Err(why) = client.start().await {
        println!("Failed to start a bot: {:?}", why);
        exit(2)
    }
}
