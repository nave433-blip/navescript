// SwiftNvs: NaveScript-Swift Bridge
import Foundation

public class SwiftNvs {
    private var ctx: UnsafeMutableRawPointer?

    public init() {
        self.ctx = nave_init()
    }

    public func run(path: String) -> Int32 {
        return path.withCString { cPath in
            return nave_run(self.ctx, cPath)
        }
    }
}
