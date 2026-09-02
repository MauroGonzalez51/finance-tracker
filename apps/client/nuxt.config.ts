export default defineNuxtConfig({
    compatibilityDate: "2025-07-15",
    devtools: { enabled: true },
    extends: ["../../packages/nuxt-base"],

    ///////////////////////////////////////////////////////
    // TAURI REQUIRED CONFIG
    ///////////////////////////////////////////////////////
    ssr: false,

    devServer: {
        host: "0",
    },

    vite: {
        clearScreen: false,
        envPrefix: ["VITE", "TAURI_"],
        server: {
            strictPort: true,
        },
    },
    ignore: ["**/src-tauri/**"],

    modules: ["nuxt-prepare"],
});
