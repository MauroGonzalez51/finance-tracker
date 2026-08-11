import { withConfig } from "@finance-tracker/tools-eslint";
import { withNuxt } from "./.nuxt/eslint.config.mjs";

export default withNuxt(
    withConfig({
        type: "app",
        typescript: true,
        vue: true,
        ignores: [".output", ".nuxt", "node_modules", ".eslintcache"],
    }),
    {
        files: ["**/*.vue"],
        rules: {
            "vue/script-indent": ["error", 4, { baseIndent: 1 }],
            "style/indent": "off",
            "vue/singleline-html-element-content-newline": "off",
            "vue/operator-linebreak": "off",
            "vue/no-deprecated-slot-attribute": [
                "error",
                {
                    ignore: ["ion-buttons"],
                },
            ],
        },
    },
);
