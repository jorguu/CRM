import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";
import tailwindcss from "@tailwindcss/vite";
// @ts-expect-error type error without @types/node package
import process from "node:process";

const host = process.env.TAURI_DEV_HOST;

// Opciones pensadas para `tauri dev` / `tauri build`.
export default defineConfig(() => ({
  plugins: [react(), tailwindcss()],
  // No tapar los errores de Rust en la consola.
  clearScreen: false,
  server: {
    // Tauri espera un puerto fijo; falla si está ocupado.
    port: 1420,
    strictPort: true,
    host: host || false,
    hmr: host ? { protocol: "ws", host, port: 1421 } : undefined,
    watch: {
      ignored: ["**/src-tauri/**", "**/spikes/**"],
    },
  },
}));
