import { describe, expect, it } from "vitest";
import deCH from "./locales/de-CH.json";
import en from "./locales/en.json";

type Tree = { [key: string]: string | Tree };

function flatten(tree: Tree, prefix = ""): [string, string][] {
  return Object.entries(tree).flatMap(([key, value]) => {
    const path = prefix ? `${prefix}.${key}` : key;
    return typeof value === "string" ? [[path, value] as [string, string]] : flatten(value, path);
  });
}

const deEntries = flatten(deCH);
const enEntries = flatten(en);

describe("traducciones", () => {
  it("de-CH y en tienen exactamente las mismas claves", () => {
    const deKeys = deEntries.map(([key]) => key).sort();
    const enKeys = enEntries.map(([key]) => key).sort();
    expect(enKeys).toEqual(deKeys);
  });

  it("ningún texto está vacío", () => {
    for (const [key, value] of [...deEntries, ...enEntries]) {
      expect(value.trim(), key).not.toBe("");
    }
  });

  it("de-CH no usa «ß» (en Suiza se escribe «ss»)", () => {
    for (const [key, value] of deEntries) {
      expect(value, key).not.toContain("ß");
    }
  });
});
