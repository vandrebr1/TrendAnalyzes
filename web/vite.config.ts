import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";

const GATEWAY = "http://localhost:3000";

/**
 * Everything the gateway owns is proxied so the browser talks to one origin in
 * development, matching how a built bundle is served by the gateway itself.
 *
 * /api is the exception that needs rewriting: the gateway mounts its routes at
 * the root, and the prefix exists only to tell Vite what to forward. Swagger is
 * forwarded untouched so the footer link works in both modes.
 */
export default defineConfig({
  plugins: [react()],
  server: {
    proxy: {
      "/api": {
        target: GATEWAY,
        changeOrigin: true,
        rewrite: (path) => path.replace(/^\/api/, ""),
      },
      "/swagger": { target: GATEWAY, changeOrigin: true },
      "/api-doc": { target: GATEWAY, changeOrigin: true },
    },
  },
});
