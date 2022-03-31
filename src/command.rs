use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::CommandResult;
use serenity::model::channel::Message;
use serenity::prelude::Context;

use crate::{status_checker, CONFIG};

#[group]
#[commands(ping, check)]
pub struct General;

#[command]
async fn ping(context: &Context, message: &Message) -> CommandResult {
    message.channel_id.say(&context.http, "Pong.").await?;

    Ok(())
}

#[command]
async fn check(context: &Context, message: &Message) -> CommandResult {
    let msg = format!(
        "Checking '{}'...",
        &*CONFIG
            .target_url
            .replace("http://", "")
            .replace("https://", "")
    );
    message.channel_id.say(&context.http, msg).await?;

    status_checker::send_status(&context.http, &message.channel_id).await;

    Ok(())
}
