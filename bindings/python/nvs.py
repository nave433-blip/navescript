# NaveScript Python Binding
import ctypes

class NvsRuntime:
    def __init__(self):
        self.lib = ctypes.CDLL("./libnvs_runtime.so")
    
    def run(self, code):
        self.lib.nvs_run(code.encode())

# Usage:
# import nvs
# runtime = nvs.NvsRuntime()
# runtime.run("print('Hello from Python!')")
