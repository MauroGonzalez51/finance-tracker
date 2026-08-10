import type { Component } from "vue";

declare global {
    namespace Components {
        type ComponentLoader = () => Promise<{ default: Component }>;
    }
}
