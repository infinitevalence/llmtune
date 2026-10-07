// SPDX-License-Identifier: GPL-2.0-only
//! Agnostic OS package-manager detection — re-exported from platform::pkg.

pub use crate::platform::pkg::{detect_pm, install_cmd, toolchain_packages, install_packages};
