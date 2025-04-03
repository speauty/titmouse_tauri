import { defineConfig } from "vite"
import vue from "@vitejs/plugin-vue"
import { resolve } from 'path'
import tailwindcss from '@tailwindcss/vite'

const host = process.env.TAURI_DEV_HOST

export default defineConfig(async () => ({
  plugins: [ vue(), tailwindcss()],

  resolve: {
    alias: [
      { find: "@", replacement: resolve(__dirname, "src") },
      { find: "@ant-design/icons-vue/es/icons", replacement: resolve(__dirname, "node_modules/@ant-design/icons-vue/lib/icons") },
    ],
  },
  build: {
    rollupOptions: {
      external: [
      ],
    },
  },
  clearScreen: false,
  server: {
    port: 1420,
    strictPort: true,
    host: host || false,
    hmr: host
      ? {
          protocol: "ws",
          host,
          port: 1421,
        }
      : undefined,
    watch: {
      ignored: ["**/src-tauri/**"],
    },
  },
}))
