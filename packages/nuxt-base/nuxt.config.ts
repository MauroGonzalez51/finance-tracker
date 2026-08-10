import tailwindcss from "@tailwindcss/vite";

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
    css: ["~/assets/css/tailwind.css", "~/assets/css/ionic.css"],
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
            "~/composables/**/!(*test|*.spec).{ts,js,mjs,mts}",
            "~/utils/**/!(*test|*.spec).{ts,js,mjs,mts}",
            "~~/shared/utils/**/!(*test|*.spec).{ts,js,mjs,mts}",
        ],
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
