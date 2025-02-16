mod internal;

use actix_web::{web, App, HttpServer};
use crate::internal::repo::user::Repo;
use internal::api::http::v1::franklin;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let repo = Repo::new().await;
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
