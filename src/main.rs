mod line_item;
mod schema;

use crate::line_item::LineItem;
use diesel::prelude::*;
use dotenvy::dotenv;
use log::*;
use std::env;

use crate::line_item::{LineItemKind, NewLineItem};

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

    let db = &mut connect();

    let line_item = LineItem::create(db, &LineItemKind::Standard, "Gas", &120.0, None);

    let results = line_items
        .filter(planned.eq(120.0))
        .limit(5)
        .load::<LineItem>(db)
        .expect("Error loading posts");
    println!("{:?}", results);
}
