-- 0001: ajustes de la app (clave/valor). Las tablas de negocio llegan en S1.
-- Regla: una migración publicada no se modifica nunca; los cambios van en una nueva.
-- Sin BEGIN/COMMIT: cada migración ya se ejecuta dentro de una transacción.

CREATE TABLE settings (
    key        TEXT PRIMARY KEY NOT NULL,
    value      TEXT NOT NULL,
    updated_at TEXT NOT NULL -- UTC ISO-8601
) STRICT;
