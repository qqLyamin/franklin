mod internal;

use actix_web::{web, App, HttpServer};
use crate::internal::repo::user::Repo;
use internal::api::http::v1::franklin;
use dotenvy::dotenv;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").unwrap();
    let repo = Repo::new(db_url).await;
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(repo.clone()))
            .route("/users/", web::get().to(franklin::users::<Repo>))
            .route("/ping", web::get().to(franklin::hello))
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
