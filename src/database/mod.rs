use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use std::error::Error;
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

pub mod caption;
pub mod category;
pub mod feed;
pub mod image;
pub mod link;
pub mod media;
pub mod post;
pub mod schema;
pub mod sessionid;
pub mod subscription;
pub mod user;

pub fn establish_connection() -> PgConnection {
    let database_url = &crate::CONF.database.url;

    PgConnection::establish(database_url)
        .unwrap_or_else(|_| panic!("error connecting at {}", database_url))
}

pub fn run_migrations() -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    let connection = &mut establish_connection();
    if connection.has_pending_migration(MIGRATIONS)? {
        info!("running database migrations!");
    }
    connection.run_pending_migrations(MIGRATIONS)?;

    Ok(())
}
