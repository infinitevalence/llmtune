// SPDX-License-Identifier: GPL-2.0-only
//! Path resolution.

use crate::platform::PathResolver;
use anyhow::Result;
use std::path::PathBuf;

pub struct PlatformPathResolver;

impl PlatformPathResolver {
    pub fn new() -> Self {
        PlatformPathResolver
    }
}

impl PathResolver for PlatformPathResolver {
    fn state_dir() -> PathBuf {
        crate::paths::state_dir()
    }

    fn shared_state_dir() -> PathBuf {
        crate::paths::shared_state_dir()
    }

    fn config_dir() -> Option<PathBuf> {
        crate::paths::config_dir()
    }

    fn lock_dir() -> PathBuf {
        crate::paths::state_dir().join("lock")
    }

    fn config_file(name: &str) -> Option<PathBuf> {
        crate::paths::config_file(name)
    }

    fn write_api_key_file(unit: &str, api_key: &str) -> Result<PathBuf> {
        let path = crate::paths::shared_state_dir().join("api-key");
        crate::platform::init::sudo_tee_secret(&path, api_key)?;
        Ok(path)
    }
}
