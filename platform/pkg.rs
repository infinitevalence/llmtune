// SPDX-License-Identifier: GPL-2.0-only
//! Agnostic OS package-manager detection and package installation layer.

use anyhow::{bail, Result};
use crate::platform::init::sudo;

const PM_CANDIDATES: &[&str] = &["pacman", "apt", "apk", "dnf", "yum"];

fn which(cmd: &str) -> bool {
	std::env::var_os("PATH").map(|path| {
		std::env::split_paths(&path).any(|dir| {
			let p = dir.join(cmd);
			p.is_file()
		})
	}).unwrap_or(false)
}

pub fn detect_pm() -> Option<String> {
	PM_CANDIDATES
		.iter()
		.find(|&&name| which(name))
		.map(|&name| name.to_string())
}

pub fn install_cmd(pkg: &str) -> String {
	detect_pm().map(|pm| {
		match pm.as_str() {
			"apk" => format!("install: apk add {pkg}"),
			"pacman" => format!("install: pacman -S --needed {pkg}"),
			"apt" => format!("install: apt install {pkg}"),
			"dnf" | "yum" => format!("install: {pm} install {pkg}"),
			_ => format!("install: {pm} {pkg}"),
		}
	}).unwrap_or_else(|| {
		format!("install {pkg} for your distro")
	})
}

pub fn toolchain_packages(pm: &str) -> &'static [&'static str] {
	match pm {
		"pacman" => &["base-devel", "cmake", "git", "vulkan-headers", "vulkan-icd-loader", "spirv-headers", "shaderc"],
		"apk" => &["gcc", "g++", "cmake", "git", "vulkan-headers", "vulkan-loader-dev", "spirv-headers", "shaderc-dev"],
		"apt" => &["build-essential", "cmake", "git", "libvulkan-dev", "glslc", "spirv-headers"],
		"dnf" | "yum" => &["gcc-c++", "cmake", "git", "vulkan-headers", "vulkan-loader-devel", "glslc", "spirv-headers"],
		_ => &[],
	}
}

pub fn install_packages(pkgs: &[&str]) -> Result<()> {
	let pm = detect_pm().ok_or_else(|| anyhow::anyhow!("no supported package manager found (pacman, apk, apt, dnf, yum)"))?;
	let args = match pm.as_str() {
		"pacman" => {
			let mut a = vec![pm.clone(), "-S".into(), "--noconfirm".into(), "--needed".into()];
			a.extend(pkgs.iter().map(|s| s.to_string()));
			a
		}
		"apk" => {
			let _ = sudo(&[pm.as_str(), "update"]);
			let mut a = vec![pm.clone(), "add".into()];
			a.extend(pkgs.iter().map(|s| s.to_string()));
			a
		}
		"apt" => {
			let _ = sudo(&[pm.as_str(), "update"]);
			let mut a = vec![pm.clone(), "install".into(), "-y".into()];
			a.extend(pkgs.iter().map(|s| s.to_string()));
			a
		}
		"dnf" | "yum" => {
			let mut a = vec![pm.clone(), "install".into(), "-y".into()];
			a.extend(pkgs.iter().map(|s| s.to_string()));
			a
		}
		_ => bail!("unsupported package manager `{pm}`"),
	};
	let refs: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
	sudo(&refs)
}
