use crate::types::types::{Data, Error};
use anyhow::Result;
use poise::serenity_prelude as serenity;

pub async fn message(
    _ctx: &serenity::Context,
    _event: &serenity::FullEvent,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    _data: &Data,
    new_message: &serenity::Message,
) -> Result<(), Error> {
    if new_message.author.bot {
        return Ok(());
    }

    Ok(())
}