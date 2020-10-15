use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};

use diesel::prelude::*;
use crate::database::{self, DbConn, models, schema};

#[derive(Deserialize, Serialize)]
struct StatusPayload {
    status: String
}

#[get("/")]
fn index() -> Json<StatusPayload> {
    Json(StatusPayload { status: "ok".to_string() })
}

#[get("/transactions/<trans_id>")]
fn get_transaction(conn: DbConn, trans_id: i32) -> Json<Option<models::Transaction>> {
    use schema::transactions::dsl::*;

    if let Ok(trans) = transactions.find(trans_id).get_result::<models::Transaction>(&*conn) {
        return Json(Some(trans));
    }
    Json(None)
}

pub fn run() {
    rocket::ignite()
        .manage(database::init_pool())
        .mount("/api", routes![index, get_transaction]).launch();
}
