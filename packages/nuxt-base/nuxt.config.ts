import tailwindcss from "@tailwindcss/vite";
import { createResolver } from "nuxt/kit";

const { resolve } = createResolver(import.meta.url);

export default defineNuxtConfig({
    compatibilityDate: "2025-07-15",
    devtools: { enabled: true },
    modules: [
        "@nuxt/eslint",
        "nuxt-lucide-icons",
        "@vueuse/nuxt",
        "@nuxtjs/ionic",
        "@nuxtjs/color-mode",
    ],

    ///////////////////////////////////////////////////////
    // VITE
    ///////////////////////////////////////////////////////
    css: [resolve("./app/assets/css/tailwind.css"), resolve("./app/assets/css/ionic.css")],
    vite: {
        plugins: [tailwindcss()],
        server: {
            hmr: {
                overlay: false,
            },
        },
        css: {
            devSourcemap: false,
        },
    },
    ///////////////////////////////////////////////////////
    // NUXT
    ///////////////////////////////////////////////////////
    imports: {
        dirs: [
            resolve("./app/composables/**/!(*test|*.spec).{ts,js,mjs,mts}"),
            resolve("./app/utils/**/!(*test|*.spec).{ts,js,mjs,mts}"),
            resolve("./shared/utils/**/!(*test|*.spec).{ts,js,mjs,mts}"),
        ],
    },
    appConfig: {
        composables: {
            UseModal: {
                MAX_STORAGE_LENGTH: 5,
            },
        },
    },

    ///////////////////////////////////////////////////////
    // MODULES CONFIG
    ///////////////////////////////////////////////////////
    eslint: {
        config: {
            standalone: false,
        },
    },
    ionic: {
        integrations: {
            icons: false,
        },
        css: {
            utilities: true,
        },
    },
    colorMode: {
        classSuffix: "",
    },
});
