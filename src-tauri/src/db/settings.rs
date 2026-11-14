//! Ajustes clave/valor (tabla `settings`). SQL siempre parametrizado.

use rusqlite::{Connection, OptionalExtension};

use crate::error::AppError;

pub fn get(conn: &Connection, key: &str) -> Result<Option<String>, AppError> {
    Ok(conn
        .query_row("SELECT value FROM settings WHERE key = ?1", [key], |row| {
            row.get(0)
        })
        .optional()?)
}

pub fn set(conn: &Connection, key: &str, value: &str) -> Result<(), AppError> {
    conn.execute(
        "INSERT INTO settings (key, value, updated_at)
         VALUES (?1, ?2, strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
         ON CONFLICT (key) DO UPDATE SET value = excluded.value, updated_at = excluded.updated_at",
        [key, value],
    )?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::migrations::{MIGRATIONS, apply_pending};

    #[test]
    fn set_then_get_and_overwrite() {
        let mut conn = Connection::open_in_memory().unwrap();
        apply_pending(&mut conn, MIGRATIONS).unwrap();

        assert_eq!(get(&conn, "language").unwrap(), None);
        set(&conn, "language", "en").unwrap();
        assert_eq!(get(&conn, "language").unwrap().as_deref(), Some("en"));
        set(&conn, "language", "de-CH").unwrap();
        assert_eq!(get(&conn, "language").unwrap().as_deref(), Some("de-CH"));
    }
}
