use std::path::{Path, PathBuf};

use miette::{IntoDiagnostic, Result};

use crate::errors::GitMitInstallError;
pub fn link(hook_path: &Path, hook_name: &str) -> Result<()> {
    #[cfg(target_os = "windows")]
    let suffix = ".exe";
    #[cfg(not(target_os = "windows"))]
    let suffix = "";
    let binary_path = which::which(format!("mit-{}{}", hook_name, suffix)).unwrap();
    let install_path = hook_path.join(format!("{}{}", hook_name, suffix));
    let install_path_destination = install_path.read_link();
    if let Ok(existing_hook_path) = install_path_destination.and_then(|x| x.canonicalize()) {
        if existing_hook_path == install_path {
            return Ok(());
        }
    }

    if install_path.exists() {
        if let Ok(dest) = install_path.read_link() {
            return Err(GitMitInstallError::ExistingSymlink(
                install_path.to_string_lossy().to_string(),
                dest.to_string_lossy().to_string(),
            )
            .into());
        }

        return Err(
            GitMitInstallError::ExistingHook(install_path.to_string_lossy().to_string()).into(),
        );
    }

    symlink(binary_path, install_path)?;

    Ok(())
}

#[cfg(not(target_os = "windows"))]
fn symlink(binary_path: PathBuf, install_path: PathBuf) -> Result<()> {
    std::os::unix::fs::symlink(binary_path, install_path).into_diagnostic()?;

    Ok(())
}

#[cfg(target_os = "windows")]
fn symlink(binary_path: PathBuf, install_path: PathBuf) -> Result<()> {
    std::os::windows::fs::symlink_file(binary_path, install_path).into_diagnostic()?;

    Ok(())
}
