use actix_web::middleware::Logger;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

use super::resources;

#[get("/health")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok()
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
                    .configure(resources::budget::routes)
                    .configure(resources::line_item_group::routes)
                    .configure(resources::line_item::routes),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
