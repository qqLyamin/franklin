use serde::Deserialize;

#[derive(Deserialize)]
pub struct UsersQuery {
    pub skip:  Option<u32>,
    pub limit: Option<u32>,
}

#[derive(Deserialize)]
pub struct UserBody {
    pub name:      Option<String>,
    pub email:     String,
    pub password:  String,
    pub skills:    Option<String>,
    pub interests: Option<String>,
}
