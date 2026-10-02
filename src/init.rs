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
/// Runs `systemctl --version`: success → systemd, failure → OpenRC.
pub fn is_systemd() -> bool {
	*IS_SYSTEMD.get_or_init(|| {
		let ok = std::process::Command::new("systemctl")
			.arg("--version")
			.output()
			.map(|o| o.status.success())
			.unwrap_or(false);
		ok
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

/// Check whether a service/unit is enabled. Works on systemd and OpenRC.
pub fn service_enabled(unit: &str) -> bool {
	if is_systemd() {
		std::process::Command::new("systemctl")
			.args(["is-enabled", unit])
			.status()
			.map(|s| s.success())
			.unwrap_or(false)
	} else {
		let out = std::process::Command::new("rc-update")
			.args(["show", "--bare", "default"])
			.output();
		match out {
			Ok(o) => {
				let s = match String::from_utf8(o.stdout) {
					Ok(s) => s,
					Err(_) => return false,
				};
				s.split_whitespace().any(|name| name == unit)
			}
			Err(_) => false,
		}
	}
}

/// Restart a service/unit. Works on systemd and OpenRC.
pub fn service_restart(unit: &str) {
	service_ctl(unit, "restart");
}

/// Enable or disable a service/unit. Works on systemd and OpenRC.
pub fn service_ctl_unit(unit: &str, action: &str) {
	if is_systemd() {
		std::process::Command::new("systemctl")
			.args([action, unit])
			.status()
			.ok();
	} else {
		let action = match action {
			"enable" => "add",
			"disable" => "del",
			_ => "add",
		};
		std::process::Command::new("rc-update")
			.args([action, "--sysvinit", unit])
			.status()
			.ok();
	}
}

/// Build the argv for running a service in a transient scope.
/// On systemd: systemd-run --unit <unit> …
/// On OpenRC: rc-service directly.
pub fn service_run_argv(unit: &str, _rpc_bin: &str, _bind: &str, _port: u16) -> Vec<String> {
	if is_systemd() {
		vec![
			"systemd-run".into(),
			format!("--unit={unit}"),
			"--collect".into(),
			"--pipe".into(),
			"sh".into(),
			"-c".into(),
			"sh".into(),
		]
	} else {
		vec![
			"rc-service".into(),
			"start".into(),
			unit.into(),
		]
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
