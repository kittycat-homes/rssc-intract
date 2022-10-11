pub mod structs;
mod user;
use structs::*;

pub fn main_menu() -> MenuPage {
    MenuPage {
        items: vec![
            MenuItem {
                name: "users".to_string(),
                function: || user::usermenu().open(),
            },
            MenuItem {
                name: "quit".to_string(),
                function: || Ok(()),
            },
        ],
    }
}
