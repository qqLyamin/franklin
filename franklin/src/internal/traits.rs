use std::future::Future;
use crate::internal::entity::user::Model;
pub trait UserRepo {
    fn get_many(&self, skip: u32, limit: u32) -> impl Future<Output=Vec<Model>>;
}
