use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};

use diesel::prelude::*;
use crate::database::{self, DbConn, models, schema};

type JsonResult<T> = Json<std::result::Result<T, StatusPayload>>;

/// A generic payload with a status and message
#[derive(Deserialize, Serialize)]
struct StatusPayload {
    status: String,
    message: Option<String>
}

impl StatusPayload {
    fn ok(message: &str) -> StatusPayload {
        StatusPayload {
            status: "ok".to_string(),
            message: Some(String::from(message))
        }
    }

    fn error(message: &str) -> StatusPayload {
        StatusPayload {
            status: "error".to_string(),
            message: Some(String::from(message))
        }
    }
}

#[get("/")]
fn index() -> Json<StatusPayload> {
    Json(StatusPayload::ok("ok"))
}

#[get("/transactions/<trans_id>")]
fn get_transaction(conn: DbConn, trans_id: i32) -> JsonResult<models::Transaction> {
    use schema::transactions::dsl::*;

    if let Ok(trans) = transactions.find(trans_id).get_result::<models::Transaction>(&*conn) {
        return Json(Ok(trans));
    }

    Json(Err(StatusPayload::error("resource does not exist")))
}

#[post("/transactions/new", format = "application/json", data = "<trans>")]
fn create_transaction(conn: DbConn, trans: Json<models::NewTransaction>) -> JsonResult<models::Transaction> {
    use schema::transactions::dsl::*;

    let result = diesel::insert_into(transactions).values(&trans.into_inner()).get_result(&*conn);

    match result {
        Ok(created_trans) => return Json(Ok(created_trans)),
        Err(e) => return Json(Err(StatusPayload::error(&format!("error: {}", e))))
    }
}

pub fn run() {
    rocket::ignite()
        .manage(database::init_pool())
        .mount("/api", routes![
            index,
            get_transaction,
            create_transaction
        ]).launch();
}
