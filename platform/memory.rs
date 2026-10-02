// SPDX-License-Identifier: GPL-2.0-only
//! Memory budget implementation.

use crate::platform::{MemoryTelemetry, MemoryBudget};

pub struct PlatformMemoryTelemetry;

impl PlatformMemoryTelemetry {
    pub fn new() -> Self {
        PlatformMemoryTelemetry
    }
}

impl MemoryTelemetry for PlatformMemoryTelemetry {
    fn read_memory_budget(&self) -> Option<MemoryBudget> {
        let text = std::fs::read_to_string("/proc/meminfo").ok()?;
        let (total, avail) = meminfo_parser(&text)?;
        Some(MemoryBudget {
            vram_total: 0,
            gtt_total: 0,
            vram_used: 0,
            gtt_used: 0,
            sys_total: total,
            sys_available: avail,
        })
    }
}

fn meminfo_parser(text: &str) -> Option<(u64, u64)> {
    let mut total = 0;
    let mut avail = 0;
    for l in text.lines() {
        if let Some((k, v)) = l.split_once(':') {
            let val = v.split_whitespace().next().and_then(|n| n.parse::<u64>().ok()).unwrap_or(0) * 1024;
            if k.trim() == "MemTotal" {
                total = val;
            } else if k.trim() == "MemAvailable" {
                avail = val;
            }
        }
    }
    if total > 0 {
        Some((total, avail))
    } else {
        None
    }
}
