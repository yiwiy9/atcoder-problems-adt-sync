(async () => {
    try {
        const { default: init, run_content } = await import(chrome.runtime.getURL("pkg/wasm_extension.js"));
        await init();
        run_content();
    } catch (error) {
        console.error(error);
    }
})();
