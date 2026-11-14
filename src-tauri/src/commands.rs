//! Comandos IPC (frontend → Rust). Toda entrada se valida aquí antes de tocar la BD.
//! Cada comando debe figurar también en `build.rs` y en `capabilities/main.json`.

use serde::Serialize;
use tauri::{AppHandle, State};

use crate::AppState;
use crate::db::{migrations, settings};
use crate::error::AppError;

pub const SUPPORTED_LANGUAGES: &[&str] = &["de-CH", "en"];
pub const DEFAULT_LANGUAGE: &str = "de-CH";
const LANGUAGE_SETTING: &str = "language";

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AppInfo {
    app_version: String,
    schema_version: u32,
    language: String,
}

#[tauri::command]
pub fn app_info(app: AppHandle, state: State<'_, AppState>) -> Result<AppInfo, AppError> {
    state.with_db(|conn| {
        let language = settings::get(conn, LANGUAGE_SETTING)?
            .filter(|value| is_supported_language(value))
            .unwrap_or_else(|| DEFAULT_LANGUAGE.to_string());
        Ok(AppInfo {
            app_version: app.package_info().version.to_string(),
            schema_version: migrations::current_version(conn)?,
            language,
        })
    })
}

#[tauri::command]
pub fn set_language(state: State<'_, AppState>, language: String) -> Result<(), AppError> {
    if !is_supported_language(&language) {
        return Err(AppError::InvalidInput);
    }
    state.with_db(|conn| settings::set(conn, LANGUAGE_SETTING, &language))
}

fn is_supported_language(value: &str) -> bool {
    SUPPORTED_LANGUAGES.contains(&value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_supported_languages_are_accepted() {
        assert!(is_supported_language("de-CH"));
        assert!(is_supported_language("en"));
        for invalid in [
            "",
            "fr-CH",
            "de",
            "DE-CH",
            "de-CH; DROP TABLE settings",
            "en ",
        ] {
            assert!(!is_supported_language(invalid), "{invalid:?}");
        }
    }
}
