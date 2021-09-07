(() => {
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

    // Window events
    const MAIN_BUTTON = 1;
    const DOUBLE_CLICK = 2;
    document.addEventListener("DOMContentLoaded", () => {
        // Title bar events
        const drag_zone = document.getElementById("drag_zone");
        drag_zone.addEventListener('mousedown', (e) => {
            if (e.buttons === MAIN_BUTTON) {
                if (e.detail === DOUBLE_CLICK) {
                    window.rpc.notify('toggle_maximize');
                } else {
                    window.rpc.notify('drag_window');
                }
            }
        })

        const minimize = document.getElementById("minimize");
        minimize.addEventListener("click", () => {
            window.rpc.notify("minimize");
        });

        const maximize = document.getElementById("maximize");
        maximize.addEventListener("click", () => {
            window.rpc.notify("toggle_maximize");
        });

        document.addEventListener("tse_maximized_state_changed", (e) => {
            if (e.detail.is_maximized === true) {
                maximize.classList.add("maximized");
            } else {
                maximize.classList.remove("maximized");
            }
        });

        const close = document.getElementById("close");
        close.addEventListener("click", () => {
            window.rpc.notify("close");
        });

        // Show the window when initialized
        window.rpc.notify("init");
    });
})();