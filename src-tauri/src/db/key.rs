//! Clave de la BD: 32 bytes aleatorios guardados en `db.key`, protegidos con DPAPI (D-007).
//! La exportación de la clave de recuperación llega en S5.

use std::fs;
use std::path::Path;

use zeroize::Zeroizing;

use super::{DbKey, dpapi};
use crate::error::AppError;

pub const KEY_FILE: &str = "db.key";
const KEY_LEN: usize = 32;

/// Carga la clave de `data_dir`. Si no hay clave y tampoco BD, crea una nueva.
/// Nunca crea una clave nueva si la BD ya existe: la dejaría ilegible.
pub fn load_or_create(data_dir: &Path, db_exists: bool) -> Result<DbKey, AppError> {
    let key_path = data_dir.join(KEY_FILE);

    if key_path.exists() {
        let blob = fs::read(&key_path)?;
        let raw = Zeroizing::new(dpapi::unprotect(&blob)?);
        if raw.len() != KEY_LEN {
            return Err(AppError::Key("longitud de clave no válida"));
        }
        let mut key: DbKey = Zeroizing::new([0u8; KEY_LEN]);
        key.copy_from_slice(&raw);
        return Ok(key);
    }

    if db_exists {
        return Err(AppError::KeyMissing);
    }

    let mut key: DbKey = Zeroizing::new([0u8; KEY_LEN]);
    getrandom::fill(key.as_mut())
        .map_err(|_| AppError::Key("generador aleatorio no disponible"))?;
    let blob = dpapi::protect(key.as_ref())?;
    // Escritura atómica: primero a un temporal y luego se renombra.
    let tmp_path = data_dir.join(format!("{KEY_FILE}.tmp"));
    fs::write(&tmp_path, &blob)?;
    fs::rename(&tmp_path, &key_path)?;
    Ok(key)
}

#[cfg(all(test, windows))]
mod tests {
    use super::*;
    use crate::error::ErrorCode;

    #[test]
    fn creates_a_key_once_and_reloads_the_same_key() {
        let dir = tempfile::tempdir().unwrap();
        let first = load_or_create(dir.path(), false).unwrap();
        let second = load_or_create(dir.path(), true).unwrap();
        assert_eq!(*first, *second);

        let on_disk = fs::read(dir.path().join(KEY_FILE)).unwrap();
        assert!(!on_disk.windows(KEY_LEN).any(|w| w == first.as_ref()));
    }

    #[test]
    fn refuses_to_create_a_new_key_for_an_existing_database() {
        let dir = tempfile::tempdir().unwrap();
        let err = load_or_create(dir.path(), true).unwrap_err();
        assert_eq!(err.code(), ErrorCode::KeyMissing);
    }
}
