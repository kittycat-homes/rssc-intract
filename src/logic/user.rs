#[derive(Debug, PartialEq, Eq)]
pub struct Userid {
    url: Option<String>,
    name: String,
}

fn massage_url(input: &str) -> String {
    input.replacen("https://", "", 1).replacen("http://", "", 1)
}

impl Userid {
    pub fn parse(from: &str) -> Self {
        let values = from.split_once('@');

        match values {
            Some(s) => Userid {
                name: s.0.to_string(),
                url: Some(massage_url(s.1)),
            },
            None => Userid {
                url: None,
                name: from.to_string(),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(
            Userid::parse("zork@example.com"),
            Userid {
                url: Some("example.com".to_string()),
                name: "zork".to_string()
            }
        );
        assert_eq!(
            Userid::parse("zork@https://example.com"),
            Userid {
                url: Some("example.com".to_string()),
                name: "zork".to_string()
            }
        );
        assert_eq!(
            Userid::parse("zork"),
            Userid {
                url: None,
                name: "zork".to_string()
            }
        )
    }
}
