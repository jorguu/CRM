# 01 — Plan, arquitectura y sprints

> Estado: borrador para validar con Jorge. Las decisiones firmes se pasan a `DECISIONS.md`.

## 1. Posicionamiento
CRM de escritorio, simple, offline, con datos en el PC del comercio, pensado para Suiza (de-CH primero, CHF, convenciones locales). Nicho MVP: oficios/servicios (D-001). Competencia a revisar antes de construir: bexio, Klara, Abacus, Odoo, Pipedrive, HubSpot (gratis), Excel/WhatsApp.
**Diferenciador a validar:** "sus datos se quedan en su PC" + cero curva de aprendizaje + pago único.

## 2. Observaciones críticas sobre el brief original
1. **"Sin nube = sin problemas DSGVO/DPA" es una simplificación.** El comercio sigue siendo *Verantwortlicher* (debe informar, atender derechos, asegurar datos). Tú sigues tratando datos de licencia/updates/soporte; y si ves su BD en soporte remoto, eres *Auftragsbearbeiter* (necesitas contrato). Backups en OneDrive/Google Drive = datos en nube de terceros (mitigar con backup cifrado por contraseña propia).
2. **1–10 empleados ⇒ probablemente varios PCs.** SQLite en carpeta de red no es fiable. Decidido (D-003): MVP monopuesto; multipuesto en v2 (servidor local o sync).
3. **Idiomas:** el brief lista EN/ES/FR/DE. Para Suiza: `de-CH` (ss, sin ß), `fr-CH`, `it-CH`, `en`. Español no aporta al mercado.
4. **Kanban de pipeline** encaja en servicios/oficios con presupuestos; en retail (clientes de mostrador) aporta poco. Nicho elegido: oficios/servicios (D-001) → el Kanban se mantiene en el MVP.
5. **Firma de código:** EV ya no elimina SmartScreen al instante; Azure Artifact Signing no está disponible para organizaciones/personas en Suiza (solo EE.UU., Canadá, UE, Reino Unido) → certificado OV de una CA (≈150–300 USD/año). La reputación se acumula con descargas.
6. **Drizzle/Prisma + SQLCipher en Tauri** es incómodo. Propuesta: BD gestionada en Rust (`rusqlite` + `bundled-sqlcipher`) y el frontend llama a comandos tipados. Validar en Spike S0.
7. **Licencia "lifetime" offline** no es a prueba de piratería. Aceptable: licencia firmada (Ed25519) + trial 14 días; sin DRM agresivo.
8. **Adjuntos locales** no quedan cifrados por SQLCipher. MVP: carpeta de la app + recomendar BitLocker; v1.1: cifrado de adjuntos.
9. **Salud/clínicas:** datos "besonders schützenswert" → requisitos mucho más duros. Excluir del MVP.
10. **WhatsApp:** usar enlace `https://wa.me/<número>`; no usar su logo.

## 3. Decisiones recomendadas (por defecto)
| Tema | Recomendación | Estado |
|------|---------------|--------|
| Nicho | Oficios/servicios (Handwerker, instaladores, reparaciones, agencias pequeñas) | ✅ D-001 |
| Plataforma | Windows 10/11 primero | 🟡 |
| Usuarios | 1 PC / 1 puesto en MVP; multipuesto v2 | ✅ D-003 |
| Framework | Tauri v2 (Rust mínimo; lógica de UI en TS) | ✅ D-006 |
| BD | SQLite + SQLCipher, migraciones SQL versionadas | 🟡 spike |
| Clave BD | Almacenada con DPAPI (auto-desbloqueo) + clave de recuperación exportable | ✅ D-007 |
| Idiomas | de-CH, en → fr-CH, it-CH | 🟡 |
| Licencia | Trial 14 d + clave firmada; compra vía Merchant of Record | 🟡 |
| Updates | `tauri-plugin-updater` con clave propia, endpoint estático | 🟡 |
| Telemetría | Ninguna | ✅ |

## 4. Arquitectura
```
[ WebView (React/TS) ]  ←IPC tipado→  [ Núcleo Rust ]  →  SQLCipher (.db en %APPDATA%\<App>\)
                                          │               Adjuntos (carpeta de app / elegida)
                                          ├ Backups cifrados → carpeta elegida por usuario
                                          ├ Notificaciones nativas (recordatorios)
                                          └ Red (solo): licencia + updates (HTTPS, firmados)
```
- Toda la lógica que toca datos vive en Rust; el frontend no accede a ficheros ni SQL.
- Migraciones: automáticas al arrancar, con backup previo y rollback si fallan.
- Instalación por usuario (sin admin). WebView2 incluido en Win11; bootstrapper para Win10.

