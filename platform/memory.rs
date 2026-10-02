// SPDX-License-Identifier: GPL-2.0-only
//! Memory telemetry implementation.

#![allow(dead_code)]

use crate::platform::{MemoryBudget, MemoryTelemetry};

pub struct PlatformMemoryTelemetry;

impl PlatformMemoryTelemetry {
    pub fn new() -> Self {
        PlatformMemoryTelemetry
    }
}

impl MemoryTelemetry for PlatformMemoryTelemetry {
    fn read_memory_budget(&self) -> Option<MemoryBudget> {
        let text = std::fs::read_to_string("/proc/meminfo").ok()?;
        let (used_mib, total_mib) = meminfo_parser(&text)?;
        Some(MemoryBudget {
            vram_total: 0,
            gtt_total: 0,
            vram_used: 0,
            gtt_used: 0,
            sys_total: total_mib as u64 * 1024 * 1024,
            sys_available: (total_mib - used_mib) as u64 * 1024 * 1024,
        })
    }
}

fn meminfo_parser(text: &str) -> Option<(u64, u64)> {
    crate::platform::telemetry::parse_meminfo(text).map(|(u, t)| (u as u64, t as u64))
}
