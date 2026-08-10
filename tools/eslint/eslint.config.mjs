import { withConfig } from "./src/index.ts";

export default withConfig({
    type: "lib",
    typescript: true,
    ignores: ["dist", "node_modules", ".eslintcache"],
});
