use actix_web::{web, HttpResponse, Responder};
use crate::internal::api::http::v1::model::request::{UsersQuery, UserBody};
use crate::internal::api::http::v1::model::response::User;
use crate::internal::traits::UserRepo;
use crate::internal::entity::user::Model;
use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHasher, SaltString
    },
    Argon2
};
use uuid::Uuid;

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
    HttpResponse::Ok().json(users)
}

pub async fn user<R: UserRepo>(
    p: web::Path<Uuid>,
    repo: web::Data<R>,
) -> impl Responder {
    repo
        .get_one(p.into_inner())
        .await
        .map(from_model)
        .map(|user| HttpResponse::Ok().json(user))
        .unwrap_or(HttpResponse::NotFound().finish())
}

pub async fn sign_up<R: UserRepo>(
    body: web::Json<UserBody>,
    repo: web::Data<R>,
) -> impl Responder {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let hashed_password = argon2
        .hash_password(body.password.as_bytes(), &salt)?
        .to_string();
    repo
        .create(Model{
            id:        Uuid::new_v4(),
            name:      body.name.clone().unwrap_or(String::new()),
            email:     body.email.clone(),
            salt:      salt.to_string(),
            interests: body.interests.clone().unwrap_or(String::new()),
            skills:    body.skills.clone().unwrap_or(String::new()),
            hashed_password,
        })
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
