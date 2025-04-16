import init, { run_background } from "./pkg/wasm_extension.js";

(async () => {
    try {
        await init();
        run_background();
    } catch (error) {
        console.error(error);
    }
})();
