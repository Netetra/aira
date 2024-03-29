use std::num::ParseIntError;

use once_cell::unsync::Lazy;
use poise::serenity_prelude::{
    self as serenity, ChannelId, CreateEmbed, CreateEmbedAuthor, CreateEmbedFooter, CreateMessage,
    GuildId, Message, MessageId, Timestamp,
};
use regex::Regex;

use crate::util::discord::{get_channel, get_guild, get_message};

#[derive(Debug, PartialEq, Eq)]
pub struct MessageLink {
    guild_id: GuildId,
    channel_id: ChannelId,
    message_id: MessageId,
}

pub async fn message_link_preview(
    ctx: &serenity::Context,
    new_message: &Message,
    message_links: Vec<MessageLink>,
) {
    for message_link in message_links {
        let MessageLink {
            guild_id,
            channel_id,
            message_id,
        } = message_link;
        let datas = async {
            let guild = get_guild(ctx, &guild_id).await?;
            let channel = get_channel(ctx, &guild, &channel_id).await?;
            let message = get_message(ctx, &channel, &message_id).await?;
            Some((guild, channel, message))
        }
        .await;
        if datas.is_none() {
            continue;
        }
        let (guild, channel, message) = datas.unwrap();
        let embed = create_embed(
            &message.author.name,
            &message.author.face(),
            &guild.name,
            &guild.icon_url(),
            channel.name(),
            &message.content,
            &message.timestamp,
        );
        let builder = CreateMessage::new()
            .reference_message(new_message)
            .embed(embed);
        let _ = new_message.channel_id.send_message(ctx, builder).await;
    }
}

impl MessageLink {
    pub fn find_all(text: &str) -> Option<Vec<MessageLink>> {
        let re = Lazy::new(|| {
            Regex::new(r"https://discord.com/channels/(?-u)(\d+)/(\d+)/(\d+)").unwrap()
        });
        let caps = re.captures_iter(text);
        // TODO: イテレータが空かの判定
        // if caps.is_empty() {
        //     return None;
        // }
        let mut message_links = Vec::<MessageLink>::new();
        for (_, [guild_id, channel_id, message_id]) in caps.map(|c| c.extract()) {
            if let Ok(p) = Self::parse(guild_id, channel_id, message_id) {
                message_links.push(p)
            }
        }
        if message_links.is_empty() {
            return None;
        }
        Some(message_links)
    }

    fn parse(
        raw_guild_id: &str,
        raw_channel_id: &str,
        raw_message_id: &str,
    ) -> Result<MessageLink, ParseIntError> {
        let guild_id = GuildId::new(raw_guild_id.parse::<u64>()?);
        let channel_id = ChannelId::new(raw_channel_id.parse::<u64>()?);
        let message_id = MessageId::new(raw_message_id.parse::<u64>()?);
        Ok(MessageLink {
            guild_id,
            channel_id,
            message_id,
        })
    }
}

fn create_embed(
    autor_name: &str,
    autor_icon: &str,
    guild_name: &str,
    guild_icon: &Option<String>,
    channel_name: &str,
    content: &str,
    timestamp: &Timestamp,
) -> CreateEmbed {
    let autor = CreateEmbedAuthor::new(autor_name).icon_url(autor_icon);
    let mut footer = CreateEmbedFooter::new(format!("{} in {}", channel_name, guild_name));
    if let Some(url) = guild_icon {
        footer = footer.icon_url(url)
    }
    CreateEmbed::new()
        .description(content)
        .author(autor)
        .footer(footer)
        .timestamp(timestamp)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    #[rstest]
    // 一つだけ含まれてるパターン
    #[case(
        "https://discord.com/channels/123456789012345678/234567890123456789/3456789012345678901",
        Some(vec![
            MessageLink {
                guild_id: GuildId::new(123456789012345678),
                channel_id: ChannelId::new(234567890123456789),
                message_id: MessageId::new(3456789012345678901)
            }
        ])
    )]
    // 複数含まれてるパターン
    #[case(
        "https://discord.com/channels/123456789012345678/234567890123456789/3456789012345678901https://discord.com/channels/123456789012345678/234567890123456789/3456789012345678901",
        Some(vec![
            MessageLink {
                guild_id: GuildId::new(123456789012345678),
                channel_id: ChannelId::new(234567890123456789),
                message_id: MessageId::new(3456789012345678901)
            },
            MessageLink {
                guild_id: GuildId::new(123456789012345678),
                channel_id: ChannelId::new(234567890123456789),
                message_id: MessageId::new(3456789012345678901)
            }
        ])
    )]
    // 部分文字列として含まれてるパターン
    #[case(
        "HugaHugahttps://discord.com/channels/123456789012345678/234567890123456789/3456789012345678901HogeHoge",
        Some(vec![
            MessageLink {
                guild_id: GuildId::new(123456789012345678),
                channel_id: ChannelId::new(234567890123456789),
                message_id: MessageId::new(3456789012345678901)
            }
        ])
    )]
    // 含まれてないパターン
    #[case("Hello World!", None)]
    // 大文字が含まれてるパターン
    #[case(
        "Https://discord.com/channels/123456789012345678/234567890123456789/3456789012345678901",
        None
    )]
    #[case(
        "https://disCord.com/channels/123456789012345678/234567890123456789/3456789012345678901",
        None
    )]
    // 全角文字が含まれてるパターン
    #[case(
        "ｈttps://discord.com/channels/123456789012345678/234567890123456789/3456789012345678901",
        None
    )]
    #[case(
        "https://disｃord.com/channels/123456789012345678/234567890123456789/3456789012345678901",
        None
    )]
    #[case(
        "https://discord.com/channels/12345678９012345678/234567890123456789/3456789012345678901",
        None
    )]
    // IDが足りないパターン
    #[case(
        "https://discord.com/channels/123456789012345678/234567890123456789/",
        None
    )]
    #[case(
        "https://discord.com/channels/123456789012345678/234567890123456789",
        None
    )]
    // IDが64bit以上のパターン
    #[case(
        "https://discord.com/channels/18446744073709551616/234567890123456789/3456789012345678901",
        None
    )]
    fn find_all_test(#[case] input: &str, #[case] output: Option<Vec<MessageLink>>) {
        let result = MessageLink::find_all(input);
        assert_eq!(result, output);
    }
}
