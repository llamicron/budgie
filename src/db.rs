use crate::error::Result;
use diesel::prelude::*;
use log::*;
use std::env;

pub fn connect() -> Result<PgConnection> {
    let database_url = env::var("DATABASE_URL")?;
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
