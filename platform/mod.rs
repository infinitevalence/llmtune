// SPDX-License-Identifier: GPL-2.0-only
//! Platform abstraction layer: traits + compile-time dispatch
//!
//! Wireframe: fill in the TODO stubs.

#![allow(dead_code)]

use anyhow::Result;
use serde::{Serialize, Deserialize};
use std::path::PathBuf;

// ==========================================================================
// MODULE REGISTRY
// ==========================================================================

// GPU telemetry implementations (compile-time selection)
mod amdgpu;
mod nvidia;
mod cpuonly;
mod memory;

// Init system detection + actuators
pub mod init;
pub mod systemd;
mod openrc;
pub use systemd::{SystemdActuator, user_unit_dir};
pub use openrc::OpenrcActuator;

// Unix platform hooks (compile-time: Linux/Unix only)
mod lock;
mod paths;

// Memory/telemetry parsing helpers
mod telemetry;

// ==========================================================================
// TRAIT DEFINITIONS
// ==========================================================================

/// GPU telemetry - hardware-specific, implemented per-compile platform.
pub trait GpuTelemetry {
    /// Read GPU telemetry (gpu_metrics sysfs or NVML/nvidia-smi).
    /// Returns None when no GPU is present.
    fn read_gpu_telemetry(&self) -> Option<TelemetryData>;
}

/// Memory budget - platform-neutral, one impl (Linux)
pub trait MemoryTelemetry {
    /// Read memory budget (/proc/meminfo and equivalent).
    /// Returns None when /proc/meminfo is unreadable.
    fn read_memory_budget(&self) -> Option<MemoryBudget>;
}

/// How a platform (systemd or OpenRC) reconfigures and bounces the llama service.
pub trait Actuator {
    fn stage(&mut self, unit: &str, dropin_content: &str, fmt: DropinFormat) -> Result<()>;
    fn rollback(&mut self, unit: &str) -> Result<()>;
    fn commit(&mut self) -> Result<()>;
    fn restart(&mut self, unit: &str) -> Result<()>;
    fn reload_restart(&mut self, unit: &str) -> Result<()> {
        self.restart(unit)
    }
}

/// Advisory lock (flock / platform equivalent).
pub trait Lock {
    fn lock(&self, path: &str) -> Result<()>;
    fn try_lock(&self, path: &str) -> LockResult;
    fn unlock(&self) -> Result<()>;
}

/// Where all llmtune files live.
pub trait PathResolver {
    fn state_dir() -> PathBuf where Self: Sized;
    fn shared_state_dir() -> PathBuf where Self: Sized;
    fn config_dir() -> Option<PathBuf> where Self: Sized;
    fn lock_dir() -> PathBuf where Self: Sized;
    fn config_file(name: &str) -> Option<PathBuf> where Self: Sized;
    fn write_api_key_file(unit: &str, api_key: &str) -> Result<PathBuf> where Self: Sized;
}

// ==========================================================================
// COMPILE-TIME GPU PLATFORM DISPATCH (set from build.rs)
// ==========================================================================

/// Which GPU platform to use at compile time.
/// Set by `cargo:rustc-env=GPU_PLATFORM=...` in build.rs.
pub enum GpuPlatform {
    /// AMD GPU (sysfs gpu_metrics_v2_2)
    AmdGpu,
    /// Nvidia GPU (NVML or nvidia-smi)
    NvidiaGpu,
    /// CPU-only (no GPU, telemetry always returns None)
    CpuOnly,
}

// TODO: Set GPU_PLATFORM in Cargo.toml: rustc-env=GPU_PLATFORM=...
// For now: a simple hard-coded default. Set by build.rs.
// const GPU_PLATFORM: GpuPlatform = GpuPlatform::AmdGpu;

// ==========================================================================
// RUNTIME DISPATCH (one-time init at startup)
// ==========================================================================

pub fn detect_init_system() -> InitSystemType {
    if init::is_systemd() {
        InitSystemType::Systemd
    } else {
        InitSystemType::OpenRC
    }
}

pub fn is_systemd() -> bool {
    init::is_systemd()
}

pub fn pick_actuator() -> Box<dyn Actuator + 'static> {
    if is_systemd() {
        Box::new(systemd::SystemdActuator::new())
    } else {
        Box::new(openrc::OpenrcActuator::new())
    }
}

pub fn pick_lock() -> Box<dyn Lock + 'static> {
    Box::new(lock::PlatformLock::new())
}

pub fn pick_path_resolver() -> &'static dyn PathResolver {
    &paths::PlatformPathResolver
}

// ==========================================================================
// GPU TELEMETRY DISPATCH
// ==========================================================================

pub fn pick_gpu_telemetry() -> Box<dyn GpuTelemetry + 'static> {
    Box::new(amdgpu::AmdGpuTelemetry::new())
}

// ==========================================================================
// MEMORY BUDGET DISPATCH
// ==========================================================================

pub fn pick_memory_telemetry() -> Box<dyn MemoryTelemetry + 'static> {
    Box::new(memory::PlatformMemoryTelemetry::new())
}

// ==========================================================================
// SUPPORTING TYPES
// ==========================================================================

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InitSystemType {
    Systemd,
    OpenRC,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DropinFormat {
    Systemd,
    OpenRC,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelemetryData {
    pub gfxclk_mhz: u16,
    pub uclk_mhz: u16,
    pub temp_c: f64,
    #[serde(default)]
    pub power_w: Option<f64>,
    #[serde(default)]
    pub mem_total_mib: Option<u32>,
    #[serde(default)]
    pub mem_used_mib: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryBudget {
    pub vram_total: u64,
    pub gtt_total: u64,
    pub vram_used: u64,
    pub gtt_used: u64,
    pub sys_total: u64,
    pub sys_available: u64,
}

pub enum LockResult {
    Locked,
    Busy,
    None,
}
