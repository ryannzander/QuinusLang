//! Bring/module preprocessor for QuinusLang
//! Reads .q files, resolves bring statements recursively, outputs flattened source.
//! Handles module-level compile flags: #define, #if, #else, #endif.

use crate::error::{Error, Result};
use std::collections::HashSet;
use std::path::{Path, PathBuf};

/// Process #define, #if, #ifdef, #ifndef, #else, #endif. Modifies defines in place. Returns filtered source.
pub fn apply_compile_flags(source: &str, defines: &mut HashSet<String>) -> String {
    let mut out = String::new();
    let mut stack: Vec<bool> = vec![true]; // true = take, false = skip. Top = current branch.

    for line in source.lines() {
        let trimmed = line.trim_start();
        let take = *stack.last().unwrap_or(&true);
        if trimmed.starts_with("#define ") {
            if take {
                let rest = trimmed[7..].trim_start();
                let name = rest
                    .split_whitespace()
                    .next()
                    .unwrap_or("")
                    .trim_end_matches(|c: char| !c.is_alphanumeric() && c != '_');
                if !name.is_empty() {
                    defines.insert(name.to_string());
                }
            }
            continue;
        }
        if trimmed.starts_with("#if ") {
            let rest = trimmed[4..].trim_start();
            let cond = rest.split_whitespace().next().unwrap_or("");
            let ok = defines.contains(cond) || cond == "1" || cond == "true";
            stack.push(ok && take);
            continue;
        }
        if trimmed == "#ifndef" || trimmed.starts_with("#ifndef ") {
            let name = if trimmed == "#ifndef" {
                ""
            } else {
                trimmed[8..]
                    .trim_start()
                    .split_whitespace()
                    .next()
                    .unwrap_or("")
            };
            let ok = !defines.contains(name);
            stack.push(ok && take);
            continue;
        }
        if trimmed.starts_with("#ifdef ") {
            let name = trimmed[7..]
                .trim_start()
                .split_whitespace()
                .next()
                .unwrap_or("");
            let ok = defines.contains(name);
            stack.push(ok && take);
            continue;
        }
        if trimmed == "#else" {
            if let Some(top) = stack.last_mut() {
                *top = !*top;
            }
            continue;
        }
        if trimmed == "#endif" {
            stack.pop();
            continue;
        }
        if take {
            out.push_str(line);
            out.push('\n');
        }
    }
    out
}

/// Resolve a bring path (e.g. ["compiler", "lexer"] or ["vec"]) to a file path.
/// Resolution order: base_dir/path.q, base_dir/src/path.q, base_dir/stdlib/path.q
/// Special case: "std.fs" -> stdlib/fs.q (flat stdlib layout)
fn resolve_bring_path(base_dir: &Path, path: &[String]) -> Result<PathBuf> {
    let rel: PathBuf = path.iter().collect();
    let ext = rel.with_extension("q");

    let mut search_roots: Vec<PathBuf> = vec![base_dir.to_path_buf()];

    // Also search next to the compiler executable (installed stdlib)
    if let Ok(exe) = std::env::current_exe() {
        if let Some(exe_dir) = exe.parent() {
            if exe_dir != base_dir {
                search_roots.push(exe_dir.to_path_buf());
            }
        }
    }

    let mut candidates: Vec<PathBuf> = Vec::new();
    for root in &search_roots {
        candidates.push(root.join(&ext));
        candidates.push(
            root.join("src")
                .join(ext.file_name().unwrap_or(std::ffi::OsStr::new("main.q"))),
        );
        candidates.push(root.join("stdlib").join(&ext));
        candidates.push(root.join(rel.join("mod.q")));
        candidates.push(root.join("stdlib").join(rel.join("mod.q")));
        if path.len() == 2 && path[0] == "std" {
            candidates.push(root.join("stdlib").join(format!("{}.q", path[1])));
        }
    }

    for p in &candidates {
        if p.exists() {
            return Ok(p.clone());
        }
    }
    let path_str = path.join(".");
    Err(Error::Package {
        message: format!("Module not found: {} (tried {:?})", path_str, candidates),
    })
}

