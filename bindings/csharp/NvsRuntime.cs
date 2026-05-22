// NaveScript C# P/Invoke Binding
using System;
using System.Runtime.InteropServices;

public class NvsRuntime {
    [DllImport("nvs_runtime.dll")]
    private static extern IntPtr nvs_create_runtime();
    
    [DllImport("nvs_runtime.dll")]
    private static extern void nvs_run(IntPtr handle, string code);
    
    private IntPtr _handle;
    
    public NvsRuntime() {
        _handle = nvs_create_runtime();
    }
    
    public void Run(string code) {
        nvs_run(_handle, code);
    }
}
