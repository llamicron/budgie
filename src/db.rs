use crate::error::Result;
use diesel::prelude::*;
use log::*;
use std::env;

pub fn db_url() -> Result<String> {
    env::var("DATABASE_URL").map_err(|e| e.into())
}
pub fn connect() -> Result<PgConnection> {
    let database_url = db_url()?;
    info!("Attempting to connect to DB with database url: {database_url}");
    PgConnection::establish(&database_url).map_err(|e| e.into())
}

/// Clears all rows in all tables
#[cfg(test)]
pub fn nuke(db: &mut PgConnection) {
    diesel::sql_query("TRUNCATE TABLE line_items, line_item_groups, transactions CASCADE;")
        .execute(db)
        .unwrap();
}
