import ctypes
import os

class Nave:
    def __init__(self, lib_path="./libnave.so"):
        self.lib = ctypes.CDLL(lib_path)
        self.lib.nave_init.restype = ctypes.c_void_p
        self.lib.nave_run.argtypes = [ctypes.c_void_p, ctypes.c_char_p]
        self.lib.nave_run.restype = ctypes.c_int
        self.ctx = self.lib.nave_init()

    def run(self, module_path):
        return self.lib.nave_run(self.ctx, module_path.encode('utf-8'))

    def __del__(self):
        self.lib.nave_free(self.ctx)
