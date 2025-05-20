import init, { run_background } from "./pkg/atcoder_problems_adt_sync.js";

(async () => {
    try {
        await init();
        run_background();
    } catch (error) {
        console.error(error);
    }
})();
