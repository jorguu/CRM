# 04 — Todo lo que hay que hacer aparte del código

> Orden sugerido. Costes y plazos son orientativos; verifica antes de pagar.

## 0. Validar antes de construir
- 5–10 conversaciones con dueños de comercios del nicho elegido: ¿cómo gestionan clientes hoy?, ¿qué les duele?, ¿cuánto pagarían?
- Meta: 2–3 que digan "lo pruebo" y 1 que acepte pagar (o preventa). Si nadie, cambia nicho/alcance.
- Mira bexio, Klara, Odoo, Pipedrive, HubSpot: ¿qué falta que tú resuelves?

## 1. Estructura legal y fiscal
- **Empleador:** permiso escrito para actividad accesoria y aclarar propiedad intelectual (ver `03` §5).
- **Forma jurídica:**
  - *Einzelfirma* (autónomo): simple y barata; responsabilidad personal ilimitada. Inscripción en el Handelsregister obligatoria a partir de CHF 100'000 de facturación.
  - *GmbH*: capital mínimo CHF 20'000; responsabilidad limitada; más burocracia y contabilidad.
- **Alta AHV como independiente** en la Ausgleichskasse; declarar ingresos en la declaración de impuestos.
- **MWST:** obligatoria a partir de CHF 100'000 de facturación; antes, voluntaria.
- Cuenta bancaria separada, contabilidad simple (bexio/Klara o un Treuhänder), conservar documentos 10 años.
- **Seguro:** responsabilidad civil de empresa / IT (pedir presupuesto).

## 2. Marca, dominio y presencia
- Buscar nombre en Swissreg (IGE) y EUIPO; comprobar dominio `.ch`/`.com`; reservar email (`info@`, `support@`, `security@`).
- Web de aterrizaje en alemán (luego fr/it): propuesta de valor, precios, descarga, FAQ, Impressum, Datenschutz, AGB.

## 3. Documentos legales (revisión de abogado)
EULA/licencia · AGB · Datenschutzerklärung (web + app) · Impressum · política de reembolso · AVV para soporte remoto · lista de subencargados · plantillas para el cliente (registro de tratamientos, plan de incidentes, texto informativo a sus clientes). Claude Code puede generar borradores en el Sprint 7; abogado revisa.

## 4. "Base de datos": qué es y qué no hay que montar
- **No hay servidor de BD.** Cada cliente tiene un fichero SQLite cifrado en su PC (`%LOCALAPPDATA%\<App>\data.db`). Cero coste de hosting por cliente, cero tus responsabilidades sobre sus datos.
- Cosas a diseñar: esquema + **migraciones** (cada versión sabe actualizar la BD anterior, con backup previo), **backups** (cifrados, carpeta elegida, rotación), **restauración** (probada), **clave** (dónde se guarda, cómo se recupera).
- Único "servidor" tuyo: **licencias y actualizaciones** (+ web y pagos). Datos mínimos: email del comprador, clave, hash de equipo, versión. Opciones: servicio de licencias (Keygen.sh o el del proveedor de pagos) + fichero estático de updates (`latest.json` + instalador firmado) en CDN/hosting suizo/UE.

## 5. Seguridad operativa (tú, no la app)
- Gestor de contraseñas + **2FA en todo**: GitHub, dominio/registrar, hosting, pagos, email, proveedor de firma.
- **Clave privada del updater y de licencias:** generarla offline, copia cifrada en 2 sitios. Perderla = no poder actualizar/emitir licencias; filtrarla = cualquiera puede suplantarte.
- Repo privado, protección de rama, secretos solo en el CI, `security.txt`, plan si hay incidente (qué haces en las primeras 24 h, a quién avisas, aviso al EDÖB si hay riesgo elevado).
- Equipo de desarrollo separado del de tu empleador.

