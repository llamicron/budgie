use crate::error::Result;
use diesel::prelude::*;
use log::*;
use std::env;

pub fn connect() -> Result<PgConnection> {
    let database_url = env::var("DATABASE_URL")?;
    info!("Attempting to connect to DB with database url: {database_url}");
    PgConnection::establish(&database_url).map_err(|e| e.into())
}
