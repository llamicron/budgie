use crate::error::Result;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use std::env;
use std::sync::{Arc, Mutex};

#[cfg(test)]
use log::*;

#[derive(Clone)]
pub struct DbPool {
    pub conns: Arc<Mutex<r2d2::Pool<ConnectionManager<PgConnection>>>>,
}

impl DbPool {
    pub fn connect() -> Result<DbPool> {
        let manager = ConnectionManager::<PgConnection>::new(db_url()?);
        let arced_pool = Arc::new(Mutex::new(r2d2::Pool::builder().build(manager).unwrap()));
        Ok(Self { conns: arced_pool })
    }
}

pub fn db_url() -> Result<String> {
    env::var("DATABASE_URL").map_err(|e| e.into())
}

#[cfg(test)]
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
