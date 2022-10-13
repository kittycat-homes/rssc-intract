use base64;
use std::{error::Error, time::SystemTime};

use rocket::FromForm;

use crate::database::{
    share::Share,
    tag::{NewTag, NewTagBuilder},
};

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

    pub fn save(&self, username: &str) -> Result<(), Box<dyn Error>> {
        let savable = self.into_share(username);
        Ok(())
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
fn parse_into_tag(input: &str, username: &str, post_id: &str) -> Vec<NewTag> {
    input
        .split(",")
        .filter_map(|v| {
            let value = v.trim();
            if value.is_empty() {
                return None;
            }
            Some(NewTag {
                value: value.to_string(),
                username: username.to_string(),
                post_id: post_id.to_string(),
            })
        })
        .collect::<Vec<NewTag>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tagnames() {
        let username = "zorkthegreat";
        let postid = "someid";

        let a = NewTag {
            value: "a".to_string(),
            username: username.to_string(),
            post_id: postid.to_string(),
        };

        let b = NewTag {
            value: "b".to_string(),
            username: username.to_string(),
            post_id: postid.to_string(),
        };

        let c = NewTag {
            value: "c".to_string(),
            username: username.to_string(),
            post_id: postid.to_string(),
        };

        let abc = vec![a, b, c];

        assert_eq!(abc, parse_into_tag("a, b, c", username, postid));
        assert_eq!(
            abc,
            parse_into_tag("   a   , b   ,   c   ", username, postid)
        );
        assert_eq!(abc, parse_into_tag("a,, b, c,", username, postid));
        assert_eq!(abc, parse_into_tag(",, a , b ,, ,, , c,", username, postid));
        assert!(parse_into_tag(",,,   ,,,", username, postid).is_empty());
    }
}
