import { createConsola } from "consola";

export default defineNuxtPlugin({
    name: "logger",
    parallel: true,
    setup() {
        if (import.meta.dev) {
            return {
                provide: {
                    logger: createConsola({ level: 4 }),
                },
            };
        }

        return {
            provide: {
                logger: createConsola({ level: -999 }),
            },
        };
    },
});
