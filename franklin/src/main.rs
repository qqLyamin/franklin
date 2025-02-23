mod internal;

use actix_web::{web, App, HttpServer, middleware::from_fn};
use crate::internal::repo::user::Repo;
use internal::api::http::v1::{
    franklin,
    middleware::auth::{
        jwt,
        id_checker
    },
};
use dotenvy::dotenv;
use actix_cors::Cors;
use crate::internal::config::Config;
use crate::internal::contracts::Service;

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
            .app_data(web::Data::new(Service{
                repo:   repo.clone(),
                secret: cfg.secret.clone(),
            }))
            .route("/users", web::get().to(franklin::users::<Repo>))
            .route("/users", web::post().to(franklin::sign_up::<Repo>))
            .route("/login", web::post().to(franklin::login::<Repo>))
            .route("/users/{id}", web::get().to(franklin::get_user::<Repo>))
            .service(
                web::resource("/users/{id}")
                    .wrap(from_fn(jwt::<Repo>))
                    .wrap(from_fn(id_checker))
                    .route(web::delete().to(franklin::delete_user::<Repo>))
                    .route(web::patch().to(franklin::update_user::<Repo>))
            )
            .route("/ping", web::get().to(franklin::hello))
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
