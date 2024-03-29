use poise::serenity_prelude::{self as serenity, ActivityData, Ready};

pub fn ready(ctx: &serenity::Context, _: &Ready) {
    let crate_version = env!("CARGO_PKG_VERSION");
    let activity_text = format!("Version: {}", crate_version);
    ctx.set_activity(Some(ActivityData::custom(activity_text)));
}
