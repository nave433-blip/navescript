// NaveScript Java/JNI Binding
public class NvsRuntime {
    static {
        System.loadLibrary("nvs_runtime");
    }
    
    private long handle;
    
    public NvsRuntime() {
        this.handle = createRuntime();
    }
    
    public native long createRuntime();
    public native void run(String code);
}
