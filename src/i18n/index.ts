import i18n from "i18next";
import { initReactI18next } from "react-i18next";
import deCH from "./locales/de-CH.json";
import en from "./locales/en.json";

export const SUPPORTED_LANGUAGES = ["de-CH", "en"] as const;
export type Language = (typeof SUPPORTED_LANGUAGES)[number];
export const DEFAULT_LANGUAGE: Language = "de-CH";

void i18n.use(initReactI18next).init({
  resources: {
    "de-CH": { translation: deCH },
    en: { translation: en },
  },
  lng: DEFAULT_LANGUAGE,
  fallbackLng: DEFAULT_LANGUAGE,
  // React ya escapa el texto.
  interpolation: { escapeValue: false },
});

export default i18n;
