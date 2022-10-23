use std::{
    io::Write,
    process::{Command, Output},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=./src");
    println!("cargo:rerun-if-changed=./public");
    // println!("cargo:rerun-if-changed=./craco.config.js");
    println!("cargo:rerun-if-changed=./package.json");
    println!("cargo:rerun-if-changed=./tailwind.config.js");

    handle_output(
        Command::new("npm").args(["ci", "--force"]).output()?,
        "npm ci --force",
    )?;

    handle_output(
        Command::new("npm").args(["run", "build"]).output()?,
        "npm run build",
    )?;

    Ok(())
}

fn handle_output(
    output: Output,
    command_description: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    if !output.status.success() {
        let mut stderr = std::io::stderr();
        stderr
            .write_all(b"Error when running npm install.\nStdout:\n\n")
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
