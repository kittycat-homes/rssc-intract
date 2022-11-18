use accept_language;
use rocket::{
    outcome::Outcome,
    request::{self, FromRequest},
    Request,
};
use std::convert::Infallible;

mod english;
mod german;

#[derive(Clone, Copy)]
pub struct Translation {
    /// code represnting the language, for putting in the html header
    /// english would be en and german would be de
    pub code: &'static str,

    pub username: &'static str,
    pub password: &'static str,
    pub login: &'static str,
    pub display_name: &'static str,
    pub new_password: &'static str,
    pub save: &'static str,
    pub language: &'static str,

    // headings
    //// settings page
    pub settings_page_heading: &'static str,
    pub settings_page_profile_heading: &'static str,
    pub settings_page_client_heading: &'static str,
    pub settings_page_password_heading: &'static str,
    //// login page
    pub login_page_heading: &'static str,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Translation {
    type Error = Infallible;

    /**
     * determine the language and return the fitting translation
     */
    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        /// used for parsing the accept language header
        fn parse_language(s: &str) -> Translation {
            let user_languages = accept_language::parse(s);

            for lang in user_languages {
                match lang.as_str() {
                    "de" => return german::TRANSLATION,
                    "en" => return english::TRANSLATION,
                    _ => (),
                }
            }

            english::TRANSLATION
        }

        // see if a language cookie exists
        // if yes use this as language
        match req.cookies().get("language") {
            None => (),
            Some(cookie) => return Outcome::Success(parse_language(cookie.value())),
        }

        // if no cookie exists, then use the browser header
        let t: Translation = match req.headers().get_one("accept-language") {
            Some(s) => parse_language(s),
            None => english::TRANSLATION,
        };

        Outcome::Success(t)
    }
}
