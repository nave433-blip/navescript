/**
 * NVsodide JS Bridge
 * Allows browser-side applications to interact with Nλvescript components.
 */
class NVsodide {
    constructor() {
        this.instance = null;
    }

    async init(wasmUrl, policies = []) {
        this.policies = policies; // e.g., ["fs:read", "net:connect"]
        const response = await fetch(wasmUrl);
        const bytes = await response.arrayBuffer();
        
        const obj = await WebAssembly.instantiate(bytes, {
            nasi: {
                log: (msg) => console.log(msg),
                syscall: (id, args) => {
                    const cap = this.mapSyscallToCap(id);
                    if (!this.policies.includes(cap)) {
                        throw new Error(`Permission Denied: Capability '${cap}' not granted.`);
                    }
                    console.log(`⚡ Browser NASI syscall: ${cap}`);
                    return 0;
                }
            }
        });
        this.instance = obj.instance;
    }

    mapSyscallToCap(id) {
        const caps = { 1: "fs:read", 2: "fs:write", 3: "net:connect" };
        return caps[id] || "unknown";
    }


    run(modulePath) {
        return this.instance.exports.nv_run(modulePath);
    }
}
