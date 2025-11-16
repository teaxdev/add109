use r2d2_sqlite::SqliteConnectionManager;
use r2d2::{Pool, PooledConnection};
use rusqlite::{params, Connection, Result};

pub type DbPool = Pool<SqliteConnectionManager>;
pub type DbConn = PooledConnection<SqliteConnectionManager>;


pub fn init_db() -> DbPool {
    let manager = SqliteConnectionManager::file("notes.db");
    let pool = Pool::new(manager).expect("Failed to create DB pool");

    {
        let conn = pool.get().expect("Failed to get DB connection");
        run_migrations(&conn).expect("Failed to run migrations");
    }

    pool
}

fn run_migrations(conn: &rusqlite::Connection) -> Result<()> {
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS users (
            id          INTEGER PRIMARY KEY AUTOINCREMENT,
            username    TEXT NOT NULL UNIQUE,
            email       TEXT NOT NULL UNIQUE,
            password    TEXT NOT NULL,
            created_at  DATETIME DEFAULT CURRENT_TIMESTAMP 
        );
        
        CREATE TABLE IF NOT EXISTS notes (
            id          INTEGER PRIMARY KEY AUTOINCREMENT,
            user_id     INTEGER NOT NULL,
            title       TEXT NOT NULL,
            content     TEXT,
            created_at  DATETIME DEFAULT CURRENT_TIMESTAMP,
            updated_at  DATETIME DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY(user_id) REFERENCES users(id) ON DELETE CASCADE
        );
        
        CREATE TRIGGER IF NOT EXISTS notes_updated_at_trigger
        AFTER UPDATE ON notes
        FOR EACH ROW
        BEGIN
            UPDATE notes SET updated_at = CURRENT_TIMESTAMP WHERE id = OLD.id;
        END;
        
        CREATE TABLE IF NOT EXISTS tags (
            id   INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL UNIQUE
        );
        
        CREATE TABLE IF NOT EXISTS note_tags (
            note_id INTEGER NOT NULL,
            tag_id  INTEGER NOT NULL,
            PRIMARY KEY (note_id, tag_id),
            FOREIGN KEY(note_id) REFERENCES notes(id) ON DELETE CASCADE,
            FOREIGN KEY(tag_id)  REFERENCES tags(id)  ON DELETE CASCADE
        );",
    )?;

    Ok(())
}