use r2d2_sqlite::SqliteConnectionManager;
use r2d2::Pool;
use rusqlite::{params, Connection, Result};

fn main() {
    let manager = SqliteConnectionManager::file("notes.db");
    let pool = Pool::new(manager).unwrap();
    pool.get()
        .unwrap()
        .execute(
            "CREATE TABLE IF NOT EXISTS users (
                id          INTEGER PRIMARY KEY AUTOINCREMENT,
                username    TEXT NOT NULL UNIQUE,
                email       TEXT NOT NULL UNIQUE,
                password    TEXT NOT NULL,
                created_at  DATETIME DEFAULT CURRENT_TIMESTAMP 
            );
            
            CREATE TABLE notes (
                id          INTEGER PRIMARY KEY AUTOINCREMENT,
                user_id     INTEGER NOT NULL,
                title       TEXT NOT NULL,
                content     TEXT,
                created_at  DATETIME DEFAULT CURRENT_TIMESTAMP,
                updated_at  DATETIME DEFAULT CURRENT_TIMESTAMP,
                FOREIGN KEY(user_id) REFERENCES users(id) ON DELETE CASCADE
            );
            
            CREATE TRIGGER notes_updated_at_trigger
            AFTER UPDATE ON notes
            FOR EACH ROW
            BEGIN
                UPDATE notes SET updated_at = CURRENT_TIMESTAMP WHERE id = OLD.id;
            END;
            
            CREATE TABLE tags (
                id   INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL UNIQUE
            );
            
            CREATE TABLE note_tags (
                note_id INTEGER NOT NULL,
                tag_id  INTEGER NOT NULL,
                PRIMARY KEY (note_id, tag_id),
                FOREIGN KEY(note_id) REFERENCES notes(id) ON DELETE CASCADE,
                FOREIGN KEY(tag_id)  REFERENCES tags(id)  ON DELETE CASCADE
            );",
            params![])
        .unwrap();
    }