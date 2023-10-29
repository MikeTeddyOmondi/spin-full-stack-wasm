import { defineConfig } from 'vite'
import { svelte } from '@sveltejs/vite-plugin-svelte'

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [svelte()],
  server: {
    port: 3000,
    strictPort: true,
  },
  proxy: {
    "/api": {
      target: "https://localhost:3000/api",
      changeOrigin: true,
      // rewrite: (path) => path.replace(/^\/api/, ""),
      secure: false,
      ws: true,
    },
  },
});