## 6. Build, firma y distribución del `.exe`
- **CI** (GitHub Actions, `windows-latest`): compila, testea, audita, empaqueta (Tauri → instalador NSIS), firma y publica.
- **Firma de código:** Azure Artifact Signing (≈10 USD/mes) **no está disponible para Suiza** (solo EE.UU., Canadá, UE y Reino Unido). Alternativa: certificado **OV** de una CA (DigiCert, Sectigo, SSL.com…, ≈150–300 USD/año); clave en token/HSM o servicio de firma en la nube de la CA; verificación de identidad/organización de varios días. EV ya no da confianza instantánea en SmartScreen: **la reputación se acumula con las descargas**; los primeros usuarios pueden ver avisos.
- Falsos positivos de antivirus: enviar el instalador a Microsoft Defender (portal de envío de malware/falsos positivos) y a los principales antivirus.
- Publicar **SHA-256** del instalador en la web; instalación por usuario sin admin.
- Alternativas opcionales: winget, Microsoft Store (otro proceso y cuota).
- Versionado SemVer + changelog en alemán.

## 7. Pagos y licencias
- **Merchant of Record** (Lemon Squeezy, Paddle): venden en tu nombre, gestionan IVA internacional y facturas; comisión por venta. Alternativa: Stripe/Payrexx (TWINT) + facturas propias.
- Los comercios suizos suelen preferir **factura a 30 días** y TWINT: ofrecer al menos factura bajo petición.
- Flujo: compra → email con clave → activación (online u offline por fichero) → trial 14 días → clave firmada Ed25519 verificada localmente.
- Política de reembolso y renovación clara; en CH no hay derecho de desistimiento general en compras online, pero para clientes UE sí puede aplicar → política generosa de 14–30 días.

## 8. Soporte y operación
- Canal único (email), plantillas de respuesta, FAQ y vídeos cortos en alemán.
- Reglas de soporte remoto: solo con consentimiento y bajo AVV; preferir logs exportables sin PII y reproducir en tu máquina.
- Política de actualizaciones: parches de seguridad siempre; versiones mayores según licencia.
- Tiempo de respuesta realista (tienes pocas horas/semana): publicarlo.

## 9. Pruebas y calidad (no solo unitarias)
- VMs Windows 10 y 11 limpias; teclado y regional `de-CH` (formato `1'234.50`, `dd.mm.yyyy`, umlauts, tildes francesas).
- Pruebas manuales: instalación limpia, actualización desde versión anterior, restauración de backup, borrar persona, import desde Excel real del comercio.
- **Beta con 3 comercios** (acuerdo de beta simple), observarlos usándolo sin ayudar.

## 10. Marketing y ventas en Suiza
- Mensaje: "Ihre Kundendaten bleiben auf Ihrem PC". Prueba gratuita 14 días, demo corta, idioma local.
- Canales: Gewerbeverbände/asociaciones locales, LinkedIn/Facebook de comercios, **Treuhänder/contables como prescriptores**, Google Ads locales, boca a boca de los beta testers.

## 11. Costes orientativos de arranque (verificar)
| Partida | Orden de magnitud |
|---|---|
| Certificado de firma OV | ≈150–300 USD/año |
| Dominio + email | decenas de CHF/año |
| Hosting/CDN updates | CHF 0–20/mes |
| Licencias/pagos | comisión por venta (MoR) |
| Revisión legal de documentos | pedir presupuesto |
| Seguro RC | pedir presupuesto |
| GmbH (si aplica) | capital CHF 20'000 + gastos de constitución |

## 12. Línea de tiempo
**Antes de codificar:** validar mercado, permiso del empleador, decisiones 🔴.
**Durante:** marca/dominio, cuenta de pagos, certificado de firma (empezar el trámite pronto), borradores legales, web.
**Antes de lanzar:** abogado, seguro, beta, checklist de `03` §6.
**Después:** métricas básicas (ventas, tickets), parches de seguridad, mejoras v1.1 (adjuntos cifrados, fr/it) y evaluar multiusuario.
