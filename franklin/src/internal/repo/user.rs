use crate::internal::traits::UserRepo;
use crate::internal::entity::{prelude::User, user::Model};
use sea_orm::*;

#[derive(Clone)]
pub struct Repo {
    db: DatabaseConnection,
}

impl UserRepo for Repo {
    async fn get_many(&self, skip: u32, limit: u32) -> Vec<Model> {
        User::find()
            .offset(skip as u64)
            .limit(limit as u64)
            .all(&self.db)
            .await
            .unwrap()
    }
}

impl Repo {
    pub async fn new(db_url: String) -> Self {
        let db = Database::connect(db_url)
            .await
            .unwrap();
        Self {
            db,
        }
    }
}
