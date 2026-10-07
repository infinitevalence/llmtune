// SPDX-License-Identifier: GPL-2.0-only
//! Init system detection, root elevation, sudo execution, and service control.

#![allow(dead_code)]

use std::sync::OnceLock;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio, ExitStatus};
use std::io::Write;
use anyhow::{bail, Result};
use crate::platform::DropinFormat;

static IS_SYSTEMD: OnceLock<bool> = OnceLock::new();

/// Detect the init system once, caching the result.
pub fn is_systemd() -> bool {
	*IS_SYSTEMD.get_or_init(|| {
		if Path::new("/run/openrc").exists() || Path::new("/etc/init.d").is_dir() {
			if let Ok(content) = std::fs::read_to_string("/proc/1/comm") {
				if content.trim() == "systemd" {
					return true;
				}
			}
			return false;
		}
		let systemctl = Command::new("systemctl")
			.arg("--version")
			.output()
			.map(|o| o.status.success())
			.unwrap_or(false);
		systemctl
	})
}

/// Check whether a service/unit is active.
pub fn service_active(unit: &str) -> bool {
	if is_systemd() {
		Command::new("systemctl")
			.args(["is-active", "--quiet", unit])
			.status()
			.map(|s| s.success())
			.unwrap_or(false)
	} else {
		let svc = unit.strip_suffix(".service").unwrap_or(unit);
		Command::new("rc-status")
			.args(["-q", "--query", svc])
			.status()
			.map(|s| s.success())
			.unwrap_or(false)
	}
}

/// Start or stop a service/unit.
pub fn service_ctl(unit: &str, action: &str) {
	if is_systemd() {
		Command::new("systemctl")
			.args([action, unit])
			.status()
			.ok();
	} else {
		let svc = unit.strip_suffix(".service").unwrap_or(unit);
		Command::new("rc-service")
			.args([svc, action])
			.status()
			.ok();
	}
}

pub fn service_stop(unit: &str) {
	service_ctl(unit, "stop");
}

pub fn detect_init_system() -> &'static str {
	if is_systemd() {
		"systemd"
	} else {
		"OpenRC"
	}
}

// ==========================================================================
// Privileged Execution & Root Helpers (moved from src/swap.rs)
// ==========================================================================

pub fn is_root() -> bool {
	unsafe { libc::geteuid() == 0 }
}

#[derive(Clone, Copy)]
pub enum RootMethod {
	Sudo,
	Doas,
}

pub static ROOT_METHOD: OnceLock<RootMethod> = OnceLock::new();

fn detect_root_method() -> RootMethod {
	let _has_doas = Command::new("which")
		.arg("doas")
		.status()
		.map(|s| s.success())
		.unwrap_or(false);

	let has_sudo = Command::new("sudo")
		.args(["-n", "true"])
		.output()
		.map(|o| o.status.success())
		.unwrap_or(false);

	if _has_doas {
		RootMethod::Doas
	} else if has_sudo {
		RootMethod::Sudo
	} else {
		RootMethod::Sudo
	}
}

pub fn root_cmd() -> &'static str {
	ROOT_METHOD.get_or_init(detect_root_method);
	match ROOT_METHOD.get().copied() {
		Some(RootMethod::Sudo) => "sudo",
		Some(RootMethod::Doas) => "doas",
		None => "sudo",
	}
}

static TUI_ACTIVE: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

pub fn set_tui_active(active: bool) {
	TUI_ACTIVE.store(active, std::sync::atomic::Ordering::SeqCst);
}

fn tui_active() -> bool {
	TUI_ACTIVE.load(std::sync::atomic::Ordering::SeqCst)
}

pub fn must_be_noninteractive(tui_active: bool) -> bool {
	tui_active
}

pub fn privileged_argv<'a>(args: &'a [&'a str], root: bool) -> (&'a str, &'a [&'a str]) {
	if root {
		(args[0], &args[1..])
	} else {
		(root_cmd(), args)
	}
}

