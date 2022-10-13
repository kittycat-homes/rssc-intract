#![allow(clippy::unnecessary_lazy_evaluations)]

use crate::database::{self as db, post::Post, share::Share, tag::NewTag};
use base64;
use rocket::FromForm;
use std::{error::Error, time::SystemTime};

#[derive(Debug, FromForm)]
pub struct ShareForm<'r> {
    url: &'r str,
    description: &'r str,
    tags: &'r str,
}

impl ShareForm<'_> {
    pub async fn save(&self, username: &str) -> Result<(), Box<dyn Error>> {
        let post = self.create_new_or_get_old_post().await?;
        let _share = db::share::create(self.into_share(username, &post.id))?;

        for t in parse_into_tag(self.tags, username, &post.id) {
            let tag = db::tag::create(t)?;
            info!("saved tag: {} with id: {}", tag.value, tag.id);
        }

        Ok(())
    }

    pub fn into_share(&self, username: &str, postid: &str) -> Share {
        Share {
            post_id: postid.to_string(),
            username: username.to_string(),
            user_comment: None,
            time: SystemTime::now(),
        }
    }

    /**
     * first try looking up if a post with this id already exists
     * if not then try creating one
     */
    async fn create_new_or_get_old_post(&self) -> Result<Post, Box<dyn Error>> {
        let url = crate::logic::url::massage_url(self.url)?;
        let id = encode_url(&url);

        let old = db::post::get(id);

        // this seems bad to me but should be fine probably
        if old.is_ok() {
            return Ok(old.unwrap());
        }

        let new = Post {
            id: encode_url(&url),
            url,
            // TODO fetch title from url
            title: None,
            description: Some(self.description.to_string()),
            // TODO create a feed if this id doesn't exist yet
            feed_id: None,
            time: SystemTime::now(),
        };

        let post = db::post::create(new)?;

        Ok(post)
    }
}

/// takes a string just like you would get from an html form and parses it to tag values
fn parse_into_tag(input: &str, username: &str, post_id: &str) -> Vec<NewTag> {
    input
        .split(',')
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

/// converts a utf8 string to a postid that can be saved in the databse
fn encode_url(url: &str) -> String {
    base64::encode_config(url, base64::URL_SAFE)
}

/// converts a postid back to a string
#[allow(dead_code)]
fn decode_url(url: &str) -> Result<String, Box<dyn Error>> {
    let bytes = base64::decode_config(url, base64::URL_SAFE)?;
    let string = std::str::from_utf8(&bytes)?;
    Ok(string.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_postid_gen() {
        assert_eq!(
            "https://example.com".to_string(),
            decode_url(&encode_url("https://example.com")).unwrap()
        )
    }

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
