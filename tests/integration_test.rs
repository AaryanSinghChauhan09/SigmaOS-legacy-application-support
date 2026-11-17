// Integration test for SigmaOS-legacy-application-support
// Verifies the crate builds and links as a dependency.
#![allow(unused_imports)]

use legacy_application_support as _;

#[test]
fn crate_links_and_runs() {
    assert_eq!(2 + 2, 4);
}
