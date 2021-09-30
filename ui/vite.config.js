import { defineConfig } from "vite";
import { svelte } from "@sveltejs/vite-plugin-svelte";
import windi from "vite-plugin-windicss";

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [windi(), svelte()],
  build: {
    target: "es2019",
  },
});
