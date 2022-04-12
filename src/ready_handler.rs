use std::process::exit;

use serenity::async_trait;
use serenity::model::gateway::Ready;
use serenity::model::id::ChannelId;
use serenity::prelude::{Context, EventHandler};
use tokio_cron_scheduler::{Job, JobScheduler};

use crate::status_checker;
use crate::CONFIG;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, context: Context, ready: Ready) {
        match ChannelId::try_from(CONFIG.monitoring_channel) {
            Ok(channel) => {
                let start_message =
                    format!("Monitoring started at <#{}>", CONFIG.monitoring_channel);
                let say_result = channel.say(&context.http, &start_message).await;

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

        let http = context.http.clone();

        let mut scheduler = JobScheduler::new();
        scheduler
            .add(
                Job::new_async("0 0 * * * * *", move |_, _| {
                    let http = http.clone();

                    Box::pin(async move {
                        let channel = ChannelId::from(CONFIG.monitoring_channel);
                        status_checker::send_time(&http, &channel).await;
                        status_checker::send_status(&http, &channel).await;
                    })
                })
                .unwrap(),
            )
            .ok();

        #[cfg(feature = "signal")]
        scheduler.shutdown_on_ctrl_c();

        scheduler.start().await.ok();
    }
}
