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

#[derive(Deserialize)]
pub struct UserPatchBody {
    pub name:         Option<String>,
    pub email:        Option<String>,
    pub old_password: Option<String>,
    pub password:     Option<String>,
    pub skills:       Option<String>,
    pub interests:    Option<String>,
}

#[derive(Deserialize)]
pub struct Login {
    pub name:     Option<String>,
    pub email:    Option<String>,
    pub password: String,
}
