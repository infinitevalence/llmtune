// SPDX-License-Identifier: GPL-2.0-only
//! CPU-only telemetry impl (no GPU).

use crate::platform::GpuTelemetry;
use crate::platform::TelemetryData;

pub struct CpuOnlyTelemetry;

impl CpuOnlyTelemetry {
    pub fn new() -> Self {
        CpuOnlyTelemetry
    }
}

impl GpuTelemetry for CpuOnlyTelemetry {
    fn read_gpu_telemetry(&self) -> Option<TelemetryData> {
        None
    }
}
