use reqwest::{
    header::{self, HeaderMap, HeaderValue},
    Client,
};
use scraper::{Html, Selector};
use std::{error::Error, time::Duration};
use url::Url;

lazy_static! {
    static ref HTML_CLIENT: Client = Client::builder()
        .user_agent(concat!(
            env!("CARGO_PKG_NAME"),
            "(github.com/kittycat-homes/rssc-intract)",
            "/",
            env!("CARGO_PKG_VERSION"),
        ))
        .default_headers(default_headers())
        .timeout(Duration::new(10, 0))
        .build()
        .unwrap();
}

#[derive(thiserror::Error, Debug)]
enum WebError {
    #[error("this is not the correct scheme")]
    WrongScheme,
    #[error("no title found")]
    NoTitle,
}

fn default_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(header::ACCEPT, HeaderValue::from_static("text/html"));
    headers
}

pub async fn get_website_title(url: &str) -> Result<String, Box<dyn Error>> {
    let parsed_url = Url::parse(url)?;

    if !(parsed_url.scheme() == "http" || parsed_url.scheme() == "https") {
        return Err(WebError::WrongScheme)?;
    }

    // we are not entirely sure if this IS html
    // but parsing should fail if its not so this is fine
    let html = HTML_CLIENT.get(parsed_url).send().await?.text().await?;
    let parsed_content = Html::parse_document(&html);
    // unwrapping here should be fine because what we parse should not change
    let titletag: Selector = Selector::parse(r#"title"#).unwrap();
    let titles = parsed_content.select(&titletag).collect::<Vec<_>>();

    if titles.is_empty() {
        return Err(WebError::NoTitle)?;
    }

    let title = titles[0].inner_html();

    info!("found title for post: {}", &title);
    Ok(title)
}
