use crate::internal::contracts::UserRepo;
use crate::internal::entity::{prelude::User, user::Model};
use sea_orm::{
    DatabaseConnection,
    Database,
    EntityTrait,
    QuerySelect,
    ConnectOptions,
    SqlErr,
    IntoActiveModel,
    ActiveModelTrait,
    DbErr,
};
use crate::internal::err;
use uuid::Uuid;

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

    async fn get_one(&self, id: Uuid) -> Result<Model, err::User> {
        match User::find_by_id(id).one(&self.db).await {
            Ok(Some(user)) => Ok(user),
            Ok(None) => Err(err::User::NotFound),
            _ => Err(err::User::DB),
        }
    }

    async fn create(&self, model: Model) -> Result<Uuid, err::User> {
        let id = model.id.clone();
        User::insert(model.into_active_model())
            .exec(&self.db)
            .await
            .map(|_| id)
            .map_err(map_upsert_err)
    }

    async fn update(&self, model: Model) -> Result<(), err::User> {
        model
            .into_active_model()
            .update(&self.db)
            .await
            .map(|_| ())
            .map_err(map_upsert_err)
    }

    async fn delete(&self, id: Uuid) -> Result<(), err::User> {
        let result = User::delete_by_id(id)
            .exec(&self.db)
            .await;
        match result {
            Ok(r) => if r.rows_affected == 0 { Err(err::User::NotFound) } else { Ok(()) },
            _ => Err(err::User::DB),
        }
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

fn map_upsert_err(e: DbErr) -> err::User {
    match e.sql_err() {
        Some(SqlErr::UniqueConstraintViolation(name)) => {
            if name.contains("user_name_key") {
                err::User::NameExists
            } else {
                err::User::EmailExists
            }
        },
        _ => err::User::DB,
    }
}
