//! QuinusLang compiler CLI

use clap::{Parser, Subcommand};
use quinuslang::{analyze, codegen, fmt, package, parse, preprocess};
use std::path::PathBuf;
use std::process::Command;

#[derive(Parser)]
#[command(name = "quinus")]
#[command(about = "QuinusLang compiler and package manager")]
#[command(version = env!("CARGO_PKG_VERSION"))]
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
        /// Release build (optimize)
        #[arg(long)]
        release: bool,
        /// Emit C only, do not compile
        #[arg(long)]
        emit_c: bool,
        /// Define for #if (e.g. --define DEBUG)
        #[arg(long, action = clap::ArgAction::Append)]
        define: Vec<String>,
    },
    /// Build and run a QuinusLang program
    Run {
        #[arg(default_value = ".")]
        path: PathBuf,
        #[arg(long)]
        release: bool,
    },
    /// Parse a file (debug)
    Parse { path: PathBuf },
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
    Remove { name: String },
    /// Publish to registry
    Publish,
    /// Update dependencies
    Update,
    /// Format source files
    Fmt {
        #[arg(default_value = ".")]
        path: PathBuf,
    },
    /// Watch and rebuild on changes
    Watch {
        #[arg(default_value = ".")]
        path: PathBuf,
    },
    /// Interactive REPL (parse and show AST)
    Repl,
    /// Language Server Protocol (for IDE support)
    Lsp,
    /// Parse and type-check only (no codegen)
    Check {
        #[arg(default_value = ".")]
        path: PathBuf,
    },
    /// Preprocess: resolve brings and output flattened .q (for debugging)
    Preprocess {
        path: PathBuf,
        /// Write output to file instead of stdout
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Build {
            path,
            release,
            emit_c,
            define,
        } => cmd_build(&path, release, emit_c, &define),
        Commands::Run { path, release } => cmd_run(&path, release),
        Commands::Parse { path } => cmd_parse(&path),
        Commands::Init { path } => cmd_init(&path),
        Commands::Add { name, git } => cmd_add(&name, git.as_deref()),
        Commands::Remove { name } => cmd_remove(&name),
        Commands::Publish => cmd_publish(),
        Commands::Update => cmd_update(),
        Commands::Fmt { path } => cmd_fmt(&path),
        Commands::Watch { path } => cmd_watch(&path),
        Commands::Repl => cmd_repl(),
        Commands::Lsp => cmd_lsp(),
        Commands::Check { path } => cmd_check(&path),
        Commands::Preprocess { path, output } => cmd_preprocess(&path, output.as_ref()),
    }
}

fn cmd_build(
    path: &PathBuf,
    release: bool,
    emit_c_only: bool,
    defines: &[String],
) -> anyhow::Result<()> {
    let (source, _entry_path) = find_entry(path)?;
    let (base, base_dir) = if path.is_file() {
        let parent = path.parent().unwrap_or(path).to_path_buf();
        let mut project_root = parent.clone();
        let search = parent.canonicalize().unwrap_or(parent.clone());
        let mut p = search.as_path();
        while let Some(next) = p.parent() {
            if next.join("stdlib").exists() || next.join("quinus.toml").exists() {
                project_root = next.to_path_buf();
                break;
            }
            p = next;
        }
        (parent, project_root)
    } else {
        (path.clone(), path.clone())
    };
    let flattened = preprocess::preprocess_with_defines(&source, base_dir.as_path(), defines)?;
    let program = parse(&flattened)?;
    let annotated = analyze(&program)?;
    let c_code = codegen::c::generate(&annotated)?;

    let out_dir = base.join("build");
    std::fs::create_dir_all(&out_dir)?;
    let c_path = out_dir.join("output.c");
    std::fs::write(&c_path, c_code)?;

    if emit_c_only {
        println!("Emitted C to {}", c_path.display());
        return Ok(());
    }

    let exe_path = out_dir.join("output.exe");
    let opt_level = if release { "2" } else { "0" };
    std::env::set_var("OPT_LEVEL", opt_level);

    // Try MSVC first, then MinGW. Either works. MSVC: winget install Microsoft.VisualStudio.2022.BuildTools | MinGW: winget install mingw
    let targets: Vec<&str> = if std::env::consts::OS == "windows" {
        vec!["x86_64-pc-windows-msvc", "x86_64-pc-windows-gnu"]
    } else {
        vec!["x86_64-unknown-linux-gnu"]
    };

    std::env::set_var("OUT_DIR", &out_dir);
    std::env::set_var("PROFILE", "debug");
    std::env::set_var("DEBUG", "true");

    for target in targets {
        std::env::set_var("TARGET", target);
        std::env::set_var("HOST", target);

        let compiler = match cc::Build::new().file(&c_path).cpp(false).try_get_compiler() {
            Ok(c) => c,
            Err(_) => continue,
        };

        let mut cmd = compiler.to_command();
        cmd.arg(&c_path);
        if release {
            if compiler.is_like_msvc() {
                cmd.arg("/O2");
            } else {
                cmd.arg("-O2");
            }
        } else {
            if compiler.is_like_msvc() {
                cmd.arg("/Od");
            } else {
                cmd.arg("-O0");
            }
        }
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
        "C compiler not found. Install one:\n  MSVC: winget install Microsoft.VisualStudio.2022.BuildTools\n  MinGW: winget install mingw\nC source: {}",
        c_path.display()
    )
}

