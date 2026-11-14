import { useEffect, useState } from "react";
import { useTranslation } from "react-i18next";
import { Button } from "./components/ui/button";
import { SUPPORTED_LANGUAGES, type Language } from "./i18n";
import { errorCode, getAppInfo, setLanguage, type AppInfo } from "./lib/api";
import { formatChf, formatDate } from "./lib/format";

// Datos de ejemplo para ver los formatos suizos (no son datos de clientes).
const SAMPLE_AMOUNT_RAPPEN = 123450;
const SAMPLE_DATE_UTC = "2026-03-28T23:30:00Z";

type State =
  { status: "loading" } | { status: "ready"; info: AppInfo } | { status: "error"; code: string };

export default function App() {
  const { t, i18n } = useTranslation();
  const [state, setState] = useState<State>({ status: "loading" });

  useEffect(() => {
    getAppInfo()
      .then(async (info) => {
        await i18n.changeLanguage(info.language);
        setState({ status: "ready", info });
      })
      .catch((err: unknown) => setState({ status: "error", code: errorCode(err) }));
  }, [i18n]);

  useEffect(() => {
    document.documentElement.lang = i18n.language;
  }, [i18n.language]);

  async function changeLanguage(language: Language) {
    try {
      await setLanguage(language);
      await i18n.changeLanguage(language);
    } catch (err) {
      setState({ status: "error", code: errorCode(err) });
    }
  }

  if (state.status === "loading") {
    return <main className="p-8 text-slate-600">{t("status.loading")}</main>;
  }

  if (state.status === "error") {
    return (
      <main className="mx-auto max-w-xl p-8">
        <h1 className="text-xl font-semibold text-red-700">{t("errors.title")}</h1>
        <p className="mt-2 text-slate-700">
          {t(`errors.${state.code}`, { defaultValue: t("errors.unknown") })}
        </p>
        <p className="mt-4 text-xs text-slate-500">{t("errors.code", { code: state.code })}</p>
      </main>
    );
  }

  const { info } = state;
  return (
    <main className="mx-auto max-w-xl p-8 text-slate-900">
      <h1 className="text-2xl font-semibold">{t("home.heading")}</h1>
      <p className="mt-1 text-slate-600">{t("home.subtitle")}</p>

      <dl className="mt-6 grid grid-cols-2 gap-x-6 gap-y-2 text-sm">
        <dt className="text-slate-500">{t("info.appVersion")}</dt>
        <dd>{info.appVersion}</dd>
        <dt className="text-slate-500">{t("info.schemaVersion")}</dt>
        <dd>{info.schemaVersion}</dd>
        <dt className="text-slate-500">{t("info.database")}</dt>
        <dd>{t("info.databaseEncrypted")}</dd>
        <dt className="text-slate-500">{t("examples.amount")}</dt>
        <dd>{formatChf(SAMPLE_AMOUNT_RAPPEN)}</dd>
        <dt className="text-slate-500">{t("examples.date")}</dt>
        <dd>{formatDate(SAMPLE_DATE_UTC)}</dd>
      </dl>

      <div className="mt-6 flex items-center gap-2">
        <span className="text-sm text-slate-500">{t("language.label")}</span>
        {SUPPORTED_LANGUAGES.map((language) => (
          <Button
            key={language}
            size="sm"
            variant={i18n.language === language ? "default" : "outline"}
            onClick={() => void changeLanguage(language)}
          >
            {t(`language.${language}`)}
          </Button>
        ))}
      </div>
    </main>
  );
}
