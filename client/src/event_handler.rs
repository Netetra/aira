use crate::{events::ready::ready, Data, Error};
use poise::serenity_prelude::{self as serenity, FullEvent};

pub async fn event_handler(
    ctx: &serenity::Context,
    event: &FullEvent,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    _data: &Data,
) -> Result<(), Error> {
    if let FullEvent::Ready { data_about_bot } = event {
        ready(ctx, data_about_bot);
    }
    Ok(())
}