fn cmd_run(path: &PathBuf, release: bool) -> anyhow::Result<()> {
    cmd_build(path, release, false, &[])?;
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
        println!(
            "Executable not found. Check that the C compiler succeeded and output.exe was created."
        );
        println!("If build failed, ensure a C compiler is installed (MSVC or MinGW).");
    }
    Ok(())
}

fn cmd_check(path: &PathBuf) -> anyhow::Result<()> {
    let base = if path.is_file() {
        path.parent().unwrap_or(path).to_path_buf()
    } else {
        path.clone()
    };
    let base_dir = base.as_path();
    let (source, _entry_path) = find_entry(&base)?;
    let flattened = preprocess::preprocess(&source, base_dir)?;
    let program = parse(&flattened)?;
    let _annotated = analyze(&program)?;
    println!("Check passed.");
    Ok(())
}

fn cmd_preprocess(path: &PathBuf, output: Option<&PathBuf>) -> anyhow::Result<()> {
    let (source, base_dir) = if path.is_file() {
        let source = std::fs::read_to_string(path)
            .map_err(|e| anyhow::anyhow!("Failed to read {}: {}", path.display(), e))?;
        let parent = path.parent().unwrap_or(path).to_path_buf();
        let mut project_root = parent.clone();
        let search = parent.canonicalize().unwrap_or(parent.clone());
        let mut p = search.as_path();
        while let Some(next) = p.parent() {
            if next.join("stdlib").exists() || next.join("quinus.toml").exists() {
                project_root = next.to_path_buf();
                break;
            }
            p = next;
        }
        (source, project_root)
    } else {
        let (source, _) = find_entry(path)?;
        (source, path.clone())
    };
    let flattened = preprocess::preprocess(&source, base_dir.as_path())?;
    if let Some(out_path) = output {
        std::fs::write(out_path, &flattened)?;
        println!("Wrote {}", out_path.display());
    } else {
        println!("{}", flattened);
    }
    Ok(())
}

fn cmd_parse(path: &PathBuf) -> anyhow::Result<()> {
    let source =
        std::fs::read_to_string(path).map_err(|e| anyhow::anyhow!("Failed to read file: {}", e))?;
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
        content.replace(
            "[dependencies]",
            &format!("[dependencies]\n{}", dep_line.trim_end()),
        )
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
    let manifest_path = PathBuf::from("quinus.toml");
    if !manifest_path.exists() {
        anyhow::bail!("No quinus.toml found. Run 'quinus init' first.");
    }
    let manifest = package::manifest::parse_manifest(&manifest_path)?;
    let version = &manifest.package.version;
    let name = &manifest.package.name;

    // Validate: entry exists
    let entry_path = PathBuf::from(&manifest.build.entry);
    if !entry_path.exists() {
        anyhow::bail!("Entry point {} not found", manifest.build.entry);
    }

    // Build to validate
    let base = PathBuf::from(".");
    let (source, _) = find_entry(&base)?;
    let flattened = preprocess::preprocess(&source, base.as_path())?;
    let program = parse(&flattened)?;
    let _annotated = analyze(&program)?;

    let tag = format!("v{}", version);
    let status = Command::new("git")
        .args([
            "tag",
            "-a",
            &tag,
            "-m",
            &format!("Release {} {}", name, version),
        ])
        .status();
    match status {
        Ok(s) if s.success() => {
            println!("Created tag {}", tag);
            println!("Run 'git push origin {}' to publish", tag);
        }
        Ok(_) => anyhow::bail!("git tag failed"),
        Err(e) => anyhow::bail!("git not found or failed: {}", e),
    }
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
    let paths: Vec<PathBuf> = if path.is_dir() {
        walkdir::WalkDir::new(path)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().extension().map_or(false, |ext| ext == "q"))
            .map(|e| e.path().to_path_buf())
            .collect()
    } else if path.extension().map_or(false, |e| e == "q") {
        vec![path.clone()]
    } else {
        anyhow::bail!("No .q files found at {}", path.display());
    };
    for p in paths {
        let source = std::fs::read_to_string(&p)?;
        match parse(&source) {
            Ok(program) => {
                let formatted = fmt::format_program(&program);
                if formatted != source {
                    std::fs::write(&p, formatted)?;
                    println!("Formatted {}", p.display());
                }
            }
            Err(e) => eprintln!("Skip {}: {}", p.display(), e),
        }
    }
    Ok(())
}

