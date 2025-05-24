use std::process::{Command, Stdio};
use std::io::{self, Write};
use std::env;

fn check_ssh_connection() -> bool {
    let output = Command::new("ssh")
        .arg("-T")
        .arg("git@github.com")
        .output()
        .expect("Failed to execute SSH command");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    println!("SSH Output: {}", stdout);
    println!("SSH Error: {}", stderr);

    stdout.contains("successfully authenticated") || stderr.contains("successfully authenticated")
}

fn generate_ssh_key() {
    println!("Generating SSH key...");
    let status = Command::new("ssh-keygen")
        .arg("-t")
        .arg("ed25519")
        .status()
        .expect("Failed to execute ssh-keygen");

    if status.success() {
        println!("✅ SSH key generated.");
    } else {
        println!("❌ SSH key generation failed.");
        std::process::exit(1);
    }
}

fn gh_auth_login() {
    println!("Starting GitHub CLI auth...");

    // Unset GH_TOKEN env var before running gh auth login
    let mut cmd = Command::new("gh");
    cmd.arg("auth").arg("login");
    cmd.env_remove("GH_TOKEN");

    let status = cmd.status().expect("Failed to execute gh auth login");

    if status.success() {
        println!("✅ gh auth login completed.");
    } else {
        println!("❌ gh auth login failed.");
        std::process::exit(1);
    }
}

fn gh_add_ssh_key() {
    println!("Uploading SSH public key to GitHub...");

    let status = Command::new("gh")
        .arg("ssh-key")
        .arg("add")
        .arg(&format!("{}/.ssh/id_ed25519.pub", env::var("HOME").unwrap()))
        .arg("--title")
        .arg("JugaadSync Key")
        .status()
        .expect("Failed to execute gh ssh-key add");

    if status.success() {
        println!("✅ SSH key added to GitHub.");
    } else {
        println!("❌ Failed to add SSH key to GitHub.");
        std::process::exit(1);
    }
}

fn main() {
    if check_ssh_connection() {
        println!("✅ SSH connection to GitHub is successful!");
    } else {
        println!("❌ SSH connection failed. Setting up...");

        // Step 1: Generate SSH Key
        generate_ssh_key();

        // Step 2: Authenticate with GitHub
        gh_auth_login();

        // Step 3: Upload SSH public key
        gh_add_ssh_key();

        // Step 4: Re-check SSH
        if check_ssh_connection() {
            println!("✅ SSH connection to GitHub is now successful!");
        } else {
            println!("❌ SSH connection still failed. Please check manually.");
        }
    }
}
