use serde::{Deserialize, Serialize};

mod english;

#[derive(Serialize, Deserialize)]
pub enum Language {
    English,
}

pub fn translation(language: Language) -> Translation {
    match language {
        Language::English => english::TRANSLATION,
    }
}

pub struct Translation {
    pub settings_page_heading: &'static str,
}
