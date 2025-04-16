export function sendMessage(message) {
    return new Promise((resolve, reject) => {
        chrome.runtime.sendMessage(message, (response) => {
            if (chrome.runtime.lastError) {
                reject(chrome.runtime.lastError);
            } else {
                resolve(response);
            }
        });
    });
}

export function registerAsyncMessageHandler(asyncHandler) {
    chrome.runtime.onMessage.addListener((message, sender, sendResponse) => {
        Promise.resolve(asyncHandler(message, sender))
            .then((result) => {
                sendResponse(result);
            })
            .catch((error) => {
                console.error("Error in async message handler:", error);
                sendResponse(undefined);
            });
        return true; // Keep channel open for async response
    });
}
