use actix_web::{web, HttpResponse, Responder};
use crate::internal::api::http::v1::model::request::UsersQuery;
use crate::internal::api::http::v1::model::response::User;
use crate::internal::traits::UserRepo;
use crate::internal::entity::user::Model;

pub async fn hello() -> impl Responder {
    HttpResponse::Ok()
}

pub async fn users<R: UserRepo>(
    q: web::Query<UsersQuery>,
    repo: web::Data<R>,
) -> impl Responder {
    let users: Vec<User> = repo
        .get_many(
            q.skip.unwrap_or(0),
            q.limit.unwrap_or(10),
        )
        .await
        .into_iter()
        .map(from_model)
        .collect();
    HttpResponse::Ok()
        .content_type("application/json")
        .body(serde_json::to_string(&users).unwrap())
}

pub async fn user<R: UserRepo>(
    p: web::Path<i32>,
    repo: web::Data<R>,
) -> impl Responder {
    let maybe_user: Option<User> = repo
        .get_one(p.into_inner())
        .await
        .map(from_model);
    match maybe_user {
        Some(user) => HttpResponse::Ok().json(user),
        _ => HttpResponse::NotFound().finish(),
    }
}

fn from_model(u: Model) -> User {
    User{
        id:        u.id,
        name:      u.name,
        email:     u.email,
        skills:    u.skills,
        interests: u.interests,
    }
}
