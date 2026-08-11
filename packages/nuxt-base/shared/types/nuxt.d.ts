import type { ConsolaInstance } from "consola";

declare module "#app" {
    interface NuxtApp {
        $logger: ConsolaInstance;
    }
}

declare module "vue" {
    interface ComponentCustomProperties {
        $logger: ConsolaInstance;
    }
}
