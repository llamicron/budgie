mod api;
mod db;
mod error;
mod model;
mod schema;

use dotenvy::dotenv;

fn init_log() {
    env_logger::init();
}

fn main() {
    dotenv().ok();
    init_log();

    api::start().unwrap();
}

#[cfg(test)]
#[ctor::ctor]
fn init() {
    // Call dotenv before tests are run
    dotenv().ok();
}
