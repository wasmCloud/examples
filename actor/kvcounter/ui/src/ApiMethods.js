// KV - [/${bucket}]
export function getKV(key) {
    return this.getRequest(key === '' || key === undefined ? '/' : `/${key}`);
}
