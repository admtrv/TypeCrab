use std::io::{self};
fn main() -> io::Result<()> {

    let target = std::env::var("TARGET").expect("TARGET not set");

    // Check if the target is for WebAssembly (web target)
    if target.contains("wasm32") {
        // Set the cfg flag for the web target
        println!("cargo:rustc-cfg=getrandom_backend=\"wasm_js\"");
    }
    Ok(())
}
