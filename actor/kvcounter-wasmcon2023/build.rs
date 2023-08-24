use std::process::Command;

use anyhow::{Context, Result};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=./ui/src");
    println!("cargo:rerun-if-changed=./ui/public");
    println!("cargo:rerun-if-changed=./ui/package.json");
    println!("cargo:rerun-if-changed=./ui/tailwind.config.js");

    // Run npm ci to install dependencies
    Command::new("npm")
        .args(["ci", "--prefix", "ui"])
        .env("GENERATE_SOURCEMAP", "false")
        .output()
        .context("failed to run `npm ci`")?;

    // Build the code
    Command::new("npm")
        .args(["run", "build", "--prefix", "ui"])
        .env("GENERATE_SOURCEMAP", "false")
        .output()
        .context("failed to complete `npm run`")?;

    Ok(())
}
