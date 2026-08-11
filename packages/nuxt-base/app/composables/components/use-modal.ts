import type { Component } from "vue";

/**
 * map
 * string -> resolved dom component
 *
 * like, some sort of caching so, the same options
 *
 */
const COMPONENT_REGISTRY = shallowReactive<
    Map<
        string,
        {
            component: Component;
            addedAt: Temporal.Instant;
        }
    >
>(new Map());

export default function () {
    const { $logger } = useNuxtApp();
    const config = useAppConfig();

    const state = useState<Components.Modal.ComposableState>(
        NuxtKeys.Composables.UseModal.State,
        () => ({
            key: undefined,
            kind: "ion-modal",
            open: false,
            props: {},
        }),
    );

    const component = computed<Components.Modal.ComponentKind.ModalComponent | undefined>(() => {
        if (state.value.kind !== "ion-modal" || !state.value.key) {
            return;
        }

        return COMPONENT_REGISTRY.get(state.value.key)
            ?.component as Components.Modal.ComponentKind.ModalComponent;
    });

    async function loadComponent(
        args: Components.Modal.ComponentKind.AnyComponent,
        options?: Components.Modal.Method.LoadComponentOptions,
    ) {
        $logger.info("loadComponent", args);
        const { autoOpen } = options ?? { autoOpen: true };

        $logger.info("loadComponent -> initial state", state.value);

        /**
         * args.kind === 'ion-modal' -> resolve async component
         *  -> save in registry
         */
        if (args.kind === "ion-modal") {
            const existing = COMPONENT_REGISTRY.get(args.key);
            if (existing) {
                setState({ props: args.props });

                if (autoOpen) {
                    setOpen(true);
                }

                return;
            }

            const component = defineAsyncComponent({
                loader: args.component,
                delay: 0,
                timeout: 5_000,
            });

            COMPONENT_REGISTRY.set(args.key, { component, addedAt: Temporal.Now.instant() });
        }

        $logger.info("loadComponent -> updating state");
        setState({ key: args.key, kind: args.kind, open: false, props: args.props });

        if (autoOpen) {
            setOpen(true);
        }

        if (
            config.composables.UseModal.MAX_STORAGE_LENGTH &&
            COMPONENT_REGISTRY.size >= config.composables.UseModal.MAX_STORAGE_LENGTH
        ) {
            const oldest = Array.from(COMPONENT_REGISTRY.entries()).reduce((acc, current) => {
                if (current[1].addedAt <= acc[1].addedAt) {
                    return current;
                }

                return acc;
            });

            $logger.info(`loadComponent -> removed oldest`, oldest);

            COMPONENT_REGISTRY.delete(oldest[0]);
        }

        $logger.info("loadComponent -> final state", state.value);
    }

    function setOpen(open: boolean) {
        state.value.open = open;
    }

    function setState(patch: Partial<Components.Modal.ComposableState>) {
        state.value = { ...state.value, ...patch };
    }

    return {
        state,
        component,
        dispatch: {
            loadComponent,
            setState,
            setOpen,
        },
    };
}
