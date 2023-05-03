mod line_item;
mod schema;

use diesel::prelude::*;
use dotenvy::dotenv;
use log::*;
use std::env;

type ID = i32;

fn init_log() {
    env_logger::init();
}

fn connect() -> PgConnection {
    let database_url = env::var("DATABASE_URL").expect("DB url must be set");
    info!("Attempting to connect to DB with database url: {database_url}");
    PgConnection::establish(&database_url).expect("Couldn't connect to db")
}

fn main() {
    dotenv().ok();
    init_log();

    use self::schema::line_items::dsl::*;
    use crate::line_item::LineItem;

    let db = &mut connect();
    let results = line_items
        .filter(balance.eq(0.0))
        .limit(5)
        .load::<LineItem>(db)
        .expect("Error loading posts");
}
