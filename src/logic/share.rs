use base64;
use std::{slice::Iter, time::SystemTime};

use rocket::FromForm;

use crate::database::{share::Share, tag::Tag};

#[derive(Debug, FromForm)]
pub struct ShareForm<'r> {
    url: &'r str,
    description: &'r str,
    tags: &'r str,
}

impl ShareForm<'_> {
    fn parse_tags(input: &str) -> Vec<String> {
        input
            .split(",")
            .map(|s| s.trim().to_string())
            .collect::<Vec<String>>()
    }

    pub fn into_share(&self, username: &str) -> Share {
        let tags = ShareForm::parse_tags(self.tags);
        trace!("{:?}", tags);

        Share {
            post_id: base64::encode_config(self.url, base64::URL_SAFE),
            username: username.to_string(),
            user_comment: Some(self.description.to_owned()),
            time: SystemTime::now(),
        }
    }
}

/// takes a string just like you would get from an html form and parses it to tag values
fn parse_tags_to_names(input: &str) -> Vec<String> {
    input
        .split(",")
        .filter_map(|s| {
            if s.is_empty() {
                return None;
            }
            Some(s.trim().to_string())
        })
        .collect::<Vec<String>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tagnames() {
        let abc: Vec<String> = ["a", "b", "c"]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        assert_eq!(parse_tags_to_names("a, b, c"), abc);
        assert_eq!(parse_tags_to_names(" a,     b, c "), abc);
        assert_eq!(parse_tags_to_names("a, b, c,"), abc);
        assert_eq!(parse_tags_to_names(",a, b, c,"), abc);
        assert!(parse_tags_to_names(",").is_empty());
    }
}
