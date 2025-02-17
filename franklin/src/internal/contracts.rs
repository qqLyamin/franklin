use std::future::Future;
use sea_orm::sqlx::types::uuid;
use crate::internal::entity::user::Model;
use crate::internal::err;
use uuid::Uuid;

pub trait UserRepo {
    fn get_many(&self, skip: u32, limit: u32) -> impl Future<Output=Vec<Model>>;
    fn get_one(&self, id: Uuid) -> impl Future<Output=Option<Model>>;
    fn create(&self, model: Model) -> impl Future<Output=Result<Uuid, err::User>>;
}

pub struct Service<R: UserRepo> {
    pub repo:   R,
    pub secret: String,
}
