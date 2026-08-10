export default defineNuxtPlugin({
    name: "theme",
    parallel: true,
    setup() {
        useTheme().init();
    },
});