/// Extract bring paths from source: find all `bring "path";` or `bring path;` statements.
/// Only matches at line start (after newline or file start) to avoid matching "bring" in strings.
fn extract_brings(source: &str) -> Vec<Vec<String>> {
    let mut brings = Vec::new();
    for line in source.lines() {
        let trimmed = line.trim_start();
        if trimmed.starts_with("bring") {
            let rest = trimmed[5..].trim_start();
            if let Some(path) = parse_bring_path(rest) {
                brings.push(path);
            }
        }
    }
    brings
}

/// Parse bring path from rest of line: " \"compiler.lexer\" ;" or " compiler.lexer ;"
fn parse_bring_path(rest: &str) -> Option<Vec<String>> {
    let rest = rest.trim_start();
    let path_str = if rest.starts_with('"') {
        let end = rest[1..].find('"')?;
        rest[1..=end].to_string()
    } else {
        let end = rest
            .find(|c: char| c == ';' || c.is_whitespace())
            .unwrap_or(rest.len());
        rest[..end].to_string()
    };
    let path: Vec<String> = path_str.split('.').map(String::from).collect();
    if path.is_empty() || path.iter().any(|s| s.is_empty()) {
        return None;
    }
    Some(path)
}

/// Remove bring statements from content, keep everything else.
fn content_without_brings(source: &str) -> String {
    let mut out = String::new();
    let mut i = 0;
    while i < source.len() {
        let line_start = i;
        let line_end = source[i..]
            .find('\n')
            .map(|p| i + p)
            .unwrap_or(source.len());
        let line = source[line_start..line_end].trim();
        i = if line_end < source.len() {
            line_end + 1
        } else {
            source.len()
        };

        // Skip bring "path"; or bring path; lines
        if line.starts_with("bring") && (line.ends_with(';') || line.contains(';')) {
            continue;
        }
        out.push_str(&source[line_start..i]);
    }
    out
}

/// Recursively resolve brings and produce flattened source.
fn flatten_inner(
    source: &str,
    base_dir: &Path,
    seen: &mut HashSet<String>,
    output: &mut String,
    defines: &mut HashSet<String>,
) -> Result<()> {
    let brings = extract_brings(source);
    for path in brings {
        let path_str = path.join(".");
        if seen.contains(&path_str) {
            continue;
        }
        seen.insert(path_str.clone());
        let file_path = resolve_bring_path(base_dir, &path)?;
        let sub_source = std::fs::read_to_string(&file_path).map_err(|e| Error::Package {
            message: format!("Failed to read {}: {}", file_path.display(), e),
        })?;
        flatten_inner(&sub_source, base_dir, seen, output, defines)?;
    }

    let body = content_without_brings(source);
    let body = apply_compile_flags(&body, defines);
    if !body.trim().is_empty() {
        if !output.is_empty() {
            output.push('\n');
        }
        output.push_str(body.trim());
    }
    Ok(())
}

/// Preprocess a .q file: resolve all bring statements recursively and return flattened source.
/// Pass initial defines (e.g. from --define FOO) to apply_compile_flags.
pub fn preprocess(source: &str, base_dir: &Path) -> Result<String> {
    preprocess_with_defines(source, base_dir, &[])
}

/// Preprocess with optional compile-time defines (e.g. --define DEBUG).
pub fn preprocess_with_defines(
    source: &str,
    base_dir: &Path,
    defines: &[String],
) -> Result<String> {
    let mut seen = HashSet::new();
    let mut def_set: HashSet<String> = defines.iter().cloned().collect();
    let mut output = String::new();
    flatten_inner(source, base_dir, &mut seen, &mut output, &mut def_set)?;
    Ok(output)
}
