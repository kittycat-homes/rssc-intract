#![allow(clippy::unnecessary_lazy_evaluations)]
use std::error::Error;

#[derive(Debug, PartialEq, Eq, FromForm)]
pub struct Userid {
    pub url: Option<String>,
    pub name: String,
}

#[derive(thiserror::Error, Debug)]
enum UserError {
    #[error("this userid contains too many @s")]
    TooManyAts,
}

impl Userid {
    /**
     * try seperating a userid into a username and url
     * if there is no url then we use just the username
     */
    pub fn parse(mut from: String) -> Result<Userid, Box<dyn Error>> {
        fn massage_url(input: &str) -> String {
            input.replacen("https://", "", 1).replacen("http://", "", 1)
        }

        if from.starts_with('@') {
            from = from.replacen('@', "", 1)
        }

        // if a string has too many ats in it we should fail
        // i am very tired right now but this is the best way i could think of
        // to find out how many occurences of a char are in a string
        if from.chars().filter(|x| x == &'@').collect::<String>().len() > 1 {
            return Err(UserError::TooManyAts)?;
        }

        let values = from.split_once('@');

        Ok(match values {
            Some(s) => {
                return Ok(Userid {
                    name: s.0.to_string(),
                    url: Some(massage_url(s.1)),
                });
            }
            None => Userid {
                url: None,
                name: from.to_string(),
            },
        })
    }

    /**
     * should probably be called Serialize but unparse is funnier
     */
    pub fn unparse(&self) -> String {
        match &self.url {
            None => self.name.clone(),
            Some(url) => format!("{}@{}", self.name, url),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(
            Userid::parse("zork@example.com".to_string()).unwrap(),
            Userid {
                url: Some("example.com".to_string()),
                name: "zork".to_string()
            }
        );
        assert_eq!(
            Userid::parse("zork@https://example.com".to_string()).unwrap(),
            Userid {
                url: Some("example.com".to_string()),
                name: "zork".to_string()
            }
        );
        assert_eq!(
            Userid::parse("zork@http://example.com".to_string()).unwrap(),
            Userid {
                url: Some("example.com".to_string()),
                name: "zork".to_string()
            }
        );
        assert_eq!(
            Userid::parse("zork".to_string()).unwrap(),
            Userid {
                url: None,
                name: "zork".to_string()
            }
        );
    }
}
