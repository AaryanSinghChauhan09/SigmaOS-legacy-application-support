//! Integration tests for SigmaOS-legacy-application-support.

use sigmaos_legacy_application_support_lib::Component;

#[test]
fn component_initialises_and_runs() {
    let mut c = Component::new();
    c.init().expect("init should succeed");
    c.run().expect("run should succeed after init");
}
