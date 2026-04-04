//! Parse qpp.toml manifest

use crate::error::{Error, Result};
use std::collections::HashMap;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct Manifest {
    pub package: PackageInfo,
    pub dependencies: HashMap<String, Dependency>,
    pub build: BuildConfig,
}

#[derive(Debug, Clone)]
pub struct PackageInfo {
    pub name: String,
    pub version: String,
    pub authors: Vec<String>,
    pub description: Option<String>,
}

#[derive(Debug, Clone)]
pub enum Dependency {
    Registry { version: String },
    Git { url: String, rev: Option<String> },
}

#[derive(Debug, Clone)]
pub struct BuildConfig {
    pub entry: String,
    pub out_dir: Option<String>,
    pub optimize: Option<String>,
}

impl Default for Manifest {
    fn default() -> Self {
        Self {
            package: PackageInfo {
                name: "my-app".to_string(),
                version: "0.1.0".to_string(),
                authors: vec![],
                description: None,
            },
            dependencies: HashMap::new(),
            build: BuildConfig {
                entry: "src/main.q".to_string(),
                out_dir: None,
                optimize: Some("debug".to_string()),
            },
        }
    }
}

pub fn parse_manifest(path: &Path) -> Result<Manifest> {
    let content = std::fs::read_to_string(path).map_err(|e| Error::Package {
        message: format!("Failed to read manifest: {}", e),
    })?;

    let mut manifest = Manifest::default();

    let mut in_package = false;
    let mut in_dependencies = false;
    let mut in_build = false;

    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if line == "[package]" {
            in_package = true;
            in_dependencies = false;
            in_build = false;
            continue;
        }
        if line == "[dependencies]" {
            in_package = false;
            in_dependencies = true;
            in_build = false;
            continue;
        }
        if line == "[build]" {
            in_package = false;
            in_dependencies = false;
            in_build = true;
            continue;
        }
        if line.starts_with('[') {
            in_package = false;
            in_dependencies = false;
            in_build = false;
            continue;
        }

        if let Some((key, value)) = line.split_once('=') {
            let key = key.trim().trim_matches('"');
            let value = value.trim().trim_matches('"');

            if in_package {
                match key {
                    "name" => manifest.package.name = value.to_string(),
                    "version" => manifest.package.version = value.to_string(),
                    "authors" => {
                        manifest.package.authors = value
                            .split(',')
                            .map(|s| s.trim().trim_matches('"').to_string())
                            .collect()
                    }
                    "description" => manifest.package.description = Some(value.to_string()),
                    _ => {}
                }
            } else if in_dependencies {
                let dep = if value.starts_with('{') {
                    // Inline table: { git = "..." } or { version = "..." }
                    if value.contains("git") {
                        let url = extract_string(value, "git").unwrap_or_default();
                        let rev = extract_string(value, "rev");
                        Dependency::Git { url, rev }
                    } else {
                        let version =
                            extract_string(value, "version").unwrap_or_else(|| value.to_string());
                        Dependency::Registry { version }
                    }
                } else {
                    Dependency::Registry {
                        version: value.to_string(),
                    }
                };
                manifest.dependencies.insert(key.to_string(), dep);
            } else if in_build {
                match key {
                    "entry" => manifest.build.entry = value.to_string(),
                    "out_dir" => manifest.build.out_dir = Some(value.to_string()),
                    "optimize" => manifest.build.optimize = Some(value.to_string()),
                    _ => {}
                }
            }
        }
    }

    Ok(manifest)
}

fn extract_string(s: &str, key: &str) -> Option<String> {
    let pattern = format!("{} = \"", key);
    if let Some(start) = s.find(&pattern) {
        let start = start + pattern.len();
        if let Some(end) = s[start..].find('"') {
            return Some(s[start..start + end].to_string());
        }
    }
    None
}