pub fn sudo(args: &[&str]) -> Result<()> {
	if args.is_empty() {
		bail!("privileged exec called with an empty argv");
	}
	let (prog, rest) = privileged_argv(args, is_root());
	let noninteractive = !is_root() && must_be_noninteractive(tui_active());
	let mut cmd = Command::new(prog);
	if noninteractive && prog == "sudo" {
		cmd.arg("-n");
	}
	let out = cmd.args(rest).output()?;
	if !out.status.success() {
		if noninteractive {
			bail!(
				"this needs an elevation password, but the interactive TUI can't safely prompt for one \
				 (the prompt would write straight to the terminal ratatui is drawing to, with no \
				 way to type an answer) - run `sudo -v` or authenticate in another terminal first, then retry"
			);
		}
		let stderr = String::from_utf8_lossy(&out.stderr);
		let stderr = stderr.trim();
		if stderr.is_empty() {
			bail!("{} {:?} failed ({})", prog, rest, out.status);
		} else {
			bail!("{} {:?} failed ({}): {}", prog, rest, out.status, stderr);
		}
	}
	Ok(())
}

pub fn sudo_failure_message(
	prog: &str,
	rest: &[&str],
	status: ExitStatus,
	stderr: &[u8],
) -> String {
	let msg = String::from_utf8_lossy(stderr);
	let msg = msg.trim();
	if msg.is_empty() {
		format!("{prog} {rest:?} failed ({status})")
	} else {
		format!("{prog} {rest:?} failed ({status}): {msg}")
	}
}

pub fn sudo_tee(path: &Path, content: &str) -> Result<()> {
	let mut cmd = if is_root() {
		Command::new("tee")
	} else {
		let mut c = Command::new(root_cmd());
		c.arg("tee");
		c
	};
	let mut child = cmd
		.arg(path)
		.stdin(Stdio::piped())
		.stdout(Stdio::null())
		.spawn()?;
	child
		.stdin
		.take()
		.expect("stdin piped")
		.write_all(content.as_bytes())?;
	let st = child.wait()?;
	if !st.success() {
		bail!("tee {} failed ({st})", path.display());
	}
	Ok(())
}

pub fn sudo_tee_secret(path: &Path, content: &str) -> Result<()> {
	sudo(&["install", "-m", "600", "/dev/null", &path.to_string_lossy()])?;
	sudo_tee(path, content)
}

fn unit_user(unit: &str, fmt: DropinFormat) -> String {
	match fmt {
		DropinFormat::Systemd => unit_user_systemd(unit),
		DropinFormat::OpenRC => openrc_unit_user(unit),
	}
}

fn unit_user_systemd(unit: &str) -> String {
	Command::new("systemctl")
		.args(["show", "-p", "User", "--value", unit])
		.output()
		.ok()
		.and_then(|o| String::from_utf8(o.stdout).ok())
		.map(|s| s.trim().to_string())
		.filter(|s| !s.is_empty())
		.unwrap_or_else(|| "root".to_string())
}

fn openrc_unit_user(unit: &str) -> String {
	let conf = PathBuf::from(format!("/etc/conf.d/{unit}"));
	let content = if is_root() {
		std::fs::read_to_string(&conf).unwrap_or_default()
	} else {
		Command::new(root_cmd())
			.args(["cat", &conf.to_string_lossy()])
			.output()
			.ok()
			.and_then(|o| String::from_utf8(o.stdout).ok())
			.unwrap_or_default()
	};
	content
		.lines()
		.find(|l| l.starts_with("RC_USER="))
		.and_then(|l| l.strip_prefix("RC_USER="))
		.map(|s| s.trim().to_string())
		.filter(|s| !s.is_empty())
		.unwrap_or_else(|| "root".to_string())
}

pub fn write_api_key_file(unit: &str, key: &str, fmt: DropinFormat) -> Result<String> {
	let path = crate::paths::shared_state_dir().join("api-key");
	let path_s = path.to_string_lossy().to_string();
	let user = unit_user(unit, fmt);
	if let Some(parent) = path.parent() {
		sudo(&["mkdir", "-p", &parent.to_string_lossy()])?;
	}
	sudo_tee_secret(&path, key)?;
	sudo(&["chown", "--", &user, &path_s])?;
	Ok(path_s)
}
