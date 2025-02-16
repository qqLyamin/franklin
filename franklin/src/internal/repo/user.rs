use crate::internal::traits::UserRepo;
use crate::internal::entity::{prelude::User, user::Model};
use sea_orm::*;

#[derive(Clone)]
pub struct Repo {}

impl UserRepo for Repo {
    async fn get_many(&self, skip: u32, limit: u32) -> Vec<Model> {
        let db = Database::connect("postgresql://frank:ben@localhost/franklinclub")
            .await
            .unwrap();
        User::find()
            .offset(skip as u64)
            .limit(limit as u64)
            .all(&db)
            .await
            .unwrap()
    }
}

impl Repo {
    pub fn new() -> Self {
        Self {}
    }
}
