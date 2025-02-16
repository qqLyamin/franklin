mod internal;

use actix_web::{web, App, HttpServer};
use crate::internal::repo::user::Repo;
use internal::api::http::v1::franklin;
use dotenvy::dotenv;
use std::env;
use actix_cors::Cors;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").unwrap();
    let max_conn = env::var("DATABASE_MAX_CONN").unwrap().parse::<u32>().unwrap();
    let allowed_origin = env::var("ALLOWED_ORIGIN").unwrap();
    let repo = Repo::new(db_url, max_conn).await;
    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin(&allowed_origin);
        App::new()
            .wrap(cors)
            .app_data(web::Data::new(repo.clone()))
            .route("/users", web::get().to(franklin::users::<Repo>))
            .route("/users/{id}", web::get().to(franklin::user::<Repo>))
            .route("/ping", web::get().to(franklin::hello))
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
