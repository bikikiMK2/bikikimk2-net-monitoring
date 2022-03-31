use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::CommandResult;
use serenity::model::channel::Message;
use serenity::prelude::Context;

#[group]
#[commands(ping)]
pub struct General;

#[command]
async fn ping(context: &Context, message: &Message) -> CommandResult {
    message.channel_id.say(&context.http, "Pong.").await?;

    Ok(())
}
