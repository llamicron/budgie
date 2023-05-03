mod db;
mod error;
mod line_item;
mod schema;
mod transaction;

use dotenvy::dotenv;

fn init_log() {
    env_logger::init();
}

fn main() {
    dotenv().ok();
    init_log();
}

#[cfg(test)]
#[ctor::ctor]
fn init() {
    // Call dotenv before tests are run
    dotenv().ok();
}
