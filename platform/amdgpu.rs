// SPDX-License-Identifier: GPL-2.0-only
//! AMD GPU telemetry implementation.

#![allow(dead_code)]

use crate::platform::GpuTelemetry;
use crate::platform::TelemetryData;

pub struct AmdGpuTelemetry;

impl AmdGpuTelemetry {
    pub fn new() -> Self {
        AmdGpuTelemetry
    }
}

impl GpuTelemetry for AmdGpuTelemetry {
    fn read_gpu_telemetry(&self) -> Option<TelemetryData> {
        let blob = std::fs::read("/sys/class/drm/card0/device/gpu_metrics_v2_2").ok()?;
        let _ = blob;
        None
    }
}
