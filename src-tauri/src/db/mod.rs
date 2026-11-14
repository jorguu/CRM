//! Base de datos local cifrada con SQLCipher (rusqlite + bundled-sqlcipher).

pub mod backup;
mod dpapi;
pub mod key;
pub mod migrations;
pub mod settings;

use std::fs;
use std::path::Path;

use rusqlite::Connection;
use zeroize::Zeroizing;

use crate::error::AppError;
use migrations::Migration;

pub const DB_FILE: &str = "data.db";
pub const BACKUP_DIR: &str = "backups";

/// Clave de 32 bytes que se borra de la memoria al soltarla.
pub type DbKey = Zeroizing<[u8; 32]>;

/// Abre (o crea) la BD de `data_dir`, hace una copia si hay migraciones pendientes y migra.
pub fn open_and_migrate(data_dir: &Path, migrations: &[Migration]) -> Result<Connection, AppError> {
    fs::create_dir_all(data_dir)?;
    let db_path = data_dir.join(DB_FILE);
    let key = key::load_or_create(data_dir, db_path.exists())?;
    let mut conn = open_encrypted(&db_path, &key)?;

    let current = migrations::current_version(&conn)?;
    let latest = migrations::latest_version(migrations);
    if current > latest {
        return Err(AppError::SchemaTooNew {
            found: current,
            supported: latest,
        });
    }
    if current > 0 && current < latest {
        backup::pre_migration(&conn, &data_dir.join(BACKUP_DIR), &key, current)?;
    }
    migrations::apply_pending(&mut conn, migrations)?;
    Ok(conn)
}

/// Abre un fichero SQLCipher con la clave. Falla si la clave no corresponde.
pub fn open_encrypted(path: &Path, key: &DbKey) -> Result<Connection, AppError> {
    let conn = Connection::open(path)?;
    // PRAGMA no admite parámetros. La clave es hex generado por la app, nunca texto del usuario.
    let pragma = Zeroizing::new(format!(
        "PRAGMA key = \"x'{}'\";",
        to_hex(key.as_ref()).as_str()
    ));
    conn.execute_batch(&pragma)?;
    // Leer la cabecera obliga a descifrar: con una clave incorrecta falla aquí.
    conn.query_row("SELECT count(*) FROM sqlite_master", [], |row| {
        row.get::<_, i64>(0)
    })
    .map_err(|err| match err.sqlite_error_code() {
        Some(rusqlite::ErrorCode::NotADatabase) => AppError::KeyMismatch,
        _ => AppError::Database(err),
    })?;
    conn.execute_batch("PRAGMA foreign_keys = ON;")?;
    Ok(conn)
}

fn to_hex(bytes: &[u8]) -> Zeroizing<String> {
    Zeroizing::new(bytes.iter().map(|b| format!("{b:02x}")).collect())
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::error::ErrorCode;

    pub fn test_key(byte: u8) -> DbKey {
        Zeroizing::new([byte; 32])
    }

    pub fn is_plain_sqlite(path: &Path) -> bool {
        let bytes = fs::read(path).unwrap();
        bytes.starts_with(b"SQLite format 3\0")
    }

    #[test]
    fn wrong_key_is_rejected_and_file_is_not_plain_sqlite() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("t.db");
        open_encrypted(&path, &test_key(1))
            .unwrap()
            .execute_batch("CREATE TABLE t (x TEXT);")
            .unwrap();

        assert!(!is_plain_sqlite(&path));
        let err = open_encrypted(&path, &test_key(2)).unwrap_err();
        assert_eq!(err.code(), ErrorCode::Key);
    }

    // Los tests de `open_and_migrate` usan DPAPI: solo en Windows.
    #[cfg(windows)]
    mod with_dpapi {
        use super::*;
        use crate::db::migrations::{MIGRATIONS, current_version, latest_version};

        const EXTRA: Migration = Migration {
            version: 2,
            name: "test_extra",
            sql: "CREATE TABLE extra (id INTEGER PRIMARY KEY);",
        };

        fn backups(dir: &Path) -> Vec<String> {
            match fs::read_dir(dir.join(BACKUP_DIR)) {
                Ok(entries) => entries
                    .map(|e| e.unwrap().file_name().to_string_lossy().into_owned())
                    .collect(),
                Err(_) => Vec::new(),
            }
        }

        #[test]
        fn new_database_is_created_encrypted_at_latest_version() {
            let dir = tempfile::tempdir().unwrap();
            let conn = open_and_migrate(dir.path(), MIGRATIONS).unwrap();

            assert_eq!(current_version(&conn).unwrap(), latest_version(MIGRATIONS));
            assert!(dir.path().join(key::KEY_FILE).exists());
            assert!(!is_plain_sqlite(&dir.path().join(DB_FILE)));
            assert!(backups(dir.path()).is_empty());
        }

        #[test]
        fn reopening_does_not_migrate_or_back_up_again() {
            let dir = tempfile::tempdir().unwrap();
            drop(open_and_migrate(dir.path(), MIGRATIONS).unwrap());
            let conn = open_and_migrate(dir.path(), MIGRATIONS).unwrap();

            assert_eq!(current_version(&conn).unwrap(), 1);
            assert!(backups(dir.path()).is_empty());
        }

        #[test]
        fn pending_migration_makes_a_readable_backup_first() {
            let dir = tempfile::tempdir().unwrap();
            drop(open_and_migrate(dir.path(), MIGRATIONS).unwrap());

            let v2 = [Migration { ..MIGRATIONS[0] }, EXTRA];
            let conn = open_and_migrate(dir.path(), &v2).unwrap();
            assert_eq!(current_version(&conn).unwrap(), 2);

            let files = backups(dir.path());
            assert_eq!(files.len(), 1);
            assert!(files[0].starts_with("pre-migration-v1-"), "{}", files[0]);

            let key = key::load_or_create(dir.path(), true).unwrap();
            let copy = open_encrypted(&dir.path().join(BACKUP_DIR).join(&files[0]), &key).unwrap();
            assert_eq!(current_version(&copy).unwrap(), 1);
        }

        #[test]
        fn database_from_a_newer_app_is_not_touched() {
            let dir = tempfile::tempdir().unwrap();
            let v2 = [Migration { ..MIGRATIONS[0] }, EXTRA];
            drop(open_and_migrate(dir.path(), &v2).unwrap());

            let err = open_and_migrate(dir.path(), MIGRATIONS).unwrap_err();
            assert_eq!(err.code(), ErrorCode::SchemaTooNew);
        }

        #[test]
        fn missing_key_file_with_existing_database_is_an_error() {
            let dir = tempfile::tempdir().unwrap();
            drop(open_and_migrate(dir.path(), MIGRATIONS).unwrap());
            fs::remove_file(dir.path().join(key::KEY_FILE)).unwrap();

            let err = open_and_migrate(dir.path(), MIGRATIONS).unwrap_err();
            assert_eq!(err.code(), ErrorCode::KeyMissing);
        }
    }
}
