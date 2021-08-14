
export async function js_open() {
    return window.rpc.call("open");
}

export async function js_save_dialog(path) {
    return window.rpc.call("save_dialog", path);
}

export async function js_save(rpc_file) {
    return window.rpc.call("save", rpc_file);
}

export async function js_reload(path) {
    return window.rpc.call("reload", path);
}

export async function js_load_database(path) {
    return window.rpc.call("load_database", path);
}
