//! Dependency resolution

use crate::package::manifest::{Dependency, Manifest};
use crate::error::Result;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ResolvedPackage {
    pub name: String,
    pub version: String,
    pub path: String,
}

pub fn resolve_dependencies(manifest: &Manifest) -> Result<Vec<ResolvedPackage>> {
    let mut resolved = Vec::new();
    let mut seen = HashMap::new();

    for (name, dep) in &manifest.dependencies {
        resolve_one(name, dep, &mut resolved, &mut seen)?;
    }

    Ok(resolved)
}

fn resolve_one(
    name: &str,
    dep: &Dependency,
    resolved: &mut Vec<ResolvedPackage>,
    seen: &mut HashMap<String, String>,
) -> Result<()> {
    if seen.contains_key(name) {
        return Ok(());
    }

    let (version, path) = match dep {
        Dependency::Registry { version } => {
            let path = format!("packages/{}-{}", name, version);
            (version.clone(), path)
        }
        Dependency::Git { url: _url, rev } => {
            let rev_str = rev.as_deref().unwrap_or("main");
            let path = format!("packages/{}-git-{}", name, rev_str.replace('/', "_"));
            (format!("git:{}", rev_str), path)
        }
    };

    seen.insert(name.to_string(), version.clone());
    resolved.push(ResolvedPackage {
        name: name.to_string(),
        version,
        path,
    });

    Ok(())
}
