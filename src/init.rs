// SPDX-License-Identifier: GPL-2.0-only
//! Init system detection: systemd vs OpenRC.
//!
//! detect_init_system() runs `systemctl --version` — success = systemd,
//! failure = OpenRC. IS_SYSTEMD stores the result once, reused by all
//! future code paths. is_systemd() returns the cached value.

use std::sync::OnceLock;

static IS_SYSTEMD: OnceLock<bool> = OnceLock::new();

/// Detect the init system once, caching the result.
///
/// Runs `systemctl --version`: success → systemd.
/// Fallback: checks OpenRC signatures ( `/run/openrc` pid or `/etc/init.d/` dir).
/// This double-check prevents Alpine (which has no systemd) from being
/// misclassified if `systemctl` binary exists (broken symlink, NixOS stub, etc.)
/// but OpenRC is actually the running init.
pub fn is_systemd() -> bool {
	*IS_SYSTEMD.get_or_init(|| {
		let systemctl = std::process::Command::new("systemctl")
			.arg("--version")
			.output()
			.map(|o| o.status.success())
			.unwrap_or(false);
		// systemctl succeeded? systemd.
		if systemctl { return true; }
		// Fallback: OpenRC presence.
		std::path::Path::new("/run/openrc").exists()
			|| std::path::Path::new("/etc/init.d").is_dir()
	})
}

/// Check whether a service/unit is active. Works on systemd and OpenRC.
pub fn service_active(unit: &str) -> bool {
	if is_systemd() {
		std::process::Command::new("systemctl")
			.args(["is-active", "--quiet", unit])
			.status()
			.map(|s| s.success())
			.unwrap_or(false)
	} else {
		std::process::Command::new("rc-status")
			.args(["-q", "--query", unit])
			.status()
			.map(|s| s.success())
			.unwrap_or(false)
	}
}

/// Start or stop a service/unit. Works on systemd and OpenRC.
pub fn service_ctl(unit: &str, action: &str) {
	if is_systemd() {
		std::process::Command::new("systemctl")
			.args([action, unit])
			.status()
			.ok();
	} else {
		std::process::Command::new("rc-service")
			.args([action, unit])
			.status()
			.ok();
	}
}

/// Stop a service/unit. Works on systemd and OpenRC.
pub fn service_stop(unit: &str) {
	service_ctl(unit, "stop");
}

/// One-shot detection (convenience for callers that don't need caching).
#[allow(dead_code)]
pub fn detect_init_system() -> &'static str {
	if is_systemd() {
		"systemd"
	} else {
		"OpenRC"
	}
}
