wit_bindgen::generate!({
    path: "../wit/nasi_core.wit",
    world: "nasi-core",
});

struct NasiCore;

impl Guest for NasiCore {
    fn run(input: String) -> String {
        // In a real component, logging::log would be imported
        // For this guest, we demonstrate the trait implementation
        format!("NASI v0.4 Response: {} → SUCCESS", input)
    }
}

export!(NasiCore);
