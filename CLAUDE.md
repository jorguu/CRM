# CLAUDE.md — CRM de escritorio para comercios pequeños (Suiza)

## Proyecto
- App de escritorio (Windows primero) tipo CRM *local-first* para comercios/servicios de 1–10 personas en Suiza. Datos en SQLite cifrado en el PC del cliente; sin nube propia para datos de clientes.
- Dueño: Jorge. Habla español con Claude, trabaja en alemán. Perfil: soporte IT, lee código pero no es desarrollador de profesión → código simple, legible, sin trucos.
- Brief original (referencia): `claude_md_desktop_crm_planning_architecture.md`. Si contradice este archivo o `docs/DECISIONS.md`, mandan estos.
- Docs: `docs/01-PLAN.md` (plan/arquitectura/sprints) · `docs/02-PREGUNTAS.md` (decisiones abiertas) · `docs/03-SEGURIDAD-CUMPLIMIENTO.md` · `docs/04-GUIA-NO-CODIGO.md` · `docs/DECISIONS.md` (decisiones tomadas = fuente de verdad).

## Cómo trabajar con Jorge
- Responde en español, breve y directo. Sin introducciones ni resúmenes de tareas simples.
- Cambia solo lo necesario; respeta estructura y convenciones existentes. No reescribas archivos enteros si basta una sección.
- Pide autorización antes de: borrar datos/archivos, cambiar esquema de BD ya aplicado, añadir dependencias, cambiar arquitectura o seguridad, tocar claves/firma/licencias, `git push`, cualquier gasto o cuenta externa.
- No afirmes haber ejecutado o probado algo que no ejecutaste. Indica qué corriste y el resultado.
- Texto de UI siempre vía i18n; comentarios en código solo si aportan.

## Estado actual: Sprint 0 hecho (esqueleto técnico) — siguiente: S1
1. Al empezar, lee `docs/DECISIONS.md` y `docs/02-PREGUNTAS.md`. Cómo compilar/testear: `README.md`.
2. S1 no arranca hasta completar M0 (validación con comercios, D-002). Preguntas 🟡/🔴 sin responder → pregúntalas a Jorge (máx. 4 por tanda con AskUserQuestion, opción recomendada primero) y anótalas en `DECISIONS.md`.
3. Antes de cada sprint, presenta su plan (pasos, riesgos, verificación) y espera aprobación.
4. Spikes desechables en `/spikes`.

## Stack (confirmado en S0, D-006)
- Tauri v2 + React + TypeScript + Tailwind + shadcn/ui.
- SQLite + SQLCipher gestionado desde Rust (`rusqlite` con `bundled-sqlcipher`), migraciones SQL versionadas. Spike S0 lo valida; alternativa en `docs/01-PLAN.md`.
- i18n: `de-CH` por defecto, `en`, luego `fr-CH`, `it-CH`. Alemán suizo: "ss" (nunca "ß"), formato CHF `1'234.50` (salida de `Intl` de-CH, D-008), fechas `dd.mm.yyyy`.
- Dinero en enteros (Rappen/céntimos) + moneda. Fechas UTC ISO-8601, mostrar Europe/Zurich. IDs UUIDv7.
- Instalador NSIS por usuario (sin admin), updater firmado, licencias firmadas Ed25519 verificables offline.

## Reglas de seguridad y privacidad (no negociables)
- Cero telemetría ni llamadas de red salvo: comprobación de licencia y de actualizaciones (documentadas, sin datos de clientes). Crash reports solo opt-in y sin datos personales.
- BD cifrada (SQLCipher). Clave nunca en el repo ni en logs. Backups cifrados con contraseña propia del usuario.
- SQL siempre parametrizado. Validar toda entrada que cruza IPC (frontend → Rust). Nada de `eval`, HTML sin sanitizar ni contenido remoto en la webview.
- Tauri: CSP estricta, capabilities mínimas por ventana, sin `shell`/fs abiertos; acceso a ficheros solo a rutas elegidas por el usuario y carpeta de datos de la app (evitar path traversal).
- Export CSV/Excel: neutralizar *formula injection* (celdas que empiezan por `=`, `+`, `-`, `@`).
- Logs sin datos personales. Sin secretos en el repo (`.env`, claves, certificados → `.gitignore`).
- Funciones DSG/DSGVO obligatorias en el producto: exportar todo de una persona, borrar/anonimizar persona (con adjuntos), campo de consentimiento de marketing, registro de auditoría. Ver `docs/03-SEGURIDAD-CUMPLIMIENTO.md`.
- Sin datos de salud ni categorías sensibles en el MVP.

## Licencias y propiedad intelectual
- Dependencias permitidas: MIT, Apache-2.0, BSD, ISC, MPL-2.0 (con cuidado), Unicode-3.0, Zlib, blessing, WTFPL (D-009). Política aplicada en `src-tauri/deny.toml` y `scripts/check-licenses.mjs`. Prohibidas sin aprobación de Jorge: GPL, AGPL, SSPL, "non-commercial".
- Antes de añadir una dependencia: licencia, mantenimiento y OK de Jorge. Mantener `THIRD_PARTY_LICENSES` generado (cargo-about / license-checker).
- No copiar código, textos, iconos, fuentes ni UI de otros productos. Marcas de terceros (WhatsApp, etc.) solo vía enlace estándar (`wa.me`), sin logos ajenos.

## Calidad
- Tests para lógica de negocio, migraciones y backup/restore. E2E de flujos críticos.
- Definition of Done: compila, tests pasan, i18n completo, sin warnings de `cargo audit`/`npm audit` graves, sin PII en logs, DECISIONS.md actualizado si cambió algo.
- Commits pequeños con mensaje claro. Nada de `push --force`.

## Fuera del MVP
Multiusuario/red, sincronización, facturación/QR-Rechnung, email integrado, móvil, IA, datos de salud.
