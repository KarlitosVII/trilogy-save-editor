
export async function call(method) {
    return window.rpc.call(method);
}

export async function call_with_params(method, params) {
    return window.rpc.call(method, params);
}