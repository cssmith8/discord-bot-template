use crate::{
    types::types::{Data, Error},
    utils::env
};
use anyhow::Result;
use poise::serenity_prelude as serenity;
use serenity::model::id::ChannelId;


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