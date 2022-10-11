pub mod structs;
mod user;
use structs::*;

pub fn main_menu() -> MenuPage {
    MenuPage {
        items: vec![
            MenuItem {
                name: "users",
                function: || user::usermenu().open(),
            },
            MenuItem {
                name: "quit",
                function: || Ok(()),
            },
        ],
    }
}
