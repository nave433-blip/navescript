// NaveScript Swift Binding
import Foundation

public class NvsRuntime {
    private var handle: UnsafeMutableRawPointer
    
    public init() {
        self.handle = nvs_create_runtime()
    }
    
    public func run(code: String) {
        nvs_run(self.handle, code)
    }
}
