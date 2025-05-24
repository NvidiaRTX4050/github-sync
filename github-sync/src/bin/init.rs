// 


use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::fs;
use std::process::{Command};

fn get_folder_path_from_user() -> PathBuf {
    print!("📂 Enter the path to the folder you want to sync: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let trimmed = input.trim();

    let path = PathBuf::from(trimmed);
    if !path.exists() || !path.is_dir() {
        eprintln!("❌ Provided path is not a valid folder.");
        std::process::exit(1);
    }

    path
}

fn get_folder_name(path: &Path) -> String {
    path.file_name()
        .expect("Failed to extract folder name.")
        .to_str()
        .expect("Invalid UTF-8 in folder name.")
        .to_string()
}

fn run_command(command: &mut Command) {
    let status = command.status().expect("Failed to execute command");
    if !status.success() {
        eprintln!("❌ Command failed: {:?}", command);
        std::process::exit(1);
    }
}

fn git_init_if_needed(path: &Path) {
    if !path.join(".git").exists() {
        println!("🔧 Initializing git repository...");
        run_command(Command::new("git").arg("init").current_dir(path));
    } else {
        println!("✅ Git repository already initialized.");
    }
}

fn create_gitignore_if_needed(path: &Path) {
    let gitignore_path = path.join(".gitignore");
    if !gitignore_path.exists() {
        println!("📄 Creating .gitignore file...");
        let content = "target/\nnode_modules/\n*.log\n.env\n";
        fs::write(&gitignore_path, content).expect("Failed to write .gitignore");
    }
}

fn git_add_commit(path: &Path) {
    println!("🗂️ Staging and committing files...");
    run_command(Command::new("git").args(["add", "."]).current_dir(path));
    run_command(
        Command::new("git")
            .args(["commit", "-m", "Initial commit"])
            .current_dir(path),
    );
}

fn create_github_repo(path: &Path, repo_name: &str) {
    println!("🌐 Creating private GitHub repository...");
    let status = Command::new("gh")
        .args([
            "repo",
            "create",
            repo_name,
            "--private",
            "--source=.",
            "--remote=origin",
            "--push",
        ])
        .current_dir(path)
        .status()
        .expect("Failed to run gh repo create");

    if !status.success() {
        eprintln!("❌ Failed to create GitHub repository.");
        std::process::exit(1);
    } else {
        println!("✅ GitHub repository created and linked.");
    }
}

fn main() {
    let folder_path = get_folder_path_from_user();
    let repo_name = get_folder_name(&folder_path);

    git_init_if_needed(&folder_path);
    create_gitignore_if_needed(&folder_path);
    git_add_commit(&folder_path);
    create_github_repo(&folder_path, &repo_name);
}
