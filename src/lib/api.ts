import { invoke } from "@tauri-apps/api/core";
import type { Language } from "../i18n";

/** Respuesta del comando `app_info` (src-tauri/src/commands.rs). */
export interface AppInfo {
  appVersion: string;
  schemaVersion: number;
  language: Language;
}

export function getAppInfo(): Promise<AppInfo> {
  return invoke<AppInfo>("app_info");
}

export function setLanguage(language: Language): Promise<void> {
  return invoke<void>("set_language", { language });
}

/** Rust solo devuelve `{ code }` en los errores (sin rutas ni datos personales). */
export function errorCode(err: unknown): string {
  if (typeof err === "object" && err !== null && "code" in err && typeof err.code === "string") {
    return err.code;
  }
  return "unknown";
}
