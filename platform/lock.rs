// SPDX-License-Identifier: GPL-2.0-only
//! File locking - unix flock.

use crate::platform::{Lock, LockResult};
use anyhow::Result;
use std::path::PathBuf;

pub struct PlatformLock {
    lock_path: PathBuf,
}

impl PlatformLock {
    pub fn new() -> Self {
        PlatformLock {
            lock_path: PathBuf::from("/tmp/llmtune.lock"),
        }
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
