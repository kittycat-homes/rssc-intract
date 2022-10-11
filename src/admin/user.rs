use crate::admin::structs::*;

pub fn usermenu() -> MenuPage {
    MenuPage {
        items: vec![MenuItem {
            name: "quit",
            function: || Ok(()),
        }],
    }
}
