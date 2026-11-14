# 03 — Seguridad, protección de datos y propiedad intelectual

> Resumen técnico-práctico, **no es asesoramiento legal**. Verificar artículos y plazos con un abogado/DPO suizo antes del lanzamiento.

## 1. Marco aplicable
- **nDSG/revDSG** (en vigor desde 1.9.2023): aplica a comercios en Suiza que tratan datos de personas. Sanciones penales de hasta CHF 250'000, dirigidas a las personas responsables (no solo a la empresa).
- **DSGVO** (UE): aplica además si ofreces/vendes a personas en la UE o las monitorizas. Diseño compatible desde el inicio.
- **Roles:**
  - El **comercio** = *Verantwortlicher* de los datos de sus clientes.
  - **Tú** = Verantwortlicher de tus propios datos (clientes que compran la licencia, soporte, web, pagos).
  - Tú = *Auftragsbearbeiter* **solo si** accedes a su BD (soporte remoto, backups en tus servidores…) → contrato de encargo (Art. 9 nDSG / Art. 28 DSGVO). Regla de producto: **nunca tener acceso a datos de clientes**; si hay soporte con pantalla compartida, hacerlo bajo contrato y con consentimiento.

## 2. Obligaciones del comercio y cómo las cubre la app
| Obligación (nDSG) | Funcionalidad / material |
|---|---|
| Información al interesado (Art. 19) | Texto-plantilla de Datenschutzerklärung + campo "origen del dato" |
| Derecho de acceso (Art. 25; respuesta normalmente ≤30 días) | Botón "Exportar todo de esta persona" (PDF/JSON) |
| Rectificación/borrado | "Borrar o anonimizar persona" incl. adjuntos; aviso sobre backups antiguos |
| Minimización/finalidad (Art. 6) | Campos opcionales, sin recopilar de más; sin datos sensibles en MVP |
| Privacy by design/default (Art. 7) | Cifrado por defecto, sin telemetría, permisos mínimos |
| Seguridad de datos (Art. 8 + DSV) | Ver §4 |
| Aviso de brechas al EDÖB (Art. 24) cuando hay riesgo elevado | Plantilla de plan de incidentes; registro de incidentes ≥2 años |
| Registro de tratamientos (Art. 12; excepciones <250 empleados salvo riesgo alto) | Plantilla ligera descargable |
| Publicidad por mail/SMS (UWG Art. 3 Abs. 1 lit. o) | Campo de consentimiento + posibilidad de baja |
| Conservación contable 10 años (Art. 958f OR) | Marca `legal_retention` en documentos; avisar del conflicto "borrar vs. conservar" |
| Transferencias al extranjero (Art. 16) | Backup en OneDrive/Google = tercero; recomendar backup cifrado con contraseña propia |

## 3. Tus obligaciones como proveedor
- Datenschutzerklärung web y de la app, **Impressum** (identificación en e-commerce, UWG Art. 3 Abs. 1 lit. s), AGB/EULA, política de reembolso.
- Lista de subencargados (pagos, hosting, email, CDN) y dónde está cada dato.
- Servidor de licencias/updates: guardar lo mínimo (email, clave, hash de equipo), logs con retención corta, hosting en CH/UE.
- Plan de respuesta a incidentes y contacto de seguridad (`security.txt`).
- Contrato de encargo (AVV) tipo para soporte remoto.

## 4. Requisitos de seguridad (el código debe cumplirlos)
**Datos**
- SQLCipher en reposo; clave protegida (DPAPI) + recuperación exportable; derivación con Argon2 si hay contraseña.
- Backups cifrados con contraseña del usuario; verificación de integridad; prueba de restauración automatizada.
- Borrado seguro/anonimizado real (no solo `deleted_at`).
- Adjuntos: fuera de BD → recomendar BitLocker (MVP), cifrar en v1.1.

**Aplicación (Tauri)**
- CSP estricta, sin contenido remoto, capabilities mínimas por ventana, sin APIs `shell`/`fs` abiertas.
- Validación de todos los comandos IPC; SQL parametrizado; límites de tamaño.
- Rutas: canonicalizar y restringir a carpetas permitidas (anti path traversal); nunca ejecutar adjuntos, solo abrirlos con la app del sistema por acción explícita.
- Export CSV/Excel: neutralizar *formula injection*. Import: validar codificación (UTF-8/Windows-1252) y tamaño.
- Bloqueo por inactividad (opcional).

