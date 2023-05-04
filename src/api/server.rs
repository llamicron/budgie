use crate::db::db_url;
use crate::error::UserError;
use actix_web::middleware::Logger;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Result};
use diesel::r2d2::{self, ConnectionManager};
use diesel::PgConnection;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

use super::resources;

#[get("/health")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

#[derive(Deserialize, Serialize)]
struct PingPong {
    ping: Option<String>,
    pong: Option<String>,
}

#[post("/ping")]
async fn ping(ping_pong: web::Json<PingPong>) -> Result<impl Responder> {
    let mut resp = PingPong {
        ping: None,
        pong: None,
    };
    if let Some(ping) = &ping_pong.ping {
        resp.pong = Some(ping.clone());
    }
    Ok(web::Json(resp))
}

#[get("/error")]
async fn forced_error() -> Result<&'static str, UserError> {
    let err = UserError::CustomError(String::from("an (expected) error occured"));
    Err(err.into())
}

#[actix_web::main]
pub async fn start() -> std::io::Result<()> {
    let db_url = db_url().expect("Couldn't get DB url from env variables or .env file");
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    let arced_pool = Arc::new(Mutex::new(r2d2::Pool::builder().build(manager).unwrap()));
    let db_pool = crate::db::DbPool { conns: arced_pool };

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(db_pool.clone()))
            .service(
                web::scope("/api")
                    .service(health_check)
                    .service(ping)
                    .service(forced_error)
                    .configure(resources::budget::routes),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
