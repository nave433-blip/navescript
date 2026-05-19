/**
 * NVsodide JS Bridge
 * Allows browser-side applications to interact with Nλvescript components.
 */
class NVsodide {
    constructor() {
        this.instance = null;
    }

    async init(wasmUrl) {
        const response = await fetch(wasmUrl);
        const bytes = await response.arrayBuffer();
        const obj = await WebAssembly.instantiate(bytes, {
            nasi: {
                // Browser-native implementations of NASI interfaces
                log: (msg) => console.log(msg),
                fetch: (url) => fetch(url).then(r => r.text())
            }
        });
        this.instance = obj.instance;
    }

    run(modulePath) {
        return this.instance.exports.nv_run(modulePath);
    }
}
