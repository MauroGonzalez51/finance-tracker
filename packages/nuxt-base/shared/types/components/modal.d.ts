import type { ActionSheetOptions, AlertOptions, IonModal, LoadingOptions } from "@ionic/vue";

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
                props: ActionSheetOptions;
            }

            interface Alert extends Base {
                kind: "ion-alert";
                props: AlertOptions;
            }

            type ModalComponent = typeof IonModal;

            interface Modal extends Base {
                kind: "ion-modal";
                component: Components.ComponentLoader<ModalComponent>;
                props: object;
            }

            interface Loading extends Base {
                kind: "ion-loading";
                props: LoadingOptions;
            }

            type AnyComponent = ActionSheet | Alert | Modal | Loading;
        }

        namespace Method {
            interface LoadComponentOptions {
                autoOpen: boolean;
            }
        }
    }
}
