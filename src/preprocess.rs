//! Bring/module preprocessor for QuinusLang
//! Reads .q files, resolves bring statements recursively, outputs flattened source.

use crate::error::{Error, Result};
use std::collections::HashSet;
use std::path::{Path, PathBuf};

/// Resolve a bring path (e.g. ["compiler", "lexer"] or ["vec"]) to a file path.
/// Resolution order: base_dir/path.q, base_dir/src/path.q, base_dir/stdlib/path.q
fn resolve_bring_path(base_dir: &Path, path: &[String]) -> Result<PathBuf> {
    let rel: PathBuf = path.iter().collect();
    let ext = rel.with_extension("q");
    let candidates = [
        base_dir.join(&ext),
        base_dir
            .join("src")
            .join(ext.file_name().unwrap_or(std::ffi::OsStr::new("main.q"))),
        base_dir.join("stdlib").join(&ext),
        base_dir.join(rel.join("mod.q")),
        base_dir.join("stdlib").join(rel.join("mod.q")),
    ];
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
        flatten_inner(&sub_source, base_dir, seen, output)?;
    }

    let body = content_without_brings(source);
    if !body.trim().is_empty() {
        if !output.is_empty() {
            output.push('\n');
        }
        output.push_str(body.trim());
    }
    Ok(())
}

/// Preprocess a .q file: resolve all bring statements recursively and return flattened source.
pub fn preprocess(source: &str, base_dir: &Path) -> Result<String> {
    let mut seen = HashSet::new();
    let mut output = String::new();
    flatten_inner(source, base_dir, &mut seen, &mut output)?;
    Ok(output)
}
