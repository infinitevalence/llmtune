// SPDX-License-Identifier: GPL-2.0-only
//! OpenRC actuator implementation.

#![allow(dead_code)]

use crate::platform::{Actuator, DropinFormat};
use anyhow::Result;
use std::path::PathBuf;
use std::process::Command;
use crate::swap::{is_root, root_cmd, sudo, sudo_tee_secret};

pub struct OpenrcActuator {
    wrote: Option<String>,
    prev: Option<String>,
}

impl OpenrcActuator {
    pub fn new() -> Self {
        OpenrcActuator {
            wrote: None,
            prev: None,
        }
    }
}

impl Default for OpenrcActuator {
    fn default() -> Self {
        Self::new()
    }
}

impl Actuator for OpenrcActuator {
    fn stage(&mut self, unit: &str, dropin_content: &str, _fmt: DropinFormat) -> Result<()> {
        let svc = unit.strip_suffix(".service").unwrap_or(unit);
        let conf = PathBuf::from(format!("/etc/conf.d/{svc}"));
        self.prev = if is_root() {
            std::fs::read_to_string(&conf).ok()
        } else {
            Command::new(root_cmd())
                .args(["cat", &conf.to_string_lossy()])
                .output()
                .ok()
                .and_then(|o| String::from_utf8(o.stdout).ok())
        };
        self.wrote = Some(dropin_content.to_string());
        sudo_tee_secret(&conf, dropin_content)
    }

    fn rollback(&mut self, unit: &str) -> Result<()> {
        if let Some(prev) = self.prev.take() {
            let svc = unit.strip_suffix(".service").unwrap_or(unit);
            let p = PathBuf::from(format!("/etc/conf.d/{svc}"));
            sudo_tee_secret(&p, &prev)?;
        }
        Ok(())
    }

    fn commit(&mut self) -> Result<()> {
        Ok(())
    }

    fn restart(&mut self, unit: &str) -> Result<()> {
        let svc = unit.strip_suffix(".service").unwrap_or(unit);
        sudo(&["rc-service", svc, "restart"])?;
        Ok(())
    }
}
