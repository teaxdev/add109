use rocket::form::Form;
use rocket::http::{Status, CookieJar};
use rocket::response::Redirect;
use rocket::serde::json::{Json, serde_json};
use rocket::{get, post, State};
use rocket_dyn_templates::Template;
use std::collections::HashMap;

use crate::db::DbPool;
use crate::error::{db_error, pool_error, RouteResult};
use crate::models::{NewNote, Note, NewUser, User, LoginForm};

#[get("/")]
pub fn index(pool: &State<DbPool>, cookies: &CookieJar<'_>) -> Template {
    let mut context: HashMap<&str, String> = HashMap::new();
    context.insert("title", "Rust Notes App".to_string());
    context.insert("message", "Noty ".to_string());
    
    if let Some(user_id_cookie) = cookies.get_private("user_id") {
        if let Ok(user_id) = user_id_cookie.value().parse::<i32>() {
            if let Ok(conn) = pool.get() {
                if let Ok(username) = conn.query_row(
                    "SELECT username FROM users WHERE id = ?1",
                    rusqlite::params![user_id],
                    |row| row.get::<_, String>(0),
                ) {
                    context.insert("logged_in", "true".to_string());
                    context.insert("username", username);
                }
            }
        }
    }
    
    if !context.contains_key("logged_in") {
        context.insert("logged_in", "false".to_string());
    }

    Template::render("index", &context)
}

#[get("/signup")]
pub fn signup() -> Template {
    Template::render("signup", &[("title", "Rust Noty App")].iter().cloned().collect::<HashMap<_, _>>())
}

#[get("/login")]
pub fn login_page() -> Template {
    Template::render("login", &[("title", "Rust Noty App")].iter().cloned().collect::<HashMap<_, _>>())
}

#[post("/login", data = "<login_form>")]
pub fn login(
    pool: &State<DbPool>,
    login_form: Form<LoginForm>,
    cookies: &CookieJar<'_>,
) -> RouteResult<Redirect> {
    let conn = pool.get().map_err(pool_error)?;
    let login = login_form.into_inner();

    if let Ok((user_id, stored_password)) = conn.query_row(
        "SELECT id, password FROM users WHERE username = ?1",
        rusqlite::params![login.username],
        |row| Ok((row.get::<_, i32>(0)?, row.get::<_, String>(1)?)),
    ) {
        if stored_password == login.password {
            cookies.add_private(rocket::http::Cookie::new("user_id", user_id.to_string()));
            return Ok(Redirect::to(uri!(crate::routes::index)));
        }
    }

    Err(Status::Unauthorized)
}

#[get("/logout")]
pub fn logout(cookies: &CookieJar<'_>) -> Redirect {
    cookies.remove_private(rocket::http::Cookie::from("user_id"));
    Redirect::to(uri!(crate::routes::index))
}

#[get("/notes")]
pub fn get_notes(pool: &State<DbPool>, cookies: &CookieJar<'_>) -> RouteResult<Template> {
    // Get user_id from cookie
    let user_id = cookies
        .get_private("user_id")
        .and_then(|cookie| cookie.value().parse::<i32>().ok())
        .ok_or(Status::Unauthorized)?;

    let conn = pool.get().map_err(pool_error)?;

    let mut stmt = conn
        .prepare("SELECT id, user_id, title, content, color, created_at, updated_at FROM notes WHERE user_id = ?1 ORDER BY created_at DESC")
        .map_err(db_error)?;

    let notes_iter = stmt
        .query_map(rusqlite::params![user_id], |row| {
            Ok(Note {
                id: row.get(0)?,
                user_id: row.get(1)?,
                title: row.get(2)?,
                content: row.get(3)?,
                color: row.get::<_, Option<String>>(4)?,
                created_at: row.get(5)?,
                updated_at: row.get(6)?,
            })
        })
        .map_err(db_error)?;

    let mut notes = Vec::new();
    for note in notes_iter {
        notes.push(note.map_err(db_error)?);
    }

    let mut context: HashMap<String, serde_json::Value> = HashMap::new();
    context.insert("title".to_string(), serde_json::Value::String("My Notes".to_string()));
    context.insert("notes_count".to_string(), serde_json::Value::Number(notes.len().into()));
    context.insert("notes".to_string(), serde_json::to_value(&notes).map_err(|_| Status::InternalServerError)?);

    Ok(Template::render("notes", &context))
}

