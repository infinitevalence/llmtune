// SPDX-License-Identifier: GPL-2.0-only
//! AMD GPU telemetry implementation.

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
        // Delegate to main telemetry parser if desired, or return None for now
        let _ = blob;
        None
    }
}
