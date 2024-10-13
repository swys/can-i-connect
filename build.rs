use std::process::Command;

fn main() {
    // Get the short commit hash from Git
    let output = Command::new("git")
        .args(&["rev-parse", "--short", "HEAD"])
        .output()
        .expect("Failed to get Git commit hash");

    let git_hash = String::from_utf8(output.stdout)
        .expect("Failed to convert Git commit hash to String");

    // Remove any trailing newline characters
    let git_hash = git_hash.trim();

    // Pass the Git commit hash as an environment variable
    println!("cargo:rustc-env=GIT_HASH={}", git_hash);
}