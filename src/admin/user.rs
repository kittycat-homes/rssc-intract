use crate::database::user::*;
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

pub fn add_user() -> Result<(), Box<dyn std::error::Error>> {
    let username = Input::<String>::new()
        .with_prompt("username")
        .interact_text()?;
    let password = Password::new()
        .with_prompt("password")
        .allow_empty_password(false)
        .interact()?;
    let user = UserBuilder::default().username(username).
        //TODO stores passwords in plaintext, horrible, get rid of this
        hash(Some(password)).build()?;
    let _r = create(user)?;
    Ok(())
}
