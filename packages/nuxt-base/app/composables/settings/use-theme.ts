const ORDER: App.Settings.Theme.Mode[] = ["light", "dark", "system"];

export default function () {
    const mode = useColorMode();

    function updateClasses(dark: boolean) {
        if (!import.meta.client) {
            return;
        }

        document.documentElement.classList.toggle("ion-palette-dark", dark);
        document.documentElement.classList.toggle("dark", dark);
    }

    function getNextMode(current: App.Settings.Theme.Mode) {
        const currentIndex = ORDER.findIndex((mode) => mode === current);
        const nextIndex = (currentIndex + 1) % ORDER.length;
        return ORDER[nextIndex] ?? "light";
    }

    function toggle() {
        mode.preference = getNextMode(mode.preference as App.Settings.Theme.Mode);
    }

    function init() {
        watch(
            () => mode.value,
            (resolvedValue) => updateClasses(resolvedValue === "dark"),
        );
    }

    return {
        mode,
        toggle,
        init,
    };
}
