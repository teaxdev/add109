use serde::{Serialize, Deserialize};
use rocket::form::FromForm;

#[derive(Debug, Serialize, Deserialize)]
pub struct Note {
    pub id: i32,
    pub user_id: i32,
    pub title: String,
    pub content: Option<String>,
    pub color: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, FromForm)]
pub struct NewNote {
    pub title: String,
    pub content: Option<String>,
    pub color: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub created_at: String,
}

#[derive(Debug, FromForm)]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, FromForm)]
pub struct LoginForm {
    pub username: String,
    pub password: String,
}
