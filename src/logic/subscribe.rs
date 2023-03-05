#[derive(thiserror::Error, Debug)]
pub enum SubError {
    #[error("could not parse a feed at this location")]
    NotAFeed,
}

pub fn subscribe_to_feed() -> Result<(), SubError> {
    Ok(())
}

fn get_from_url() {}

fn try_parse(url: &str) {}
