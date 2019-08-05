static DISCOURSE_INSTANCE: &str = "https://internals.rust-lang.org";

use crate::{is_await, Context, Item, Source};
use chrono::DateTime;
use failure::Error;
use std::io::BufReader;

pub(crate) struct Discourse;

impl Discourse {
    fn parse_feed(&self, ctx: &Context, feed: &str) -> Result<Vec<Item>, Error> {
        let response = ctx.get(&format!("{}/{}", DISCOURSE_INSTANCE, feed))?;
        Ok(rss::Channel::read_from(&mut BufReader::new(response))?
            .items()
            .iter()
            .filter(|item| {
                [item.title(), item.description(), item.content()]
                    .iter()
                    .filter_map(|i| *i)
                    .any(|t| is_await(t))
            })
            .filter_map(|item| {
                Some(Item {
                    title: item.title()?.to_string(),
                    url: item.link()?.to_string(),
                    created_at: DateTime::parse_from_rfc2822(item.pub_date()?).ok()?,
                })
            })
            .collect())
    }
}

impl Source for Discourse {
    fn threads(&self, ctx: &Context) -> Result<Vec<Item>, Error> {
        self.parse_feed(ctx, "latest.rss")
    }

    fn comments(&self, ctx: &Context) -> Result<Vec<Item>, Error> {
        self.parse_feed(ctx, "posts.rss")
    }
}
