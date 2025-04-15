use std::fs;
use std::io::{Read, Write};
use std::path::Path;

fn main() {
    // Use multiple techniques to force rebuilds

    // Force Cargo to always run this build script
    println!("cargo:rerun-if-changed=nonexistent-file");

    // Add a timestamp to ensure changes are detected
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    println!("cargo:rustc-env=BUILD_TIMESTAMP={}", timestamp);

    // Increment and set a build counter
    let counter_file = Path::new(&std::env::var("OUT_DIR").unwrap()).join("build_counter");
    let mut counter = 0;

    if counter_file.exists() {
        let mut file = fs::File::open(&counter_file).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        if let Ok(value) = contents.trim().parse::<u64>() {
            counter = value;
        }
    }

    counter += 1;
    let mut file = fs::File::create(&counter_file).unwrap();
    write!(file, "{}", counter).unwrap();

    println!("cargo:rustc-env=BUILD_COUNTER={}", counter);

    // Still include the environment variable check
    println!("cargo:rerun-if-env-changed=FORCE_REBUILD");
}
