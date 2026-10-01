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

/// One-shot detection (convenience for callers that don't need caching).
pub fn detect_init_system() -> &'static str {
	if is_systemd() {
		"systemd"
	} else {
		"OpenRC"
	}
}
