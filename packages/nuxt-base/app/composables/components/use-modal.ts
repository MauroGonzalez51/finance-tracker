import { isEqual } from "es-toolkit";

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
            component: Components.Modal.ResolvedComponent;
            options: Components.Modal.ComponentKind.AnyComponent;
            addedAt: Temporal.Instant;
        }
    >
>(new Map());

export default function () {
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

    async function createComponent(args: Components.Modal.ComponentKind.AnyComponent) {
        if (args.kind === "ion-action-sheet") {
            return await actionSheetController.create(args.controller);
        }

        if (args.kind === "ion-alert") {
            return await alertController.create(args.controller);
        }

        if (args.kind === "ion-loading") {
            return await loadingController.create(args.controller);
        }

        return defineAsyncComponent({ loader: args.component, delay: 0, timeout: 5_000 });
    }

    async function loadComponent(
        args: Components.Modal.ComponentKind.AnyComponent,
        options?: Components.Modal.Method.LoadComponentOptions,
    ) {
        const { autoOpen } = options ?? { autoOpen: true };

        /**
         * check if the provided component already exists on registry
         *
         * if exists ->
         *      kind === 'ion-modal' don't re-create component
         *          -> controlled by props
         *
         *      else -> re-create component
         */
        const entry = COMPONENT_REGISTRY.get(args.key);
        if (entry && entry.options.kind !== "ion-modal") {
            COMPONENT_REGISTRY.set(args.key, {
                component: createComponent(args),
                addedAt: Temporal.Now.instant(),
                options: args,
            });

            if (autoOpen) {
                setOpen(true);
            }

            return;
        }

        COMPONENT_REGISTRY.set(args.key, {
            component: createComponent(args),
            addedAt: Temporal.Now.instant(),
            options: args,
        });

        if (args.kind === "ion-modal" && !isEqual(state.value.props, args.props)) {
            setState({ props: args.props });
        }

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

            COMPONENT_REGISTRY.delete(oldest[0]);
        }
    }

    function setOpen(open: boolean) {
        state.value.open = open;
    }

    function setState(patch: Partial<Components.Modal.ComposableState>) {
        state.value = { ...state.value, ...patch };
    }

    return {
        state,
        dispatch: {
            loadComponent,
            setState,
            setOpen,
        },
    };
}
