use serde::{Serialize, Deserialize};
use rocket::form::FromForm;

#[derive(Debug, Serialize, Deserialize)]
pub struct Note {
    pub id: i32,
    pub user_id: i32,
    pub title: String,
    pub content: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, FromForm)]
pub struct NewNote {
    pub user_id: i32,
    pub title: String,
    pub content: Option<String>,
}
