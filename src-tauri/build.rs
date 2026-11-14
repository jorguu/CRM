fn main() {
    // Solo existen los comandos declarados aquí, y cada ventana necesita
    // permiso explícito para usarlos (ver capabilities/main.json).
    tauri_build::try_build(
        tauri_build::Attributes::new()
            .app_manifest(tauri_build::AppManifest::new().commands(&["app_info", "set_language"])),
    )
    .expect("error en tauri-build");
}
