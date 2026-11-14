# CRM de escritorio (Suiza) — guía de desarrollo

App de escritorio *local-first*: Tauri v2 + React/TypeScript + SQLite cifrado (SQLCipher).
Contexto, reglas y decisiones: [`CLAUDE.md`](CLAUDE.md) y [`docs/`](docs/).

## Requisitos (Windows)

- Node 26
- Rust stable (toolchain MSVC) con clippy y rustfmt
- Visual Studio Build Tools con "Desarrollo para el escritorio con C++"
- Strawberry Perl (lo usa la compilación de OpenSSL/SQLCipher)

## Comandos

| Qué | Comando |
|-----|---------|
| Instalar dependencias | `npm install` |
| Ejecutar en desarrollo | `npm run tauri dev` |
| Compilar sin instalador | `npm run tauri build -- --no-bundle` |
| Chequeos frontend (tipos, lint, tests, licencias) | `npm run check` |
| Chequeos Rust (fmt, clippy, tests) | `npm run check:rust` |
| Política de dependencias Rust | `cargo deny --manifest-path src-tauri/Cargo.toml check` |
| Formatear código | `npm run format` y `cargo fmt --manifest-path src-tauri/Cargo.toml` |

La primera compilación de Rust tarda unos 10 minutos (compila OpenSSL y SQLCipher).

Si PowerShell dice que `npm.ps1` no se puede cargar ("running scripts is disabled"), usa `npm.cmd` en lugar de `npm` (p. ej. `npm.cmd run tauri dev`).

## Estructura

- `src/` — frontend React. Textos en `src/i18n/locales/` (de-CH por defecto, en). Formatos suizos en `src/lib/format.ts`.
- `src-tauri/src/` — núcleo Rust: `db/` (SQLCipher, clave con DPAPI, migraciones, copias), `commands.rs` (comandos IPC, con validación de entrada).
- `src-tauri/migrations/` — migraciones SQL numeradas. Nunca editar una ya publicada: añadir una nueva.
- `src-tauri/capabilities/` — permisos de la ventana. Un comando nuevo debe añadirse también en `src-tauri/build.rs`.
- `scripts/` — herramientas del repo. `spikes/` — pruebas desechables.

## Datos locales

- Base de datos cifrada: `%LOCALAPPDATA%\ch.kmucrm.desktop\data.db` (Local, no Roaming: D-011).
- Clave: `db.key` en la misma carpeta, protegida con DPAPI (solo la puede leer tu usuario de Windows). **Si se borra `db.key`, la BD queda ilegible.** La clave de recuperación exportable llega en S5.
- Antes de cada migración se guarda una copia en `backups\pre-migration-v<N>-<fecha>.db`.

## Provisional

El nombre `CRM`, el identificador `ch.kmucrm.desktop` y el icono son provisionales: hay que fijarlos antes de la primera beta (cambiar el identificador mueve la carpeta de datos).