fn cmd_watch(path: &PathBuf) -> anyhow::Result<()> {
    use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
    use std::sync::mpsc;
    use std::time::Duration;

    let (tx, rx) = mpsc::channel();
    let mut watcher = RecommendedWatcher::new(
        move |res| tx.send(res).expect("watch channel disconnected"),
        Config::default(),
    )?;
    let watch_path = if path.is_file() {
        path.parent().unwrap_or(path).to_path_buf()
    } else {
        path.clone()
    };
    watcher.watch(&watch_path, RecursiveMode::Recursive)?;
    println!("Watching {:?} for changes...", watch_path);
    let debounce = Duration::from_millis(300);
    let mut last_event = std::time::Instant::now()
        .checked_sub(Duration::from_secs(3600))
        .unwrap_or_else(std::time::Instant::now);
    let mut pending = false;
    loop {
        match rx.recv_timeout(Duration::from_millis(100)) {
            Ok(Ok(_)) => {
                pending = true;
                last_event = std::time::Instant::now();
            }
            Ok(Err(e)) => eprintln!("Watch error: {}", e),
            Err(mpsc::RecvTimeoutError::Timeout) => {
                if pending && std::time::Instant::now().duration_since(last_event) >= debounce {
                    pending = false;
                    if let Err(e) = cmd_build(path, false, false, &[]) {
                        eprintln!("Build failed: {}", e);
                    }
                }
            }
            Err(mpsc::RecvTimeoutError::Disconnected) => break,
        }
    }
    Ok(())
}

fn cmd_repl() -> anyhow::Result<()> {
    use std::io::{self, BufRead, Write};
    println!("QuinusLang REPL (type .help for commands)");
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    for line in stdin.lock().lines() {
        let line = line?;
        let trimmed = line.trim();
        if trimmed == ".quit" || trimmed == ".exit" {
            break;
        }
        if trimmed.is_empty() {
            continue;
        }
        if trimmed == ".help" {
            writeln!(stdout, "Commands: .quit .help")?;
            writeln!(stdout, "Enter expressions; type is shown on success.")?;
            stdout.flush()?;
            continue;
        }
        let program = format!("craft _repl() -> void {{ {} }}", line);
        match parse(&program) {
            Ok(p) => match analyze(&p) {
                Ok(annotated) => {
                    writeln!(stdout, "ok")?;
                    if let Some(last) = p.items.first().and_then(|i| {
                        if let quinuslang::ast::TopLevelItem::Fn(f) = i {
                            f.body.last()
                        } else {
                            None
                        }
                    }) {
                        let expr = match last {
                            quinuslang::ast::Stmt::VarDecl { init, .. } => Some(init),
                            quinuslang::ast::Stmt::ExprStmt(e) => Some(e),
                            _ => None,
                        };
                        if let Some(e) = expr {
                            if let Some(ty) = semantic_expr_type(&annotated, e) {
                                writeln!(stdout, "  type: {}", ty)?;
                            }
                        }
                    }
                }
                Err(e) => writeln!(stdout, "Error: {}", e)?,
            },
            Err(e) => writeln!(stdout, "Error: {}", e)?,
        }
        stdout.flush()?;
    }
    Ok(())
}

