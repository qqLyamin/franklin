mod internal;

use actix_web::{web, App, HttpServer};
use crate::internal::repo::user::Repo;
use internal::api::http::v1::franklin;
use dotenvy::dotenv;
use actix_cors::Cors;
use crate::internal::config::Config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let cfg = Config::new();
    let repo = Repo::new(cfg.db_url, cfg.max_conn).await;
    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin(&cfg.allowed_origin);
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
