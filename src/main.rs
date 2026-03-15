//! QuinusLang compiler CLI

use clap::{Parser, Subcommand};
use quinuslang::{analyze, codegen, parse, package};
use std::path::PathBuf;
use std::process::Command;

#[derive(Parser)]
#[command(name = "quinus")]
#[command(about = "QuinusLang compiler and package manager")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Compile a QuinusLang source file to executable
    Build {
        #[arg(default_value = ".")]
        path: PathBuf,
    },
    /// Build and run a QuinusLang program
    Run {
        #[arg(default_value = ".")]
        path: PathBuf,
    },
    /// Parse a file (debug)
    Parse {
        path: PathBuf,
    },
    /// Create a new package
    Init {
        #[arg(default_value = ".")]
        path: PathBuf,
    },
    /// Add a dependency
    Add {
        name: String,
        #[arg(long)]
        git: Option<String>,
    },
    /// Remove a dependency
    Remove {
        name: String,
    },
    /// Publish to registry
    Publish,
    /// Update dependencies
    Update,
    /// Format source files
    Fmt {
        #[arg(default_value = ".")]
        path: PathBuf,
    },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Build { path } => cmd_build(&path),
        Commands::Run { path } => cmd_run(&path),
        Commands::Parse { path } => cmd_parse(&path),
        Commands::Init { path } => cmd_init(&path),
        Commands::Add { name, git } => cmd_add(&name, git.as_deref()),
        Commands::Remove { name } => cmd_remove(&name),
        Commands::Publish => cmd_publish(),
        Commands::Update => cmd_update(),
        Commands::Fmt { path } => cmd_fmt(&path),
    }
}

fn cmd_build(path: &PathBuf) -> anyhow::Result<()> {
    let (source, entry_path) = find_entry(path)?;
    let program = parse(&source)?;
    let annotated = analyze(&program)?;
    let c_code = codegen::c::generate(&annotated)?;

    let base = if entry_path.is_file() {
        entry_path.parent().unwrap_or(path)
    } else {
        path
    };
    let out_dir = base.join("build");
    std::fs::create_dir_all(&out_dir)?;
    let c_path = out_dir.join("output.c");
    std::fs::write(&c_path, c_code)?;

    let exe_path = out_dir.join("output.exe");

    let targets: Vec<&str> = if std::env::consts::OS == "windows" {
        vec!["x86_64-pc-windows-msvc", "x86_64-pc-windows-gnu"]
    } else {
        vec!["x86_64-unknown-linux-gnu"]
    };

    std::env::set_var("OPT_LEVEL", "0");
    std::env::set_var("OUT_DIR", &out_dir);
    std::env::set_var("PROFILE", "debug");
    std::env::set_var("DEBUG", "true");

    for target in targets {
        std::env::set_var("TARGET", target);
        std::env::set_var("HOST", target);

        let compiler = match cc::Build::new()
            .file(&c_path)
            .cpp(false)
            .try_get_compiler()
        {
            Ok(c) => c,
            Err(_) => continue,
        };

        let mut cmd = compiler.to_command();
        cmd.arg(&c_path);
        if compiler.is_like_msvc() {
            cmd.arg(format!("/Fe:{}", exe_path.display()));
        } else {
            cmd.arg("-o");
            cmd.arg(&exe_path);
        }

        if cmd.status().map(|s| s.success()).unwrap_or(false) {
            println!("Compiled to {}", exe_path.display());
            return Ok(());
        }
    }

    anyhow::bail!(
        "C compiler failed. Install MinGW (winget install mingw), MSVC Build Tools, or Clang.\n\
         C source: {}",
        c_path.display()
    )
}

fn cmd_run(path: &PathBuf) -> anyhow::Result<()> {
    cmd_build(path)?;
    let base = path.canonicalize().unwrap_or_else(|_| path.clone());
    let base = if base.is_file() {
        base.parent().unwrap_or(&base).to_path_buf()
    } else {
        base
    };
    let exe_path = base.join("build").join("output.exe");
    if exe_path.exists() {
        let status = Command::new(&exe_path).status()?;
        std::process::exit(status.code().unwrap_or(1));
    } else {
        println!("Executable not found. Build may have produced assembly only.");
        println!("Install NASM and MinGW GCC to produce .exe files.");
    }
    Ok(())
}

