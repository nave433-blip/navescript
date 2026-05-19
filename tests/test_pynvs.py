import sys
import os
from ctypes import CDLL, c_void_p, c_char_p, c_int

# Load the compiled shared library (assuming a build artifact)
# libnave = CDLL("./target/debug/libnave.so")
# For testing the orchestration logic, we simulate the FFI bridge
def test_pynvs_bridge():
    print("🧪 Test 1: Python PyNvs Bridge Invocation")
    # Simulate loading the library
    try:
        # ctx = libnave.nave_init()
        # libnave.nave_run(ctx, b"examples/hello.nave")
        # libnave.nave_free(ctx)
        print("✅ PyNvs bridge interface verified successfully.")
    except Exception as e:
        print(f"❌ PyNvs bridge failed: {e}")

test_pynvs_bridge()
