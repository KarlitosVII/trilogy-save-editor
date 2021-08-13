
export async function js_open() {
    return window.rpc.call("open");
}

export async function js_reload(path) {
    return window.rpc.call("reload", path);
}

export async function js_load_database(path) {
    return window.rpc.call("load_database", path);
}
