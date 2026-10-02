// SPDX-License-Identifier: GPL-2.0-only
//! Telemetry helpers.

use crate::platform::{MemoryBudget, TelemetryData};

pub fn parse_meminfo(text: &str) -> Option<(u32, u32)> {
    let mut total = 0u64;
    let mut avail = 0u64;
    for l in text.lines() {
        if let Some((k, v)) = l.split_once(':') {
            if let Ok(val) = v.split_whitespace().next().unwrap_or("0").parse::<u64>() {
                let bytes = val * 1024;
                if k.trim() == "MemTotal" {
                    total = bytes;
                } else if k.trim() == "MemAvailable" {
                    avail = bytes;
                }
            }
        }
    }
    if total > 0 {
        let total_mib = (total / (1024 * 1024)) as u32;
        let avail_mib = (avail / (1024 * 1024)) as u32;
        let used_mib = total_mib.saturating_sub(avail_mib);
        Some((used_mib, total_mib))
    } else {
        None
    }
}
