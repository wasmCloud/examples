// KV - [/]
export function incrementKV(num = 1) {
    return this.modifyRequest('/', 'POST', {
        num
    });
}
