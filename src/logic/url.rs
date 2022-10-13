use std::error::Error;
use url::Url;

/**
 * this should fail when trying to parse an invalid url
 */
pub fn massage_url(input: &str) -> Result<String, Box<dyn Error>> {
    let url = Url::parse(input)?;
    Ok(url.to_string())
}
