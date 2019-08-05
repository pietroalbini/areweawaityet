static SUBREDDIT: &str = "https://www.reddit.com/r/rust";

use crate::{is_await, Context, Item, Source};
use chrono::DateTime;
use failure::Error;
use std::io::BufReader;

pub(crate) struct Reddit;

impl Reddit {
    fn parse_feed(&self, ctx: &Context, feed: &str) -> Result<Vec<Item>, Error> {
        let response = ctx.get(&format!("{}{}", SUBREDDIT, feed))?;
        Ok(
            atom_syndication::Feed::read_from(&mut BufReader::new(response))?
                .entries()
                .iter()
                .filter(|entry| {
                    [
                        Some(entry.title()),
                        entry.summary(),
                        entry.content().and_then(|c| c.value()),
                    ]
                    .iter()
                    .filter_map(|i| *i)
                    .any(|t| is_await(t))
                })
                .filter_map(|entry| {
                    Some(Item {
                        title: entry.title().to_string(),
                        url: entry.links().iter().next()?.href().to_string(),
                        created_at: DateTime::parse_from_rfc3339(entry.updated()).ok()?,
                    })
                })
                .collect(),
        )
    }
}

impl Source for Reddit {
    fn threads(&self, ctx: &Context) -> Result<Vec<Item>, Error> {
        self.parse_feed(ctx, ".rss")
    }

    fn comments(&self, ctx: &Context) -> Result<Vec<Item>, Error> {
        self.parse_feed(ctx, "/comments.rss")
    }
}
