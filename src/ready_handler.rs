use std::process::exit;

use serenity::async_trait;
use serenity::model::gateway::Ready;
use serenity::model::id::ChannelId;
use serenity::prelude::{Context, EventHandler};

use crate::CONFIG;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, context: Context, ready: Ready) {
        match ChannelId::try_from(CONFIG.monitoring_channel) {
            Ok(channel) => {
                let start_message =
                    format!("Monitoring started at <#{}>", CONFIG.monitoring_channel);
                let say_result = channel.say(context.http, &start_message).await;
                match say_result {
                    Ok(_) => (),
                    Err(err) => {
                        println!("Failed to send a start message: {:?}", err);
                        exit(1)
                    }
                }
            }
            Err(err) => {
                println!(
                    "Failed to get channel '{}': {:?}",
                    CONFIG.monitoring_channel, err
                );
                exit(1)
            }
        }

        println!("Logged in as '{}'.", ready.user.name);
    }
}
