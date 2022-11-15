use serde::{Deserialize, Serialize};
mod english;

#[derive(Serialize, Deserialize, Clone, Copy)]
pub enum Language {
    English,
}

pub fn translation(language: Language) -> Translation {
    match language {
        Language::English => english::TRANSLATION,
    }
}

pub trait Translatable {
    fn language(&self) -> Language;

    fn translation(&self) -> Translation {
        translation(self.language())
    }
}

pub fn determine_language() -> Language {
    Language::English
}

pub struct Translation {
    /// code represnting the language, for putting in the html header
    /// english would be en and german would be de
    pub code: &'static str,

    pub username: &'static str,
    pub password: &'static str,
    pub login: &'static str,

    // headings
    //// settings page
    pub settings_page_heading: &'static str,
    pub settings_page_password_heading: &'static str,
    //// login page
    pub login_page_heading: &'static str,
}
