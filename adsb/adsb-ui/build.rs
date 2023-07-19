use std::{
    io::Write,
    process::{Command, Output},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=./ui");
    println!("cargo:rerun-if-changed=./public");
    println!("cargo:rerun-if-changed=./package.json");
    println!("cargo:rerun-if-changed=./tailwind.config.cjs");
    println!("cargo:rerun-if-changed=./vite.config.js");

    // Don't include sourcemaps
    std::env::set_var("GENERATE_SOURCEMAP", "false");

    handle_output(
        Command::new("yarn")
            .args(["install", "--frozen-lockfile", "--non-interactive"])
            .output()?,
        "yarn install",
    )?;

    handle_output(Command::new("vite").args(["build"]).output()?, "vite build")?;

    Ok(())
}

fn handle_output(
    output: Output,
    command_description: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    if !output.status.success() {
        let mut stderr = std::io::stderr();
        stderr
            .write_all(b"Error when running yarn install.\nStdout:\n\n")
            .unwrap();
        stderr.write_all(&output.stdout).unwrap();
        stderr.write_all(b"\n\nStderr:\n\n").unwrap();
        stderr.write_all(&output.stderr).unwrap();
        return Err(format!(
            "Unable to run {}. See stdout and stderr output above",
            command_description
        )
        .into());
    }
    Ok(())
}
