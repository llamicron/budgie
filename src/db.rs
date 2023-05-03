use diesel::prelude::*;
use log::*;
use std::env;

pub fn connect() -> PgConnection {
    let database_url = env::var("DATABASE_URL").expect("DB url must be set");
    info!("Attempting to connect to DB with database url: {database_url}");
    PgConnection::establish(&database_url).expect("Couldn't connect to db")
}
