// NaveScript Node.js Binding
const addon = require('./build/Release/nvs_runtime.node');

class NvsRuntime {
    constructor() {
        this.runtime = addon.create_runtime();
    }
    
    run(code) {
        return addon.run(this.runtime, code);
    }
}

module.exports = NvsRuntime;
