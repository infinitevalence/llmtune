// SPDX-License-Identifier: GPL-2.0-only
//! Nvidia GPU telemetry implementation.

use crate::platform::GpuTelemetry;
use crate::platform::TelemetryData;

pub struct NvidiaGpuTelemetry;

impl NvidiaGpuTelemetry {
    pub fn new() -> Self {
        NvidiaGpuTelemetry
    }
}

impl GpuTelemetry for NvidiaGpuTelemetry {
    fn read_gpu_telemetry(&self) -> Option<TelemetryData> {
        None
    }
}
