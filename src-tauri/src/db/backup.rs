//! Copias internas de la BD, cifradas con la misma clave (p. ej. antes de migrar).
//! Los backups del usuario con contraseña propia llegan en S5.

use std::fs;
use std::path::{Path, PathBuf};
use std::time::Duration;

use rusqlite::Connection;
use rusqlite::backup::Backup;

use super::{DbKey, open_encrypted};
use crate::error::AppError;

/// Copia antes de migrar: `backups/pre-migration-v{N}-{fecha UTC}.db`.
pub fn pre_migration(
    conn: &Connection,
    backup_dir: &Path,
    key: &DbKey,
    version: u32,
) -> Result<PathBuf, AppError> {
    fs::create_dir_all(backup_dir)?;
    let stamp: String = conn.query_row("SELECT strftime('%Y%m%dT%H%M%SZ', 'now')", [], |row| {
        row.get(0)
    })?;
    let dest = backup_dir.join(format!("pre-migration-v{version}-{stamp}.db"));
    copy_to(conn, &dest, key)?;
    Ok(dest)
}

/// Copia `conn` a `dest` (que no debe existir) con la API de backup de SQLite.
pub fn copy_to(conn: &Connection, dest: &Path, key: &DbKey) -> Result<(), AppError> {
    if dest.exists() {
        return Err(AppError::BackupExists);
    }
    let mut target = open_encrypted(dest, key)?;
    Backup::new(conn, &mut target)?.run_to_completion(100, Duration::ZERO, None)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::tests::{is_plain_sqlite, test_key};
    use crate::error::ErrorCode;

    #[test]
    fn copy_is_encrypted_and_readable_only_with_the_key() {
        let dir = tempfile::tempdir().unwrap();
        let key = test_key(1);
        let source = open_encrypted(&dir.path().join("source.db"), &key).unwrap();
        source
            .execute_batch("CREATE TABLE t (name TEXT); INSERT INTO t VALUES ('Zürich');")
            .unwrap();

        let dest = dir.path().join("copy.db");
        copy_to(&source, &dest, &key).unwrap();

        assert!(!is_plain_sqlite(&dest));
        let copy = open_encrypted(&dest, &key).unwrap();
        let name: String = copy
            .query_row("SELECT name FROM t", [], |r| r.get(0))
            .unwrap();
        assert_eq!(name, "Zürich");
        assert_eq!(
            open_encrypted(&dest, &test_key(2)).unwrap_err().code(),
            ErrorCode::Key
        );
    }

    #[test]
    fn never_overwrites_an_existing_file() {
        let dir = tempfile::tempdir().unwrap();
        let key = test_key(1);
        let source = open_encrypted(&dir.path().join("source.db"), &key).unwrap();
        let dest = dir.path().join("copy.db");
        fs::write(&dest, b"existente").unwrap();
        assert_eq!(
            copy_to(&source, &dest, &key).unwrap_err().code(),
            ErrorCode::Io
        );
        assert_eq!(fs::read(&dest).unwrap(), b"existente");
    }
}
