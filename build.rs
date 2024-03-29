use copy_to_output::copy_to_output;
use std::env;

fn main() {
    // Re-runs script if any files in res are changed
    println!("cargo:rerun-if-changed=res/*");
    copy_to_output("resources", &env::var("PROFILE").unwrap()).expect("Could not copy");
    copy_to_output("data", &env::var("PROFILE").unwrap()).expect("Could not copy");
    copy_to_output("LICENSE", &env::var("PROFILE").unwrap()).expect("Could not copy");
    copy_to_output("README.md", &env::var("PROFILE").unwrap()).expect("Could not copy");
}
