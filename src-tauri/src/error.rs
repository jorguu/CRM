//! Errores de la app. Al frontend solo le llega un código (`{ "code": "..." }`),
//! nunca rutas, SQL ni datos personales. El texto largo es solo para logs de desarrollo.

use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};

/// Códigos que ve el frontend (traducidos en `src/i18n/locales/*.json`, sección `errors`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ErrorCode {
    Database,
    Key,
    KeyMissing,
    Migration,
    SchemaTooNew,
    Io,
    InvalidInput,
}

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("error de base de datos: {0}")]
    Database(#[from] rusqlite::Error),
    #[error("error de E/S: {0}")]
    Io(#[from] std::io::Error),
    #[error("carpeta de datos de la app no disponible")]
    DataDir,
    #[error("la copia de seguridad ya existe")]
    BackupExists,
    #[error("DPAPI: {0}")]
    Dpapi(std::io::Error),
    #[error("clave: {0}")]
    Key(&'static str),
    #[error("la clave no corresponde a la base de datos")]
    KeyMismatch,
    #[error("falta el fichero de clave y la base de datos ya existe")]
    KeyMissing,
    #[error("la migración {version} ({name}) falló: {source}")]
    Migration {
        version: u32,
        name: &'static str,
        source: rusqlite::Error,
    },
    #[error("el esquema {found} es más nuevo que el soportado ({supported})")]
    SchemaTooNew { found: u32, supported: u32 },
    #[error("entrada no válida")]
    InvalidInput,
    #[error("la base de datos no se abrió al arrancar ({0:?})")]
    Startup(ErrorCode),
}

impl AppError {
    pub fn code(&self) -> ErrorCode {
        match self {
            AppError::Database(_) => ErrorCode::Database,
            AppError::Io(_) | AppError::DataDir | AppError::BackupExists => ErrorCode::Io,
            AppError::Dpapi(_) | AppError::Key(_) | AppError::KeyMismatch => ErrorCode::Key,
            AppError::KeyMissing => ErrorCode::KeyMissing,
            AppError::Migration { .. } => ErrorCode::Migration,
            AppError::SchemaTooNew { .. } => ErrorCode::SchemaTooNew,
            AppError::InvalidInput => ErrorCode::InvalidInput,
            AppError::Startup(code) => *code,
        }
    }
}

impl Serialize for AppError {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut state = serializer.serialize_struct("AppError", 1)?;
        state.serialize_field("code", &self.code())?;
        state.end()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn frontend_only_receives_the_code() {
        let err = AppError::Migration {
            version: 3,
            name: "secreto",
            source: rusqlite::Error::QueryReturnedNoRows,
        };
        let json = serde_json::to_string(&err).unwrap();
        assert_eq!(json, r#"{"code":"migration"}"#);
    }
}
