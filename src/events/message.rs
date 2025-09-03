use crate::{
    types::{
        translation::Translation,
        types::{Data, Error}
    },
    utils::{
        log::log,
        translations::{get_translation, save_translation}
    }
};
use anyhow::Result;
use poise::serenity_prelude as serenity;
use regex::Regex;

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