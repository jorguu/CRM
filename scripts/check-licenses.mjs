// Comprueba que todas las dependencias npm de PRODUCCIÓN (las que van dentro de la app)
// usan una licencia permitida por CLAUDE.md. Uso: `npm run licenses`.
// Si falla, no añadas la licencia a la lista sin el OK de Jorge.
import { execSync } from "node:child_process";

// Misma lista que src-tauri/deny.toml (D-009).
const ALLOWED = new Set([
  "MIT",
  "Apache-2.0",
  "BSD-2-Clause",
  "BSD-3-Clause",
  "ISC",
  "MPL-2.0",
  "Unicode-3.0",
  "Zlib",
  "blessing",
  "WTFPL",
]);

/** Acepta "MIT", "(MIT OR Apache-2.0)" y "MIT AND ISC". */
function isAllowed(expression) {
  if (typeof expression !== "string" || expression.trim() === "") return false;
  const clean = expression.replace(/[()]/g, "").trim();
  if (clean.includes(" AND "))
    return clean.split(" AND ").every((part) => ALLOWED.has(part.trim()));
  return clean.split(" OR ").some((part) => ALLOWED.has(part.trim()));
}

const output = execSync("npm ls --omit=dev --all --long --json", {
  encoding: "utf8",
  maxBuffer: 64 * 1024 * 1024,
});
const tree = JSON.parse(output);

const seen = new Map();
function walk(dependencies = {}) {
  for (const [name, info] of Object.entries(dependencies)) {
    const id = `${name}@${info.version}`;
    if (seen.has(id)) continue;
    seen.set(id, info.license);
    walk(info.dependencies);
  }
}
walk(tree.dependencies);

const rejected = [...seen].filter(([, license]) => !isAllowed(license));
if (rejected.length > 0) {
  console.error("Licencias no permitidas en dependencias de producción:");
  for (const [id, license] of rejected) console.error(`  ${id}: ${license ?? "(sin licencia)"}`);
  process.exit(1);
}
console.log(`Licencias OK: ${seen.size} paquetes de producción.`);
