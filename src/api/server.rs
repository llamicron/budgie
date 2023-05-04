use crate::db;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use diesel::r2d2::{self, ConnectionManager};
use diesel::PgConnection;
use std::sync::Arc;

type DbPool = Arc<r2d2::Pool<ConnectionManager<PgConnection>>>;

#[get("/health")]
async fn hello(_: web::Data<DbPool>) -> impl Responder {
    HttpResponse::Ok()
}

#[actix_web::main]
pub async fn start() -> std::io::Result<()> {
    let db_url = db::db_url().expect("Couldn't get DB url from env variables or .env file");
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    let db_pool = Arc::new(r2d2::Pool::builder().build(manager).unwrap());

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db_pool.clone()))
            .service(web::scope("/api").service(hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
