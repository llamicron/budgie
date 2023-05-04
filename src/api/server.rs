use crate::error::UserError;
use actix_web::middleware::Logger;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Result};
use serde::{Deserialize, Serialize};

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
    Err(UserError::CustomError(String::from(
        "an (expected) error occured",
    )))
}

#[actix_web::main]
pub async fn start() -> std::io::Result<()> {
    let db_pool = crate::db::DbPool::connect().expect("Couldn't connect to database pool");

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
