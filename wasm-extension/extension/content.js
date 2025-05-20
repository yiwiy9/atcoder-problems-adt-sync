(async () => {
    try {
        const { default: init, run_content } = await import(chrome.runtime.getURL("pkg/atcoder_problems_adt_sync.js"));
        await init();
        run_content();
    } catch (error) {
        console.error(error);
    }
})();
