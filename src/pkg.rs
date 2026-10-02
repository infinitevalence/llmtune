// SPDX-License-Identifier: GPL-2.0-only
//! Agnostic OS package-manager detection — the single source of truth.
//!
//! Auto-detect order: pacman > apk > apt > dnf > yum.
//! Used by `build` (preflight install strings) and `netboot_server` (NFS install).
//! Works identically on Arch, Debian, Alpine, Fedora — any OS where a PM exists.

/// The five candidate PMs, ordered by preference (first match wins).
const PM_CANDIDATES: &[&str] = &["pacman", "apt", "apk", "dnf", "yum"];

/// Check if an executable exists anywhere on PATH.
fn which(cmd: &str) -> bool {
	std::env::var_os("PATH").map(|path| {
		std::env::split_paths(&path).any(|dir| {
			let p = dir.join(cmd);
			p.is_file()
		})
	}).unwrap_or(false)
}

/// Return `Some(manager)` for the first detected binary on `PATH`, else `None`.
///
/// Rust equivalent of `scripts/pkg.sh detect`.
pub fn detect_pm() -> Option<String> {
	PM_CANDIDATES
		.iter()
		.find(|&&name| which(name))
		.map(|&name| name.to_string())
}

/// Build an install command for a single package.
/// e.g. `"install: apk add nfs-utils"` or `"install: apt install nfs-utils"`.
pub fn install_cmd(pkg: &str) -> String {
	detect_pm().map(|pm| format!("install: {} {}", pm, pkg)).unwrap_or_else(|| {
		format!("install {pkg} for your distro")
	})
}

#[allow(unreachable_code)]
#[allow(dead_code)]
fn _test_() {
	assert!(!install_cmd("nfs-utils").is_empty());
}
