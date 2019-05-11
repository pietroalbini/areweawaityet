mod discourse;
mod reddit;

use chrono::{DateTime, FixedOffset, Utc};
use failure::Error;
use reqwest::header::{HeaderValue, USER_AGENT};

static SOURCES: &[&dyn Source] = &[&discourse::Discourse, &reddit::Reddit];
static TEMPLATE: &str = include_str!("template.html");

struct Context {
    client: reqwest::Client,
}

impl Context {
    fn new() -> Self {
        Context {
            client: reqwest::Client::new(),
        }
    }

    fn get(&self, url: &str) -> Result<reqwest::Response, Error> {
        Ok(self
            .client
            .get(url)
            .header(USER_AGENT, HeaderValue::from_static("areweawaityet"))
            .send()?
            .error_for_status()?)
    }
}

trait Source: Send + Sync {
    fn threads(&self, ctx: &Context) -> Result<Vec<Item>, Error>;
    fn comments(&self, ctx: &Context) -> Result<Vec<Item>, Error>;
}

#[derive(serde::Serialize, Debug)]
struct Item {
    title: String,
    url: String,
    created_at: DateTime<FixedOffset>,
}

fn is_await(content: &str) -> bool {
    content.contains("await")
}

fn render(last_thread: Option<&Item>, last_comment: Option<&Item>) -> Result<String, Error> {
    #[derive(serde::Serialize)]
    struct TemplateData<'a> {
        last_thread: Option<&'a Item>,
        last_comment: Option<&'a Item>,
        updated_at: DateTime<Utc>,
    }

    let mut tera = tera::Tera::default();
    tera.add_raw_template("template.html", TEMPLATE)
        .map_err(|e| failure::err_msg(e.to_string()))?;
    Ok(tera
        .render(
            "template.html",
            &TemplateData {
                last_thread,
                last_comment,
                updated_at: Utc::now(),
            },
        )
        .map_err(|e| failure::err_msg(e.to_string()))?)
}

fn app() -> Result<(), Error> {
    let ctx = Context::new();
    let mut threads = Vec::new();
    let mut comments = Vec::new();
    for source in SOURCES {
        threads.append(&mut source.threads(&ctx)?);
        comments.append(&mut source.comments(&ctx)?);
    }
    threads.sort_by_key(|item| item.created_at);
    comments.sort_by_key(|item| item.created_at);
    std::fs::write(
        "static/index.html",
        render(threads.last(), comments.last())?.as_bytes(),
    )?;
    Ok(())
}

fn main() {
    if let Err(err) = app() {
        eprintln!("error: {}", err);
        std::process::exit(1);
    }
}
