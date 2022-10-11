use crate::database as db;
use crate::logic::auth;
use dialoguer::{Confirm, FuzzySelect, Input, Password};

use crate::admin::structs::*;

pub fn usermenu() -> MenuPage {
    MenuPage {
        items: vec![
            MenuItem {
                name: "list users".to_string(),
                function: || userpicker(Action::List),
            },
            MenuItem {
                name: "add user".to_string(),
                function: add_user,
            },
            MenuItem {
                name: "change password".to_string(),
                function: || userpicker(Action::ChangePw),
            },
            MenuItem {
                name: "delete user".to_string(),
                function: || userpicker(Action::Delete),
            },
            MenuItem {
                name: "back".to_string(),
                function: || crate::admin::main_menu().open(),
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

enum Action {
    Delete,
    List,
    ChangePw,
}

/**
 * shows a menu page with all users who have accounts
 */
fn userpicker(action: Action) -> Result<(), Box<dyn std::error::Error>> {
    let users = db::user::get_local()?;
    let items = users
        .iter()
        .map(|u| u.username.clone())
        .collect::<Vec<String>>();
    let index = FuzzySelect::new().items(&items).interact()?;
    match action {
        Action::Delete => {
            return confirm_delete(users[index].username.clone());
        }
        Action::List => {
            return Ok(());
        }
        Action::ChangePw => {
            return change_password(users[index].username.clone());
        }
    }
}

/**
 * confirm if a user should be deleted
 */
fn confirm_delete(username: String) -> Result<(), Box<dyn std::error::Error>> {
    if !Confirm::new()
        .with_prompt(format!("really delete {}", &username))
        .interact()?
    {
        return Ok(());
    }
    db::user::delete(username)?;
    Ok(())
}

fn change_password(username: String) -> Result<(), Box<dyn std::error::Error>> {
    let pw = Password::new()
        .with_prompt(format!("new password for: {}", &username))
        .interact()?;
    auth::change_password(username, pw)
}
