// Formatos suizos en toda la UI, sea cual sea el idioma (D-008).
const SWISS_LOCALE = "de-CH";
const SWISS_TIME_ZONE = "Europe/Zurich";

const chfFormatter = new Intl.NumberFormat(SWISS_LOCALE, {
  style: "currency",
  currency: "CHF",
});

const dateFormatter = new Intl.DateTimeFormat(SWISS_LOCALE, {
  timeZone: SWISS_TIME_ZONE,
  day: "2-digit",
  month: "2-digit",
  year: "numeric",
});

/** Importe en Rappen (entero) → texto en CHF, p. ej. 123450 → "CHF 1'234.50". */
export function formatChf(rappen: number): string {
  if (!Number.isSafeInteger(rappen)) {
    throw new RangeError("formatChf: se esperan Rappen enteros");
  }
  return chfFormatter.format(rappen / 100);
}

/** Fecha UTC ISO-8601 → "dd.mm.yyyy" en hora de Zúrich. */
export function formatDate(isoUtc: string): string {
  const date = new Date(isoUtc);
  if (Number.isNaN(date.getTime())) {
    throw new RangeError("formatDate: fecha ISO-8601 no válida");
  }
  return dateFormatter.format(date);
}