fn cmd_lsp() -> anyhow::Result<()> {
    use lsp_server::{Connection, Message, Notification, RequestId, Response};
    use lsp_types::{
        Hover, HoverContents, InitializeParams, InitializeResult, PublishDiagnosticsParams,
        ServerCapabilities, TextDocumentSyncCapability, TextDocumentSyncKind,
    };

    let (connection, io_threads) = Connection::stdio();
    let (id, params) = connection.initialize_start()?;
    let _params: InitializeParams = serde_json::from_value(params)?;
    let caps = ServerCapabilities {
        text_document_sync: Some(TextDocumentSyncCapability::Kind(TextDocumentSyncKind::FULL)),
        hover_provider: Some(lsp_types::HoverProviderCapability::Simple(true)),
        ..Default::default()
    };
    let result = InitializeResult {
        capabilities: caps,
        server_info: Some(lsp_types::ServerInfo {
            name: "quinus".to_string(),
            version: Some(env!("CARGO_PKG_VERSION").to_string()),
        }),
        ..Default::default()
    };
    connection.initialize_finish(id, serde_json::to_value(result)?)?;

    let mut documents: std::collections::HashMap<String, String> = std::collections::HashMap::new();

    for msg in &connection.receiver {
        match msg {
            Message::Request(req) => {
                if connection.handle_shutdown(&req)? {
                    return Ok(());
                }
                match req.method.as_str() {
                    "textDocument/hover" => {
                        let (id, params): (RequestId, lsp_types::HoverParams) =
                            serde_json::from_value(req.params)?;
                        let uri = params.text_document_position_params.text_document.uri;
                        let pos = params.text_document_position_params.position;
                        let content = documents
                            .get(&uri.to_string())
                            .and_then(|text| {
                                let offset = position_to_offset(text, pos)?;
                                let program = parse(text).ok()?;
                                let annotated = analyze(&program).ok()?;
                                find_hover_at_offset(&annotated, text, offset)
                            })
                            .unwrap_or_else(|| "No info".to_string());
                        let response = Response::new_ok(
                            id,
                            serde_json::to_value(Hover {
                                contents: HoverContents::Scalar(lsp_types::MarkedString::String(
                                    content,
                                )),
                                range: None,
                            })?,
                        );
                        connection.sender.send(Message::Response(response))?;
                    }
                    _ => {}
                }
            }
            Message::Notification(n) => match n.method.as_str() {
                "textDocument/didOpen" => {
                    let params: lsp_types::DidOpenTextDocumentParams =
                        serde_json::from_value(n.params)?;
                    documents.insert(
                        params.text_document.uri.to_string(),
                        params.text_document.text,
                    );
                }
                "textDocument/didChange" => {
                    let params: lsp_types::DidChangeTextDocumentParams =
                        serde_json::from_value(n.params)?;
                    if let Some(changes) = params.content_changes.first() {
                        documents
                            .insert(params.text_document.uri.to_string(), changes.text.clone());
                    }
                }
                "textDocument/didClose" => {
                    let params: lsp_types::DidCloseTextDocumentParams =
                        serde_json::from_value(n.params)?;
                    documents.remove(&params.text_document.uri.to_string());
                }
                "textDocument/didSave" => {
                    let params: lsp_types::DidSaveTextDocumentParams =
                        serde_json::from_value(n.params)?;
                    let uri = params.text_document.uri.to_string();
                    if let Some(text) = documents.get(&uri) {
                        let diagnostics = collect_diagnostics(text);
                        let notification = Notification::new(
                            "textDocument/publishDiagnostics".into(),
                            PublishDiagnosticsParams {
                                uri: params.text_document.uri,
                                diagnostics,
                                version: None,
                            },
                        );
                        let _ = connection.sender.send(Message::Notification(notification));
                    }
                }
                _ => {}
            },
            _ => {}
        }
    }
    io_threads.join()?;
    Ok(())
}

fn collect_diagnostics(text: &str) -> Vec<lsp_types::Diagnostic> {
    let mut diagnostics = Vec::new();
    match parse(text) {
        Ok(program) => {
            if let Err(e) = analyze(&program) {
                if let quinuslang::Error::Semantic(se) = e {
                    let range = line_col_to_range(text, se.line, se.col);
                    diagnostics.push(lsp_types::Diagnostic {
                        range,
                        severity: Some(lsp_types::DiagnosticSeverity::ERROR),
                        code: None,
                        code_description: None,
                        source: Some("quinus".into()),
                        message: format!("{}", se),
                        related_information: None,
                        tags: None,
                        data: None,
                    });
                }
            }
        }
        Err(e) => {
            let (line, col) = match &e {
                quinuslang::Error::Lexer { line, col, .. } => (*line, *col),
                quinuslang::Error::Parse { line, col, .. } => (*line, *col),
                _ => (1usize, 1usize),
            };
            let range = line_col_to_range(text, Some(line), Some(col));
            diagnostics.push(lsp_types::Diagnostic {
                range,
                severity: Some(lsp_types::DiagnosticSeverity::ERROR),
                code: None,
                code_description: None,
                source: Some("quinus".into()),
                message: format!("{}", e),
                related_information: None,
                tags: None,
                data: None,
            });
        }
    }
    diagnostics
}

