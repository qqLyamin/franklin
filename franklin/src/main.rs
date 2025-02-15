mod internal;

use actix_web::{App, HttpServer};

use internal::api::http::v1::franklin;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(franklin::hello)
            .service(franklin::users)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
