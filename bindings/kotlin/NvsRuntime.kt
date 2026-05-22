// NaveScript Kotlin (JNI) Binding
class NvsRuntime {
    init {
        System.loadLibrary("nvs_runtime")
    }

    private var handle: Long = createRuntime()

    private external fun createRuntime(): Long
    external fun run(code: String)
}
