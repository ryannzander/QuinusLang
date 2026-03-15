//! Fetch packages from registry or Git

use crate::error::{Error, Result};
use crate::package::manifest::Dependency;
use std::path::Path;
use std::process::Command;

pub fn fetch_package(name: &str, dep: &Dependency, dest: &Path) -> Result<()> {
    match dep {
        Dependency::Registry { version } => {
            // In a real implementation, would fetch from registry
            // For now, create directory structure
            let pkg_dir = dest.join(format!("{}-{}", name, version));
            std::fs::create_dir_all(&pkg_dir)
                .map_err(|e| Error::Package {
                    message: format!("Failed to create package dir: {}", e),
                })?;
            Ok(())
        }
        Dependency::Git { url, rev } => {
            let rev_str = rev.as_deref().unwrap_or("main");
            let rev_safe = rev_str.replace('/', "_");
            let pkg_dir = dest.join(format!("{}-git-{}", name, rev_safe));
            if pkg_dir.exists() {
                // Already cloned; could add `git fetch` + checkout for updates
                return Ok(());
            }
            std::fs::create_dir_all(dest)
                .map_err(|e| Error::Package {
                    message: format!("Failed to create packages dir: {}", e),
                })?;
            let mut cmd = Command::new("git");
            cmd.arg("clone");
            if let Some(r) = rev {
                cmd.arg("--branch").arg(r);
            }
            cmd.arg("--depth").arg("1");
            cmd.arg(url);
            cmd.arg(&pkg_dir);
            let status = cmd.status().map_err(|e| Error::Package {
                message: format!("Failed to run git: {}", e),
            })?;
            if !status.success() {
                return Err(Error::Package {
                    message: format!("git clone failed for {} from {}", name, url),
                });
            }
            Ok(())
        }
    }
}
