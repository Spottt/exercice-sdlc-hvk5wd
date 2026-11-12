import { defineConfig } from "vite";

// En développement, les appels /api sont transmis à l'API Rust.
// API_URL=http://127.0.0.1:3001 npm run dev si l'API tourne sur un autre port.
export default defineConfig({
  server: { proxy: { "/api": process.env.API_URL ?? "http://127.0.0.1:3000" } },
});