## 5. Esquema de datos (resumen SQLite)
Convenciones: `id` UUIDv7 · fechas UTC ISO · dinero en entero + `currency` · `created_at/updated_at` · `deleted_at` (papelera) y borrado/anonimizado real bajo demanda.

- **companies**: name, uid (CHE-xxx.xxx.xxx), mwst_number, address (street, zip, city, canton, country), website, notes
- **contacts**: company_id, salutation, first_name, last_name, **language (de/fr/it/en)**, **formality (Sie/Du)**, email, phone, mobile (E.164, +41…), address, source, **marketing_consent (bool, fecha, origen)**, notes, anonymized_at
- **pipeline_stages**: name, position, kind (open/won/lost)
- **deals**: contact_id/company_id, stage_id, title, amount, currency, expected_close, status
- **activities**: contact_id/deal_id, type (call/meeting/note/mail), occurred_at, text
- **tasks**: contact_id/deal_id, title, due_at, done_at, remind_at
- **tags** + **entity_tags** (entity_type, entity_id, tag_id)
- **attachments**: entity_type/entity_id, filename, relative_path, sha256, size, legal_retention (bool)
- **settings**: key/value (idioma, moneda por defecto, carpeta backup, frecuencia backup)
- **license**: key, tipo (trial/lifetime/annual), activada_at, expira_at, firma, device_hash
- **audit_log**: ts, action (export, delete, anonymize, import, backup, restore, login), entity, resumen sin PII
- (migraciones: tabla gestionada por la herramienta de migraciones)

**Backup y recuperación:** backup automático (diario/al cerrar) a carpeta elegida → fichero cifrado con contraseña del usuario, rotación (p. ej. últimos 14), verificación de integridad, asistente "Restaurar" probado en tests. Aviso claro: sin contraseña de backup/clave de recuperación no hay recuperación.

## 6. UX (borrador)
Barra lateral: **Hoy** · Contactos · Empresas · Pipeline · Tareas · Ajustes. Creación rápida `Ctrl+N`, búsqueda global `Ctrl+K`. Onboarding ≤3 min: idioma → moneda → plantilla del sector → primer contacto → carpeta de backup + contraseña.

## 7. Roadmap (estimación orientativa, sin base empírica; con Claude Code + revisión tuya)
| Sprint | Contenido | Horas aprox. |
|--------|-----------|--------------|
| M0 | Validación: 5–10 entrevistas con oficios/servicios (hay 1–2 contactos), competencia. Decisiones 🔴 ✅. En paralelo a S0; **S1 no arranca sin M0 completo** (D-002) | 10–20 |
| S0 ✅ | Esqueleto Tauri, SQLCipher + migraciones (spike), i18n, CI, lint, estructura (hecho 2026-11-14; ver `README.md`) | 15–25 |
| S1 | Contactos/empresas CRUD, búsqueda, etiquetas, papelera | 20–30 |
| S2 | Pipeline Kanban (drag & drop, totales) | 15–20 |
| S3 | Tareas, recordatorios, notificaciones, vista "Hoy" | 15–25 |
| S4 | Timeline de actividades + adjuntos | 15–25 |
| S5 | Import/Export CSV/Excel, backup/restore, export/borrado por persona (DSG) | 20–30 |
| S6 | Licencias, trial, updater, instalador NSIS, firma, CI de release | 25–40 |
| S7 | Hardening de seguridad, beta con 3 comercios, docs de usuario, legal | 20–30 |
| S8 | Lanzamiento (web, pagos, soporte) | 15–25 |

Total ≈ 170–270 h. A 5 h/semana ≈ 9–13 meses → objetivo de lanzamiento ~nov. 2027 (D-005). Para acortar, recortar primero adjuntos; el Kanban solo en último caso (es central en oficios/servicios).

## 8. Riesgos principales
Nicho sin demanda (→ M0) · pérdida de datos/clave (→ backups + recuperación) · SmartScreen/antivirus (→ firma + envío a Microsoft Defender) · conflicto con empleador por tiempo/IP (→ `03`, §5; autorización escrita ⏳ pendiente, D-004) · mantenimiento/soporte con poco tiempo (→ alcance pequeño, docs, sin multiusuario) · responsabilidad por pérdida de datos (→ EULA con límites, seguro, backups).
