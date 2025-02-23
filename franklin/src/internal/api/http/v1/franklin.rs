use actix_web::{
    web,
    HttpResponse,
    Responder,
    cookie::Cookie,
};
use crate::internal::api::http::v1::model::request::{UsersQuery, UserBody, UserPatchBody};
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

pub async fn get_user<R: UserRepo>(
    p: web::Path<Uuid>,
    service: web::Data<Service<R>>,
) -> impl Responder {
    service.repo
        .get_one(p.into_inner())
        .await
        .map(from_model)
        .map(|user| HttpResponse::Ok().json(user))
        .unwrap_or_else(|e| match e {
            err::User::NotFound => HttpResponse::NotFound().finish(),
            _ => HttpResponse::InternalServerError().finish(),
        })
}

pub async fn sign_up<R: UserRepo>(
    body: web::Json<UserBody>,
    service: web::Data<Service<R>>,
) -> impl Responder {
    let internal_error = HttpResponse::InternalServerError()
        .body("failed to create user");
    let maybe_hashed_password = hash_password(&body.password, None);
    if maybe_hashed_password.is_none() {
        return internal_error;
    }
    let (hashed_password, salt) = maybe_hashed_password.unwrap();
    service.repo
        .create(Model{
            id:        Uuid::new_v4(),
            name:      body.name.clone().unwrap_or(String::new()),
            email:     body.email.clone(),
            interests: body.interests.clone().unwrap_or(String::new()),
            skills:    body.skills.clone().unwrap_or(String::new()),
            salt,
            hashed_password,
        })
        .await
        .map(|id| {
            let key: Hmac<Sha256> = Hmac::new_from_slice(&service.secret.as_bytes())
                .unwrap();
            let mut claims = BTreeMap::new();
            claims.insert("sub", id.to_string());
            let time_unix = SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs();
            claims.insert("exp", (time_unix + 7*24*60*60).to_string());
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
        .unwrap_or_else(|e| map_upsert_err(e, body.name.clone(), Some(body.email.clone())))
}

pub async fn delete_user<R: UserRepo>(
    p: web::Path<Uuid>,
    service: web::Data<Service<R>>,
) -> impl Responder {
    let result = service.repo
        .delete(p.into_inner())
        .await;
    match result {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(err::User::NotFound) => HttpResponse::NotFound().finish(),
        _ => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn update_user<R: UserRepo>(
    p: web::Path<Uuid>,
    body: web::Json<UserPatchBody>,
    service: web::Data<Service<R>>,
) -> impl Responder {
    let maybe_user = service.repo.get_one(p.into_inner()).await;
    if let Err(e) = maybe_user {
        return match e {
            err::User::NotFound => HttpResponse::NotFound().finish(),
            _ => HttpResponse::InternalServerError().finish(),
        }
    }
    let user = maybe_user.ok().unwrap();
    if let Some(old_password) = &body.old_password {
        if let Some((hp, _)) = hash_password(old_password, Some(&user.salt)) {
            if hp.ne(&user.hashed_password) {
                return HttpResponse::Forbidden().finish();
            }
        } else {
            return HttpResponse::InternalServerError().finish();
        }
    };
    let maybe_hashed_password = match &body.password {
        Some(password) => hash_password(password, None),
        _ => Some((user.hashed_password, user.salt)),
    };
    if maybe_hashed_password.is_none() {
        return HttpResponse::InternalServerError().finish();
    }
    let (hashed_password, salt) = maybe_hashed_password.unwrap();
    service.repo
        .update(Model{
            id:        user.id.clone(),
            name:      body.name.clone().unwrap_or(user.name),
            email:     body.email.clone().unwrap_or(user.email),
            interests: body.interests.clone().unwrap_or(user.interests),
            skills:    body.skills.clone().unwrap_or(user.skills),
            salt,
            hashed_password,
        })
        .await
        .map(|_| HttpResponse::NoContent().finish())
        .unwrap_or_else(|e| map_upsert_err(e, body.name.clone(), body.email.clone()))
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

fn hash_password(password: &str, maybe_salt_base64: Option<&str>) -> Option<(String, String)> {
    let salt = match maybe_salt_base64 {
        Some(salt_base64) => SaltString::from_b64(salt_base64),
        _ => Ok(SaltString::generate(&mut OsRng)),
    }.ok()?;
    let ph = Argon2::default().hash_password(password.as_bytes(), &salt).ok()?;
    Some((ph.to_string(), salt.to_string()))
}

// todo: return closure
fn map_upsert_err(
    e: err::User,
    name: Option<String>,
    email: Option<String>,
) -> HttpResponse {
    match e {
        err::User::EmailExists => HttpResponse::Conflict()
            .body(format!("email {} already exists", email.unwrap())),

        err::User::NameExists => HttpResponse::Conflict()
            .body(format!("name {} already exists", name.unwrap())),

        _ => HttpResponse::InternalServerError().finish(),
    }
}