fn cmd_parse(path: &PathBuf) -> anyhow::Result<()> {
    let source = std::fs::read_to_string(path)
        .map_err(|e| anyhow::anyhow!("Failed to read file: {}", e))?;
    let program = parse(&source)?;
    println!("{:#?}", program);
    Ok(())
}

fn cmd_init(path: &PathBuf) -> anyhow::Result<()> {
    let manifest_path = path.join("quinus.toml");
    if manifest_path.exists() {
        anyhow::bail!("quinus.toml already exists");
    }
    let manifest = r#"[package]
name = "my-app"
version = "0.1.0"
authors = []
description = ""

[dependencies]

[build]
entry = "src/main.q"
"#;
    std::fs::write(&manifest_path, manifest)?;
    let src_dir = path.join("src");
    std::fs::create_dir_all(&src_dir)?;
    let main_path = src_dir.join("main.q");
    let main_content = r#"craft main() -> void {
    make shift x: int = 42;
    send;
}
"#;
    std::fs::write(&main_path, main_content)?;
    println!("Created package at {}", path.display());
    Ok(())
}

fn cmd_add(name: &str, git: Option<&str>) -> anyhow::Result<()> {
    let manifest_path = PathBuf::from("quinus.toml");
    if !manifest_path.exists() {
        anyhow::bail!("Run 'quinus init' first");
    }
    let content = std::fs::read_to_string(&manifest_path)?;
    let dep_line = if let Some(url) = git {
        format!("{} = {{ git = \"{}\" }}\n", name, url)
    } else {
        format!("{} = \"*\"\n", name)
    };
    let new_content = if content.contains("[dependencies]") {
        content.replace("[dependencies]", &format!("[dependencies]\n{}", dep_line.trim_end()))
    } else {
        format!("{}\n[dependencies]\n{}\n", content, dep_line)
    };
    std::fs::write(&manifest_path, new_content)?;
    println!("Added dependency: {}", name);
    Ok(())
}

fn cmd_remove(name: &str) -> anyhow::Result<()> {
    let manifest_path = PathBuf::from("quinus.toml");
    if !manifest_path.exists() {
        anyhow::bail!("No quinus.toml found");
    }
    let content = std::fs::read_to_string(&manifest_path)?;
    let lines: Vec<&str> = content
        .lines()
        .filter(|l| !l.trim().starts_with(&format!("{} =", name)))
        .collect();
    std::fs::write(&manifest_path, lines.join("\n"))?;
    println!("Removed dependency: {}", name);
    Ok(())
}

fn cmd_publish() -> anyhow::Result<()> {
    println!("Publish: not yet implemented");
    Ok(())
}

fn cmd_update() -> anyhow::Result<()> {
    let manifest_path = PathBuf::from("quinus.toml");
    if !manifest_path.exists() {
        anyhow::bail!("No quinus.toml found. Run 'quinus init' first.");
    }
    let manifest = package::manifest::parse_manifest(&manifest_path)?;
    let resolved = package::resolve::resolve_dependencies(&manifest)?;
    println!("Resolved {} dependencies", resolved.len());
    Ok(())
}

fn cmd_fmt(path: &PathBuf) -> anyhow::Result<()> {
    println!("Format: not yet implemented for {}", path.display());
    Ok(())
}

fn find_entry(path: &PathBuf) -> anyhow::Result<(String, PathBuf)> {
    let manifest_path = path.join("quinus.toml");
    if manifest_path.exists() {
        let manifest = package::manifest::parse_manifest(&manifest_path)?;
        let entry = path.join(&manifest.build.entry);
        let source = std::fs::read_to_string(&entry)
            .map_err(|e| anyhow::anyhow!("Failed to read {}: {}", entry.display(), e))?;
        return Ok((source, entry));
    }
    let main_path = path.join("src/main.q");
    if main_path.exists() {
        let source = std::fs::read_to_string(&main_path)?;
        return Ok((source, main_path));
    }
    let direct = path.with_extension("q");
    if direct.exists() {
        let source = std::fs::read_to_string(&direct)?;
        return Ok((source, direct));
    }
    anyhow::bail!("No entry point found. Create src/main.q or quinus.toml with [build] entry");
}
