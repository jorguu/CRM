# 02 — Preguntas abiertas

🔴 bloquea el inicio del código · 🟡 antes del MVP · 🟢 puede esperar.
"Rec." = recomendación por defecto si no tienes preferencia. Responde en `DECISIONS.md`.

## A. Nicho y mercado
1. 🔴 ¿Qué son "locales pequeños"? (comercio minorista, hostelería, peluquería/estética, oficios/servicios, otro). *Rec.:* un solo vertical con presupuestos/trabajos (oficios/servicios); el Kanban encaja ahí. → ✅ D-001 (oficios/servicios)
2. 🔴 ¿Tienes 3–5 comercios concretos dispuestos a probarlo (y pagar)? *Rec.:* hablar con 5–10 antes del Sprint 1. → ✅ D-002 (1–2 contactos; M0 antes de S1)
3. 🔴 ¿Cuántos puestos/PC por comercio? *Rec.:* MVP monopuesto; multipuesto = v2. → ✅ D-003 (monopuesto)
4. 🟡 ¿Windows solo o también Mac? *Rec.:* Windows primero.
5. 🟡 ¿Idiomas? *Rec.:* de-CH + en; fr-CH/it-CH en v1.1; español solo si lo necesitas tú.
6. 🟡 ¿Has analizado bexio, Klara, Odoo, Pipedrive, HubSpot, Excel? ¿Cuál es tu diferenciador en una frase?
7. 🟢 ¿Facturas/QR-Rechnung/MWST dentro del CRM? *Rec.:* no en MVP; export CSV hacia bexio/Klara.
8. 🟡 ¿Excluimos sectores con datos de salud/sensibles (clínicas, terapeutas)? *Rec.:* sí.

## B. Legal y estructura
9. 🔴 ¿Tu contrato de trabajo permite actividad accesoria? ¿Hay cláusulas de competencia/propiedad intelectual? ¿Trabajarás solo fuera de horario y con equipo propio? *Rec.:* autorización escrita del empleador antes de empezar. (Ver `03`, §5) → ✅ D-004 (pedirá autorización; ⏳ pendiente)
10. 🟡 ¿Vendes como Einzelfirma/autónomo o vía GmbH (¿la de Davide u otra)? *Rec.:* decidir antes de la primera venta; GmbH limita la responsabilidad pero cuesta más.
11. 🟡 ¿Participa alguien más en el código/diseño? *Rec.:* cesión de derechos por escrito.
12. 🟡 ¿Vendes solo en Suiza o también UE (DSGVO, IVA UE)? *Rec.:* CH primero; diseñar ya compatible con DSGVO.
13. 🟡 ¿Nombre del producto/marca? *Rec.:* comprobar Swissreg/IGE, EUIPO, dominio `.ch` antes de invertir.

## C. Modelo comercial
14. 🟡 Precio y licencia: ¿pago único, anual o híbrido (único + plan de actualizaciones)? *Rec.:* híbrido.
15. 🟡 ¿Cómo cobras? Merchant of Record (Lemon Squeezy/Paddle: gestionan IVA), Stripe, Payrexx/TWINT, factura manual. *Rec.:* MoR para empezar; ofrecer factura (Rechnung) a pedido.
16. 🟡 Soporte: ¿idioma, horario, canal, tiempo de respuesta? ¿Quién lo hace con tus horas?
17. 🟢 ¿Nivel de protección anti-piratería? *Rec.:* mínimo (clave firmada, sin DRM agresivo).

## D. Técnico
18. 🟡 Tauri v2 (Rust) vs Electron. *Rec.:* Tauri; el Rust lo escribe Claude Code, pero tendrás que mantenerlo/revisarlo. ¿Aceptas? → ✅ D-006 (Tauri v2)
19. 🟡 Clave de cifrado: (a) DPAPI/Windows, auto-desbloqueo, o (b) contraseña del usuario al abrir (más seguro; si la pierde, pierde los datos). *Rec.:* (a) + clave de recuperación exportable; (b) opcional. → ✅ D-007 (DPAPI)
20. 🟡 Adjuntos: ¿cifrados desde el MVP? *Rec.:* no (BitLocker) y cifrar en v1.1.
21. 🟡 Backup: ¿contraseña propia del backup? *Rec.:* sí, obligatoria.
22. 🟡 Telemetría/crash reports. *Rec.:* ninguna en MVP; log local exportable manualmente.
23. 🟡 Servidor de licencias/updates: Keygen.sh / API del proveedor de pagos / API propia mínima; hosting en Suiza (p. ej. Infomaniak) o CDN. *Rec.:* servicio gestionado + endpoint estático de updates.
24. 🟢 ¿Tienes máquinas/VM Windows 10 y 11 limpias para probar instaladores? *Rec.:* sí, VMs.

## E. Recursos
25. 🔴 ¿Horas por semana reales y fecha objetivo? (el plan asume ~5 h/semana) → ✅ D-005 (~5 h/semana; ~nov. 2027) 
26. 🟡 ¿Presupuesto inicial? Partidas: certificado de firma (≈150–300 USD/año), dominio/hosting, revisión legal de documentos (pedir presupuesto), seguro de responsabilidad (pedir presupuesto), posible GmbH.

## Próxima decisión requerida
🔴 respondidas (D-001…D-005). Siguiente: aprobar el plan del Sprint 0; M0 (validación) sigue en paralelo.
