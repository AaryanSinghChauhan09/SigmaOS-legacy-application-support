//! SigmaOS-legacy-application-support — thin CLI entry point.
//!
//! The library lives in `src/lib.rs`; this binary only wires argv/stdout.

use sigmaos_legacy_application_support_lib::Component;

fn main() {
    let mut component = Component::new();
    if let Err(e) = component.init().and_then(|_| component.run()) {
        eprintln!("SigmaOS-legacy-application-support: {e}");
        std::process::exit(1);
    }
    println!("SigmaOS-legacy-application-support: ok");
}
