use crate::{
    events::{
        message_link_preview::{message_link_preview, MessageLink},
        ready::ready,
    },
    Data, Error,
};
use poise::serenity_prelude::{self as serenity, FullEvent};

pub async fn event_handler(
    ctx: &serenity::Context,
    event: &FullEvent,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    _data: &Data,
) -> Result<(), Error> {
    match event {
        FullEvent::Ready { data_about_bot } => ready(ctx, data_about_bot),
        FullEvent::Message { new_message } => {
            if new_message.author.bot {
                return Ok(());
            }
            if let Some(message_links) = MessageLink::find_all(&new_message.content) {
                message_link_preview(ctx, new_message, message_links).await;
            }
        }
        _ => {}
    }
    Ok(())
}
