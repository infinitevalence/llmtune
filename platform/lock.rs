// SPDX-License-Identifier: GPL-2.0-only
//! Advisory file lock implementation.

#![allow(dead_code)]

use crate::platform::{Lock, LockResult};
use anyhow::Result;

pub struct PlatformLock;

impl PlatformLock {
    pub fn new() -> Self {
        PlatformLock
    }
}

impl Lock for PlatformLock {
    fn lock(&self, _path: &str) -> Result<()> {
        Ok(())
    }

    fn try_lock(&self, _path: &str) -> LockResult {
        LockResult::Locked
    }

    fn unlock(&self) -> Result<()> {
        Ok(())
    }
}
