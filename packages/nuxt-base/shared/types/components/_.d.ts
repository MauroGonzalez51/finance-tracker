import type { Component } from "vue";

declare global {
    namespace Components {
        type ComponentLoader<T extends Component> = () => Promise<{ default: T }>;
    }
}
