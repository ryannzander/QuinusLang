//! Fetch packages from registry or Git

use crate::error::{Error, Result};
use crate::package::manifest::Dependency;
use std::path::Path;

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
            // In a real implementation, would clone from Git
            let _ = url;
            let _ = rev;
            let pkg_dir = dest.join(format!("{}-git", name));
            std::fs::create_dir_all(&pkg_dir)
                .map_err(|e| Error::Package {
                    message: format!("Failed to create package dir: {}", e),
                })?;
            Ok(())
        }
    }
}
