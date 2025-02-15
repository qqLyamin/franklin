use actix_web::{get, web, HttpResponse, Responder};
use crate::internal::api::http::v1::model::request::UsersQuery;
use crate::internal::api::http::v1::model::response::User;
use crate::internal::repo::user::Repo;
use crate::internal::traits::UserRepo;

#[get("/ping")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok()
}

#[get("/users/")]
pub async fn users(q: web::Query<UsersQuery>) -> impl Responder {
    let repo = Repo::new();
    let users: Vec<User> = repo
        .get_many(
            q.skip.unwrap_or(0),
            q.limit.unwrap_or(10),
        )
        .await
        .into_iter()
        .map(|u| User{
            id: u.id,
            name: u.name,
            email: u.email,
            skills: u.skills,
            interests: u.interests,
        })
        .collect();
    HttpResponse::Ok()
        .content_type("application/json")
        .body(serde_json::to_string(&users).unwrap())
}
