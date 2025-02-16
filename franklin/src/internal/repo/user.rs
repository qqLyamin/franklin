use crate::internal::traits::UserRepo;
use crate::internal::entity::{prelude::User, user::Model};
use sea_orm::{DatabaseConnection, Database, EntityTrait, QuerySelect, ConnectOptions};

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

    async fn get_one(&self, id: i32) -> Option<Model> {
        User::find_by_id(id)
            .one(&self.db)
            .await
            .unwrap()
    }
}

impl Repo {
    pub async fn new(db_url: String, max_conn: u32) -> Self {
        let mut opt = ConnectOptions::new(db_url);
        opt.max_connections(max_conn);
        let db = Database::connect(opt)
            .await
            .unwrap();
        Self {
            db,
        }
    }
}
