mod commands;
mod db;
mod error;

use std::sync::{Mutex, PoisonError};

use rusqlite::Connection;
use tauri::Manager;

use crate::error::{AppError, ErrorCode};

/// Estado compartido: la conexión a la BD, o el código de error si no se pudo abrir al arrancar.
pub struct AppState {
    db: Result<Mutex<Connection>, ErrorCode>,
}

impl AppState {
    pub fn with_db<T>(
        &self,
        action: impl FnOnce(&mut Connection) -> Result<T, AppError>,
    ) -> Result<T, AppError> {
        match &self.db {
            Ok(mutex) => {
                let mut conn = mutex.lock().unwrap_or_else(PoisonError::into_inner);
                action(&mut conn)
            }
            Err(code) => Err(AppError::Startup(*code)),
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let db = match open_database(app.handle()) {
                Ok(conn) => Ok(Mutex::new(conn)),
                Err(err) => {
                    // Solo el tipo de error: sin rutas ni datos personales.
                    eprintln!("No se pudo abrir la base de datos: {err}");
                    Err(err.code())
                }
            };
            app.manage(AppState { db });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::app_info,
            commands::set_language
        ])
        .run(tauri::generate_context!())
        .expect("error al arrancar la aplicación");
}

/// La BD vive en la carpeta de datos de la app (`%APPDATA%\<identifier>\`).
fn open_database(app: &tauri::AppHandle) -> Result<Connection, AppError> {
    let data_dir = app.path().app_data_dir().map_err(|_| AppError::DataDir)?;
    db::open_and_migrate(&data_dir, db::migrations::MIGRATIONS)
}
