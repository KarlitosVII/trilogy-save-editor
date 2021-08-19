
export async function js_save_file(rpc_file) {
    return window.rpc.call("save_file", rpc_file);
}

export async function js_open_file(method) {
    return window.rpc.call(method);
}

export async function js_open_file_with_path(method, path) {
    return window.rpc.call(method, path);
}

export async function js_save_file_dialog(method, path) {
    return window.rpc.call(method, path);
}