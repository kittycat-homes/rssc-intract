use crate::database as db;
use crate::logic::auth;
use dialoguer::{Input, Password};

use crate::admin::structs::*;

pub fn usermenu() -> MenuPage {
    MenuPage {
        items: vec![
            MenuItem {
                name: "add user".to_string(),
                function: add_user,
            },
            MenuItem {
                name: "delete users".to_string(),
                function: || deletemenu()?.open(),
            },
            MenuItem {
                name: "quit".to_string(),
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

/**
 * shows a menu page with all users who have accounts
 */
fn deletemenu() -> Result<MenuPage, Box<dyn std::error::Error>> {
    let users = db::user::get_all()?
        .iter()
        .map(|u| MenuItem {
            function: || Ok(()),
            name: u.username.clone(),
        })
        .collect::<Vec<MenuItem>>();
    Ok(MenuPage { items: vec![] })
}

/**
 * confirm if a user should be deleted
 */
fn confirm_delete(username: String) -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}
