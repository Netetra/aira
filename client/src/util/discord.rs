use poise::serenity_prelude::{
    self as serenity, ChannelId, Guild, GuildChannel, GuildId, Message, MessageId,
};

pub async fn get_guild(ctx: &serenity::Context, guild_id: &GuildId) -> Option<Guild> {
    let guild = guild_id.to_guild_cached(ctx)?;
    Some(guild.to_owned())
}

pub async fn get_channel(
    ctx: &serenity::Context,
    guild: &Guild,
    channel_id: &ChannelId,
) -> Option<GuildChannel> {
    let channels = guild.channels(ctx).await.ok()?;
    let channel = channels.get(channel_id)?.clone();
    Some(channel)
}

pub async fn get_message(
    ctx: &serenity::Context,
    channel: &GuildChannel,
    message_id: &MessageId,
) -> Option<Message> {
    channel.message(ctx, message_id).await.ok()
}
