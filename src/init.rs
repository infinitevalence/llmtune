// SPDX-License-Identifier: GPL-2.0-only
//! Init system detection: systemd vs OpenRC (delegates to platform::init).

pub use crate::platform::init::{
	is_systemd, service_active, service_ctl, service_stop, detect_init_system,
};
