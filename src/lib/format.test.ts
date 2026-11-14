import { describe, expect, it } from "vitest";
import { formatChf, formatDate } from "./format";

// Intl separa "CHF" del importe con un espacio no separable (U+00A0).
const NBSP = " ";

describe("formatChf", () => {
  it("formatea Rappen con apóstrofo de miles y 2 decimales", () => {
    expect(formatChf(123450)).toBe(`CHF${NBSP}1'234.50`);
    expect(formatChf(123456789)).toBe(`CHF${NBSP}1'234'567.89`);
  });

  it("formatea 0 e importes negativos", () => {
    expect(formatChf(0)).toBe(`CHF${NBSP}0.00`);
    expect(formatChf(-123450)).toBe("CHF-1'234.50");
  });

  it("rechaza importes que no son Rappen enteros", () => {
    expect(() => formatChf(12.5)).toThrow(RangeError);
    expect(() => formatChf(Number.NaN)).toThrow(RangeError);
  });
});

describe("formatDate", () => {
  it("muestra dd.mm.yyyy en hora de Zúrich", () => {
    expect(formatDate("2026-01-01T00:00:00Z")).toBe("01.01.2026");
  });

  it("aplica el cambio de hora (UTC 23:30 ya es el día siguiente en Zúrich)", () => {
    expect(formatDate("2026-03-28T23:30:00Z")).toBe("29.03.2026");
    expect(formatDate("2026-10-24T22:30:00Z")).toBe("25.10.2026");
  });

  it("rechaza fechas no válidas", () => {
    expect(() => formatDate("no es una fecha")).toThrow(RangeError);
  });
});
