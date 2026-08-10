import type { ActionSheetOptions, AlertOptions, LoadingOptions } from "@ionic/vue";
import type { Component } from "vue";

declare global {
    namespace Components.Modal {
        interface ComposableState {
            key: string | undefined;
            kind: Components.Modal.ComponentKind.AnyComponent["kind"];
            open: boolean;
            props: object;
        }

        namespace ComponentKind {
            interface Base {
                key: string;
            }

            interface ActionSheet extends Base {
                kind: "ion-action-sheet";
                controller: ActionSheetOptions;
            }

            interface Alert extends Base {
                kind: "ion-alert";
                controller: AlertOptions;
            }

            interface Modal extends Base {
                kind: "ion-modal";
                component: Components.ComponentLoader;
                props: object;
            }

            interface Loading extends Base {
                kind: "ion-loading";
                controller: LoadingOptions;
            }

            type AnyComponent = ActionSheet | Alert | Modal | Loading;
        }

        type ResolvedComponent =
            HTMLIonActionSheetElement | HTMLIonAlertElement | Component | HTMLIonLoadingElement;

        namespace Method {
            interface LoadComponentOptions {
                autoOpen: boolean;
            }
        }
    }
}
