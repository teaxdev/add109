use rocket::form::Form;
use rocket::response::Redirect;
use rocket::serde::json::Json;
use rocket::{get, State};
use rocket_dyn_templates::Template;
use std::collections::HashMap;

use crate::db::DbPool;
use crate::error::{db_error, pool_error, RouteResult};
use crate::models::{NewNote, Note};

#[get("/")]
pub fn index() -> Template {
    let mut context = HashMap::new();
    context.insert("title", "Rust Notes App");
    context.insert("message", "Noty ");

    Template::render("index", &context)
}

#[get("/notes")]
pub fn get_notes(pool: &State<DbPool>) -> RouteResult<Json<Vec<Note>>> {
    let conn = pool.get().map_err(pool_error)?;

    let mut stmt = conn
        .prepare("SELECT id, user_id, title, content, created_at, updated_at FROM notes")
        .map_err(db_error)?;

    let notes_iter = stmt
        .query_map([], |row| {
            Ok(Note {
                id: row.get(0)?,
                user_id: row.get(1)?,
                title: row.get(2)?,
                content: row.get(3)?,
                created_at: row.get(4)?,
                updated_at: row.get(5)?,
            })
        })
        .map_err(db_error)?;

    let mut notes = Vec::new();
    for note in notes_iter {
        notes.push(note.map_err(db_error)?);
    }

    Ok(Json(notes))
}

#[post("/notes", data = "<note_form>")]
pub fn create_note(
    pool: &State<DbPool>,
    note_form: Form<NewNote>,
) -> RouteResult<Redirect> {
    let conn = pool.get().map_err(pool_error)?;
    let new_note = note_form.into_inner();

    conn.execute(
        "INSERT INTO notes (user_id, title, content) VALUES (?1, ?2, ?3)",
        rusqlite::params![
            new_note.user_id,
            new_note.title,
            new_note.content.unwrap_or_default()
        ],
    )
    .map_err(db_error)?;

    // After POST redirect back to index or notes page
    Ok(Redirect::to(uri!(crate::routes::index)))
}