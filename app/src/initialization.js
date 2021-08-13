(() => {
    // Show the window when initialized
    document.addEventListener("DOMContentLoaded", () => {
        window.rpc.notify("init");
    });
    // Prevent user to reload the page
    document.addEventListener("keydown", (e) => {
        if (e.key === "F5" ||
            (e.ctrlKey && e.key === "r") ||
            (e.ctrlKey && e.key === "R")) {
            e.preventDefault();
        }
    });
    // Disable WebView default context menu
    document.addEventListener("contextmenu", (e) => {
        e.preventDefault();
    });
})();