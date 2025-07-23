use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use std::io::{self, Write};

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    New { name: String },
    Build,
    Run,
}

#[derive(Serialize, Deserialize)]
struct PrexToml {
    package: Package,
    dependencies: Option<toml::Value>,
    build: Build,
}

#[derive(Serialize, Deserialize)]
struct Package {
    name: String,
    version: String,
}

#[derive(Serialize, Deserialize)]
struct Build {
    output: String,
}

#[derive(Serialize, Deserialize, Default)]
struct IgnisConfig {
    prex_path: Option<String>,
}

fn find_prex() -> Option<String> {

    if let Ok(conf) = fs::read_to_string(".ignis.conf") {
        if let Ok(cfg) = toml::from_str::<IgnisConfig>(&conf) {
            if let Some(path) = cfg.prex_path {
                if Path::new(&path).exists() {
                    return Some(path);
                }
            }
        }
    }

    if Path::new("../prex").exists() {
        return Some("../prex".to_string());
    }

    if let Ok(path) = which::which("prex") {
        return Some(path.display().to_string());
    }
    None
}

fn ask_prex_path() -> String {
    print!("Enter full path to prex binary: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let path = input.trim().to_string();
    if !Path::new(&path).exists() {
        eprintln!("File does not exist: {}", path);
        std::process::exit(1);
    }
    // Save to .ignis.conf
    let cfg = IgnisConfig { prex_path: Some(path.clone()) };
    let toml = toml::to_string(&cfg).unwrap();
    fs::write(".ignis.conf", toml).unwrap();
    path
}

fn get_prex_path() -> String {
    find_prex().unwrap_or_else(|| ask_prex_path())
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::New { name } => {
            println!("[Ignis] Creating project: {}", name);
            let _ = fs::create_dir(&name);
            let _ = fs::create_dir(format!("{}/src", name));
            let toml = format!(
                "[package]\nname = \"{}\"\nversion = \"0.1.0\"\n[dependencies]\n[build]\noutput = \"output.elf\"\n",
                name
            );
            fs::write(format!("{}/prex.toml", name), toml).unwrap();
            let mainprx = "defun main() > i32 {\n    // Hello from Rust Ignis!\n    ret 0;\n}\n";
            fs::write(format!("{}/src/main.prx", name), mainprx).unwrap();
            println!("[Ignis] Project created: {}", name);
        }
        Commands::Build => {
            println!("[Ignis] Building project in current directory...");
            if !Path::new("prex.toml").exists() {
                eprintln!("No prex.toml found!");
                return;
            }
            let toml_str = fs::read_to_string("prex.toml").unwrap();
            let config: PrexToml = toml::from_str(&toml_str).unwrap();
            let src_dir = Path::new("src");
            let mut files = vec![];
            if let Ok(entries) = fs::read_dir(src_dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.extension().and_then(|s| s.to_str()) == Some("prx") {
                        files.push(path.display().to_string());
                    }
                }
            }
            if files.is_empty() {
                eprintln!("No .prx files found in src/!");
                return;
            }
            let prex_path = get_prex_path();
            let mut cmd = format!("{} ", prex_path);
            for f in &files {
                cmd.push_str(f);
                cmd.push(' ');
            }
            cmd.push_str(&format!("; mv output.elf {}", config.build.output));
            println!("[Ignis] Command: {}", cmd);
            let status = std::process::Command::new("sh").arg("-c").arg(&cmd).status().unwrap();
            if status.success() {
                println!("[Ignis] Build OK! Output: {}", config.build.output);
            } else {
                eprintln!("[Ignis] Build failed!");
            }
        }
        Commands::Run => {
            println!("[Ignis] Building and running project...");
            let toml_str = fs::read_to_string("prex.toml").unwrap();
            let config: PrexToml = toml::from_str(&toml_str).unwrap();
            let src_dir = Path::new("src");
            let mut files = vec![];
            if let Ok(entries) = fs::read_dir(src_dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.extension().and_then(|s| s.to_str()) == Some("prx") {
                        files.push(path.display().to_string());
                    }
                }
            }
            if files.is_empty() {
                eprintln!("No .prx files found in src/!");
                return;
            }
            let prex_path = get_prex_path();
            let mut cmd = format!("{} ", prex_path);
            for f in &files {
                cmd.push_str(f);
                cmd.push(' ');
            }
            cmd.push_str(&format!("; mv output.elf {}", config.build.output));
            let status = std::process::Command::new("sh").arg("-c").arg(&cmd).status().unwrap();
            if status.success() {
                println!("[Ignis] Build OK! Output: {}", config.build.output);
                println!("[Ignis] Running: ./{}", config.build.output);
                let _ = std::process::Command::new(format!("./{}", config.build.output)).status();
            } else {
                eprintln!("[Ignis] Build failed!");
            }
        }
    }
}
