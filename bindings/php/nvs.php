<?php
// NaveScript PHP FFI Binding
$ffi = FFI::cdef("
    void* nvs_create_runtime();
    void nvs_run(void* handle, char* code);
", "libnvs_runtime.so");

class NvsRuntime {
    private $ffi;
    private $handle;

    public function __construct() {
        global $ffi;
        $this->ffi = $ffi;
        $this->handle = $this->ffi->nvs_create_runtime();
    }

    public function run($code) {
        $this->ffi->nvs_run($this->handle, $code);
    }
}
?>
