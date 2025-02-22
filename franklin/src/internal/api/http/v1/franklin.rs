use actix_web::{
    web,
    HttpResponse,
    Responder,
    cookie::Cookie,
};
use crate::internal::api::http::v1::model::request::{UsersQuery, UserBody};
use crate::internal::api::http::v1::model::response::{User, UserCreated};
use crate::internal::contracts::{UserRepo, Service};
use crate::internal::entity::user::Model;
use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHasher, SaltString
    },
    Argon2
};
use uuid::Uuid;
use crate::internal::err;
use hmac::{Hmac, Mac};
use jwt::SignWithKey;
use sha2::Sha256;
use std::{
    collections::BTreeMap,
    time::SystemTime,
};

pub async fn hello() -> impl Responder {
    HttpResponse::Ok()
}

pub async fn users<R: UserRepo>(
    q: web::Query<UsersQuery>,
    service: web::Data<Service<R>>,
) -> impl Responder {
    let users: Vec<User> = service.repo
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
    service: web::Data<Service<R>>,
) -> impl Responder {
    service.repo
        .get_one(p.into_inner())
        .await
        .map(from_model)
        .map(|user| HttpResponse::Ok().json(user))
        .unwrap_or(HttpResponse::NotFound().finish())
}

pub async fn sign_up<R: UserRepo>(
    body: web::Json<UserBody>,
    service: web::Data<Service<R>>,
) -> impl Responder {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let internal_error = HttpResponse::InternalServerError()
        .body("failed to create user");
    match argon2.hash_password(body.password.as_bytes(), &salt) {
        Err(_) => internal_error,
        Ok(hashed_password) => service.repo
            .create(Model{
                id:              Uuid::new_v4(),
                name:            body.name.clone().unwrap_or(String::new()),
                email:           body.email.clone(),
                salt:            salt.to_string(),
                interests:       body.interests.clone().unwrap_or(String::new()),
                skills:          body.skills.clone().unwrap_or(String::new()),
                hashed_password: hashed_password.to_string(),
            })
            .await
            .map(|id| {
                let key: Hmac<Sha256> = Hmac::new_from_slice(&service.secret.as_bytes())
                    .unwrap();
                let mut claims = BTreeMap::new();
                claims.insert("sub", id.to_string());
                claims.insert("exp", SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .unwrap()
                    .as_secs()
                    .to_string(),
                );
                let jwt = claims.sign_with_key(&key).unwrap();
                let c = Cookie::build("jwt", jwt.clone())
                    .domain("127.0.0.1:8080")
                    .path("/")
                    .secure(true)
                    .http_only(true)
                    .finish();
                HttpResponse::Ok()
                    .cookie(c)
                    .json(UserCreated{
                        id,
                        jwt,
                    })
            })
            .unwrap_or_else(|e| match e {
                err::User::EmailExists => HttpResponse::Conflict()
                    .body(format!("email {} already exists", body.email)),

                err::User::NameExists => HttpResponse::Conflict()
                    .body(format!("name {} already exists", body.name.clone().unwrap())),

                _ => internal_error,
            })
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
