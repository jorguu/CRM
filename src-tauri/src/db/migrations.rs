//! Migraciones SQL versionadas. La versión aplicada se guarda en `PRAGMA user_version`.

use rusqlite::Connection;

use crate::error::AppError;

pub struct Migration {
    pub version: u32,
    pub name: &'static str,
    pub sql: &'static str,
}

/// Lista oficial, en orden. Añadir siempre al final con la versión siguiente.
pub const MIGRATIONS: &[Migration] = &[Migration {
    version: 1,
    name: "init",
    sql: include_str!("../../migrations/0001_init.sql"),
}];

pub fn current_version(conn: &Connection) -> Result<u32, AppError> {
    Ok(conn.query_row("PRAGMA user_version", [], |row| row.get(0))?)
}

pub fn latest_version(migrations: &[Migration]) -> u32 {
    migrations.last().map_or(0, |m| m.version)
}

/// Aplica en orden las migraciones pendientes, cada una en su propia transacción.
/// Si una falla, se deshace entera y la versión no cambia. Devuelve cuántas aplicó.
pub fn apply_pending(conn: &mut Connection, migrations: &[Migration]) -> Result<usize, AppError> {
    let current = current_version(conn)?;
    let mut applied = 0;
    for migration in migrations.iter().filter(|m| m.version > current) {
        let tx = conn.transaction()?;
        tx.execute_batch(migration.sql)
            .map_err(|source| AppError::Migration {
                version: migration.version,
                name: migration.name,
                source,
            })?;
        tx.pragma_update(None, "user_version", migration.version)?;
        tx.commit()?;
        applied += 1;
    }
    Ok(applied)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::ErrorCode;

    fn table_exists(conn: &Connection, name: &str) -> bool {
        conn.query_row(
            "SELECT count(*) FROM sqlite_master WHERE type = 'table' AND name = ?1",
            [name],
            |row| row.get::<_, i64>(0),
        )
        .unwrap()
            == 1
    }

    #[test]
    fn versions_are_consecutive_from_1() {
        for (index, migration) in MIGRATIONS.iter().enumerate() {
            assert_eq!(migration.version, index as u32 + 1, "{}", migration.name);
        }
    }

    #[test]
    fn new_database_reaches_latest_version() {
        let mut conn = Connection::open_in_memory().unwrap();
        let applied = apply_pending(&mut conn, MIGRATIONS).unwrap();
        assert_eq!(applied, MIGRATIONS.len());
        assert_eq!(current_version(&conn).unwrap(), latest_version(MIGRATIONS));
        assert!(table_exists(&conn, "settings"));
    }

    #[test]
    fn applying_again_does_nothing() {
        let mut conn = Connection::open_in_memory().unwrap();
        apply_pending(&mut conn, MIGRATIONS).unwrap();
        assert_eq!(apply_pending(&mut conn, MIGRATIONS).unwrap(), 0);
    }

    #[test]
    fn failing_migration_is_rolled_back_completely() {
        let broken = [
            Migration {
                version: 1,
                ..MIGRATIONS[0]
            },
            Migration {
                version: 2,
                name: "rota",
                // La primera sentencia se ejecuta; la segunda falla.
                sql: "CREATE TABLE half_done (id INTEGER); ESTO NO ES SQL;",
            },
        ];
        let mut conn = Connection::open_in_memory().unwrap();
        let err = apply_pending(&mut conn, &broken).unwrap_err();

        assert_eq!(err.code(), ErrorCode::Migration);
        assert_eq!(current_version(&conn).unwrap(), 1);
        assert!(!table_exists(&conn, "half_done"));
        assert!(table_exists(&conn, "settings"));
    }
}
