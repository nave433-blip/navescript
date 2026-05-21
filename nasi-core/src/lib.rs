wit_bindgen::generate!({
    path: "../wit/nasi_core.wit",
    world: "nasi-core",
});

struct NasiCore;

impl Guest for NasiCore {
    fn run(input: String) -> String {
        logging::log(&format!("[NASI Component] Received: {}", input));
        format!("NASI Core Response: {} [processed @ Nλve Runtime]", input)
    }
}

// Export the NasiCore implementation to the WIT world
export!(NasiCore);
