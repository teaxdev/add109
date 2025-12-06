#[macro_use]
extern crate rocket;

use crate::db::init_db;

mod db;
mod routes;
mod models;
mod error;

use rocket::fs::FileServer;
use rocket_dyn_templates::Template;

#[launch]
fn rocket() -> _ {
    let pool = init_db(); // gets the pool

    rocket::build()
        .manage(pool) // <- register the pool for State<DbPool>
        .attach(Template::fairing()) // for tera templates
        .mount("/static", FileServer::from("static")) // css
        .mount(
            "/",
            routes![
                routes::index,
                routes::get_notes,
                routes::create_note,
                routes::delete_note,
                routes::get_users,
                routes::create_user,
                routes::signup,
                routes::login_page,
                routes::login,
                routes::logout,
            ],
        )
}