#[post("/notes", data = "<note_form>")]
pub fn create_note(
    pool: &State<DbPool>,
    note_form: Form<NewNote>,
    cookies: &CookieJar<'_>,
) -> RouteResult<Redirect> {
    // Get user_id from cookie
    let user_id = cookies
        .get_private("user_id")
        .and_then(|cookie| cookie.value().parse::<i32>().ok())
        .ok_or(Status::Unauthorized)?;
    
    let conn = pool.get().map_err(pool_error)?;
    let new_note = note_form.into_inner();

    conn.execute(
        "INSERT INTO notes (user_id, title, content, color) VALUES (?1, ?2, ?3, ?4)",
        rusqlite::params![
            user_id,
            new_note.title,
            new_note.content.unwrap_or_default(),
            new_note.color,
        ],
    )
    .map_err(db_error)?;

    // After POST redirect back to index or notes page
    Ok(Redirect::to(uri!(crate::routes::get_notes)))
}

#[post("/notes/<note_id>/delete")]
pub fn delete_note(
    pool: &State<DbPool>,
    note_id: i32,
    cookies: &CookieJar<'_>,
) -> RouteResult<Redirect> {
    let user_id = cookies
        .get_private("user_id")
        .and_then(|cookie| cookie.value().parse::<i32>().ok())
        .ok_or(Status::Unauthorized)?;

    let conn = pool.get().map_err(pool_error)?;

    // Delete note only if it exists and belongs to the user
    match conn.execute(
        "DELETE FROM notes WHERE id = ?1 AND user_id = ?2",
        rusqlite::params![note_id, user_id],
    ) {
        Ok(0) => Err(Status::NotFound),
        Ok(_) => Ok(Redirect::to(uri!(crate::routes::get_notes))),
        Err(e) => Err(db_error(e)),
    }
}

#[get("/users")]
pub fn get_users(pool: &State<DbPool>) -> RouteResult<Json<Vec<User>>> {
    let conn = pool.get().map_err(pool_error)?;
    let mut stmt = conn
        .prepare("SELECT id, username, email, created_at FROM users")
        .map_err(db_error)?;

    let users: Result<Vec<_>, _> = stmt
        .query_map([], |row| {
            Ok(User {
                id: row.get(0)?,
                username: row.get(1)?,
                email: row.get(2)?,
                created_at: row.get(3)?,
            })
        })
        .map_err(db_error)?
        .collect();

    Ok(Json(users.map_err(db_error)?))
}

#[post("/users", data = "<user_form>")]
pub fn create_user(
    pool: &State<DbPool>,
    user_form: Form<NewUser>,
) -> RouteResult<Redirect> {
    let conn = pool.get().map_err(pool_error)?;
    let new_user = user_form.into_inner();

    // Check if username or email already exists
    let exists: bool = conn
        .query_row(
            "SELECT EXISTS(SELECT 1 FROM users WHERE username = ?1 OR email = ?2)",
            rusqlite::params![new_user.username, new_user.email],
            |row| row.get(0),
        )
        .map_err(db_error)?;

    if exists {
        return Err(Status::Conflict);
    }

    conn.execute(
        "INSERT INTO users (username, email, password) VALUES (?1, ?2, ?3)",
        rusqlite::params![new_user.username, new_user.email, new_user.password],
    )
    .map_err(db_error)?;

    Ok(Redirect::to(uri!(crate::routes::index)))
}