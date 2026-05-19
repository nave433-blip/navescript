/*
    Omni-Language Plugin Templates
*/

// 1. Rust Plugin Template (C-ABI)
/*
#[no_mangle]
pub extern "C" fn nave_execute(input: *const i8) -> *const i8 {
    "Hello from Rust plugin!".as_ptr() as *const i8
}
*/

// 2. Go Plugin Template (Cgo)
/*
package main
import "C"
//export NaveExecute
func NaveExecute(input *C.char) *C.char {
    return C.CString("Hello from Go!")
}
*/

// 3. Swift Plugin Template
/*
@_cdecl("nave_execute")
public func nave_execute(input: UnsafePointer<CChar>) -> UnsafePointer<CChar> {
    return "Hello from Swift!".withCString { strdup($0) }
}
*/

// 4. Java Plugin Template (JNI)
/*
public class NavePlugin {
    public static String naveExecute(String input) {
        return "Hello from Java!";
    }
}
*/

// 5. Perl Plugin Template (XS)
/*
use XS::Parse::Sublike;
sub nave_execute {
    return "Hello from Perl!";
}
*/

// 6. HTML/Web Plugin Template
/*
<script>
window.nave_execute = (input) => {
    return "Hello from HTML/WebView!";
}
</script>
*/