**Cadena de suministro y release**
- `cargo audit` / `cargo deny` / `npm audit` en CI; Dependabot; versiones fijadas (`Cargo.lock`, `package-lock.json`).
- Builds en CI limpio; firma de código del instalador; **updater con clave propia** (la clave privada se guarda offline y con copia: si se pierde, no puedes actualizar a nadie).
- Licencias firmadas Ed25519 (clave privada fuera del repo y de las máquinas de desarrollo).
- SBOM y `THIRD_PARTY_LICENSES` en cada release.
- Repo privado, 2FA, protección de rama, secretos solo en el gestor de secretos del CI.

**Logs y auditoría**
- Sin PII en logs. `audit_log` local para exportes, borrados, restauraciones.

## 5. Derechos de autor, licencias y empleador
**Titularidad**
- El software está protegido por derecho de autor (URG Art. 2 Abs. 3) sin registro. Autor = persona física que lo crea.
- ⚠ **Empleador:** si creas software *en ejercicio de tus tareas y obligaciones contractuales*, el empleador es el único titular de los derechos de explotación (URG Art. 17). Además: deber de fidelidad y prohibición de competencia durante la relación laboral (OR Art. 321a). Un CRM no es tu trabajo de soporte IT, pero **trabajar en horario/equipo/información del empleador o vender a sus clientes es riesgo**. → Autorización escrita, equipo y cuentas separadas, cero material del empleador.
- **Colaboradores** (Davide, diseñadores, freelancers): contrato de cesión de derechos por escrito.
- **Código generado con IA:** según los términos de Anthropic, el output es tuyo, pero la protección por derecho de autor de contenido generado por IA es jurídicamente incierta; documenta tu aportación creativa (diseño, arquitectura, revisión) y no dependas de "copyright" como único foso: añade marca, servicio y velocidad.

**Dependencias y OSS**
- Permitidas: MIT, Apache-2.0, BSD, ISC (Tauri, React, shadcn/ui, Tailwind suelen estar en estas). MPL-2.0 con cuidado. SQLCipher Community: revisar licencia y atribución exigida.
- Evitar GPL/AGPL/SSPL/no-comercial (obligarían a abrir tu código o prohibirían uso comercial).
- Herramientas: `cargo-about`/`cargo-deny` y `license-checker`; incluir avisos de licencia en la app (pantalla "Acerca de").
- Fuentes, iconos e imágenes: comprobar licencia (p. ej. Lucide ISC, fuentes OFL). Nada de capturas/logos de terceros.

**Marca y textos**
- Registrar/buscar marca (Swissreg/IGE; EUIPO si vendes en UE). Dominio `.ch`.
- No copiar textos, flujos ni diseño de competidores. Marcas ajenas (WhatsApp, Microsoft) solo en texto descriptivo.

**Tu licencia al cliente (EULA)**
- Licencia de uso no exclusiva e intransferible, prohibido ingeniería inversa (dentro de lo permitido por ley), sin código fuente, límite de responsabilidad (Art. 100 OR: no se puede excluir dolo ni negligencia grave), derecho suizo y fuero, política de actualizaciones, garantías limitadas, backup responsabilidad del cliente (la app lo facilita).

## 6. Checklist previa al lanzamiento
- [ ] Autorización escrita del empleador (si aplica)
- [ ] Estructura legal y alta como autónomo/empresa; seguro de responsabilidad
- [ ] EULA, AGB, Datenschutzerklärung, Impressum, AVV revisados por abogado
- [ ] Revisión de licencias de dependencias sin GPL/AGPL
- [ ] Pruebas: restauración de backup, migración de versión, instalación limpia, actualización
- [ ] Auditoría de seguridad básica (checklist §4) + `cargo audit`/`npm audit` limpios
- [ ] Función export/borrado por persona probada
- [ ] Plan de incidentes y contacto de seguridad publicado
- [ ] Firma de código y updater verificados en máquina limpia
