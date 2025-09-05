use crate::types::types::{Data, Error};
use anyhow::Result;
use poise::serenity_prelude as serenity;


pub async fn awake(
    _ctx: &serenity::Context,
    _event: &serenity::FullEvent,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    _data: &Data,
    data_about_bot: &serenity::Ready,
) -> Result<(), Error> {
    println!("Logged in as {}", data_about_bot.user.tag());

    Ok(())
}