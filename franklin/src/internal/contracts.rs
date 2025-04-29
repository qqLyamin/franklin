use std::future::Future;
use sea_orm::sqlx::types::uuid;
use crate::internal::entity::user::{Model, ActiveModel};
use crate::internal::err;
use uuid::Uuid;

pub struct UserFilters<'a> {
    pub interests: &'a Option<String>,
    pub skills:    &'a Option<String>,
}

pub trait UserRepo {
    fn get_many(&self, skip: u32, limit: u32, filters: UserFilters) -> impl Future<Output=Option<Vec<Model>>>;
    fn get_one(&self, id: Uuid) -> impl Future<Output=Result<Model, err::User>>;
    fn get_one_by_name(&self, name: &str) -> impl Future<Output=Result<Model, err::User>>;
    fn get_one_by_email(&self, email: &str) -> impl Future<Output=Result<Model, err::User>>;
    fn delete(&self, id: Uuid) -> impl Future<Output=Result<(), err::User>>;
    fn create(&self, model: Model) -> impl Future<Output=Result<Uuid, err::User>>;
    fn update(&self, model: ActiveModel) -> impl Future<Output=Result<(), err::User>>;
}

pub struct Service<R: UserRepo> {
    pub repo:   R,
    pub secret: String,
}
