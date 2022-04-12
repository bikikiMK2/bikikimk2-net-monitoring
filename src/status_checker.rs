use std::sync::Arc;

use serenity::http::Http;
use serenity::model::id::ChannelId;

use crate::util;
use crate::CONFIG;

pub async fn send_status(http: &Arc<Http>, channel: &ChannelId) -> () {
    let client = reqwest::Client::new();

    match client.get(&*CONFIG.target_url).send().await {
        Ok(response) => {
            channel
                .say(http, ":white_check_mark: Connected.")
                .await
                .ok();

            let message = if response.status().is_success() {
                format!(":white_check_mark: {}.", response.status().to_string())
            } else {
                format!(":x: {}.", response.status().to_string())
            };
            channel.say(http, message).await.ok()
        }
        Err(err) => {
            let message = format!(":x: Failed to connect:\n{:?}", err);
            channel.say(http, message).await.ok()
        }
    };
}

pub async fn send_time(http: &Arc<Http>, channel: &ChannelId) -> () {
    let timestamp = util::current_timestamp();
    let time = util::parse_timestamp(timestamp);

    let url = &*CONFIG
        .target_url
        .replace("http://", "")
        .replace("https://", "");

    let message = format!("'{}' status at {}:", url, time);
    channel.say(http, message).await.ok();
}