fn line_col_to_range(text: &str, line: Option<usize>, col: Option<usize>) -> lsp_types::Range {
    let line = line.unwrap_or(1).saturating_sub(1);
    let col = col.unwrap_or(1).saturating_sub(1);
    let mut line_start = 0;
    let mut line_len = 0;
    for (i, l) in text.lines().enumerate() {
        if i == line {
            line_len = l.chars().count();
            break;
        }
        line_start += l.len() + 1;
    }
    let col = col.min(line_len);
    let char_offset: usize = text[line_start..]
        .char_indices()
        .nth(col)
        .map(|(o, _)| o)
        .unwrap_or(0);
    let start_offset = line_start + char_offset;
    let start = offset_to_position(text, start_offset);
    let end = offset_to_position(text, start_offset + 1);
    lsp_types::Range { start, end }
}

fn offset_to_position(text: &str, offset: usize) -> lsp_types::Position {
    let mut line = 0;
    let mut col = 0;
    let mut current = 0;
    for (i, c) in text.char_indices() {
        if current >= offset {
            return lsp_types::Position {
                line: line as u32,
                character: col as u32,
            };
        }
        if c == '\n' {
            line += 1;
            col = 0;
        } else {
            col += 1;
        }
        current = i + 1;
    }
    lsp_types::Position {
        line: line as u32,
        character: col as u32,
    }
}

fn position_to_offset(text: &str, pos: lsp_types::Position) -> Option<usize> {
    let mut offset = 0;
    for (i, line) in text.lines().enumerate() {
        if i == pos.line as usize {
            return Some(offset + (pos.character as usize).min(line.chars().count()));
        }
        offset += line.len() + 1;
    }
    Some(offset)
}

fn find_hover_at_offset(
    annotated: &quinuslang::semantic::AnnotatedProgram,
    text: &str,
    offset: usize,
) -> Option<String> {
    // Extract identifier at offset (alphanumeric + underscore)
    let chars: Vec<char> = text.chars().collect();
    if offset >= chars.len() {
        return None;
    }
    let mut start = offset;
    while start > 0
        && (chars[start - 1].is_alphabetic()
            || chars[start - 1] == '_'
            || chars[start - 1].is_ascii_digit())
    {
        start -= 1;
    }
    let mut end = offset;
    while end < chars.len()
        && (chars[end].is_alphabetic() || chars[end] == '_' || chars[end].is_ascii_digit())
    {
        end += 1;
    }
    if start >= end {
        return None;
    }
    let name: String = chars[start..end].iter().collect();
    if name.is_empty() {
        return None;
    }
    // Look up in symbol table
    for scope in annotated.symbol_table.scopes.iter().rev() {
        if let Some(ty) = scope.vars.get(&name) {
            return Some(format!("{}: {}", name, ty));
        }
        if scope.funcs.contains_key(&name) {
            if let Some(sig) = scope.funcs.get(&name) {
                let params: Vec<String> = sig.params.iter().map(|t| t.to_string()).collect();
                let ret = sig
                    .return_type
                    .as_ref()
                    .map(|t| t.to_string())
                    .unwrap_or_else(|| "void".to_string());
                return Some(format!("craft {}({}) -> {}", name, params.join(", "), ret));
            }
        }
    }
    None
}

fn semantic_expr_type(
    annotated: &quinuslang::semantic::AnnotatedProgram,
    expr: &quinuslang::ast::Expr,
) -> Option<String> {
    let ty = quinuslang::type_of_expr(&annotated.symbol_table, expr)?;
    Some(format!("{}", ty))
}

#[allow(dead_code)]
fn resolve_build_packages(base_dir: &std::path::Path) -> Vec<(String, PathBuf)> {
    let manifest_path = base_dir.join("quinus.toml");
    if !manifest_path.exists() {
        return vec![];
    }
    let manifest = match package::manifest::parse_manifest(&manifest_path) {
        Ok(m) => m,
        Err(_) => return vec![],
    };
    let packages_dir = base_dir.join("packages");
    for (name, dep) in &manifest.dependencies {
        if let Err(e) = package::fetch::fetch_package(name, dep, &packages_dir) {
            eprintln!("Warning: failed to fetch {}: {}", name, e);
        }
    }
    let resolved = match package::resolve::resolve_dependencies(&manifest) {
        Ok(r) => r,
        Err(_) => return vec![],
    };
    resolved
        .into_iter()
        .map(|p| (p.name, base_dir.join(p.path)))
        .collect()
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
