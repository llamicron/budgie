#![feature(proc_macro_hygiene, decl_macro)]
extern crate serde;
extern crate serde_json;
extern crate chrono;
extern crate uuid;
extern crate dotenv;
extern crate r2d2;
extern crate r2d2_diesel;

#[macro_use] 
extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate diesel;




mod database;
mod api;

fn main() {
    api::run();
}
