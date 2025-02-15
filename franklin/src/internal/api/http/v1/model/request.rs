use serde::Deserialize;

#[derive(Deserialize)]
pub struct UsersQuery {
    pub skip: Option<u32>,
    pub limit: Option<u32>,
}
