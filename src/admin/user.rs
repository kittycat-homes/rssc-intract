use crate::logic::auth;
use dialoguer::{Input, Password};

use crate::admin::structs::*;

pub fn usermenu() -> MenuPage {
    MenuPage {
        items: vec![
            MenuItem {
                name: "add user",
                function: || add_user(),
            },
            MenuItem {
                name: "quit",
                function: || Ok(()),
            },
        ],
    }
}

/**
 * opens a prompt that lets you add an additional user
 */
pub fn add_user() -> Result<(), Box<dyn std::error::Error>> {
    let username = Input::<String>::new()
        .with_prompt("username")
        .interact_text()?;
    let password = Password::new()
        .with_prompt("password")
        .allow_empty_password(false)
        .interact()?;
    auth::add_user(username, password)
}
