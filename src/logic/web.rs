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

/**
 * this stores information about a website such as
 * title and description
 */
#[derive(Debug, PartialEq, Eq)]
pub struct WebsiteInfo {
    pub description: Option<String>,
    pub title: Option<String>,
}

impl WebsiteInfo {
    pub async fn get(url: &str) -> Result<Self, Box<dyn Error>> {
        Ok(WebsiteInfo::get_description_and_title(
            WebsiteInfo::parse_html(url).await?,
        ))
    }

    async fn parse_html(url: &str) -> Result<Html, Box<dyn Error>> {
        let parsed_url = Url::parse(url)?;

        if !(parsed_url.scheme() == "http" || parsed_url.scheme() == "https") {
            return Err(WebError::WrongScheme)?;
        }

        // we are not entirely sure if this IS html
        // but parsing should fail if its not so this is fine
        let html = HTML_CLIENT.get(parsed_url).send().await?.text().await?;
        Ok(Html::parse_document(&html))
    }

    fn get_description_and_title(parsed_content: Html) -> WebsiteInfo {
        // unwrapping here should be fine because what we parse should not change
        let titletag: Selector = Selector::parse(r#"title"#).unwrap();
        let titles = parsed_content.select(&titletag).collect::<Vec<_>>();

        let description_tag: Selector = Selector::parse(r#"meta[name="description"]"#).unwrap();
        let descriptions = parsed_content.select(&description_tag).collect::<Vec<_>>();

        WebsiteInfo {
            title: titles.first().map(|f| f.inner_html()),
            description: match descriptions.is_empty() {
                true => None,
                false => match descriptions[0].value().attr("content") {
                    None => None,
                    Some(e) => {
                        info!("found description {}", e);
                        Some(e.to_string())
                    }
                },
            },
        }
    }
}

#[derive(thiserror::Error, Debug)]
enum WebError {
    #[error("this is not the correct scheme")]
    WrongScheme,
}

fn default_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(header::ACCEPT, HeaderValue::from_static("text/html"));
    headers
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parsing_html() {
        assert_eq!(
            WebsiteInfo::get_description_and_title(Html::new_document()),
            WebsiteInfo {
                title: None,
                description: None
            }
        );
        assert_eq!(
            WebsiteInfo::get_description_and_title(Html::parse_document(
                r"
                <head>
                <title>hello</title>
                <meta name='description' content='world'>
                </head>
                "
            )),
            WebsiteInfo {
                title: Some("hello".to_string()),
                description: Some("world".to_string()),
            }
        )
    }
}
