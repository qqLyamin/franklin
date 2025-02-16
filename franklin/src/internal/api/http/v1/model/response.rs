use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
pub struct User {
    pub id:        Uuid,
    pub name:      String,
    pub email:     String,
    pub skills:    String,
    pub interests: String,
}

#[derive(Serialize)]
pub struct UserCreated {
    pub id:  Uuid,
    pub jwt: String,
}
