# LLMTune Architecture — `platform/` Layer

> **Status:** Implemented. Actuators, init detection, sudo/root helpers, and platform dispatchers are fully implemented and integrated.

---

## 1. Directory Structure

```
platform/
├── mod.rs          # Traits, dispatch selectors, supporting types, module registry
├── systemd.rs      # systemd Actuator (drop-in staging, commit, restart)
├── openrc.rs       # OpenRC Actuator (/etc/conf.d staging, rc-service restart)
├── lock.rs         # Unix flock abstraction (compile-time: Unix only)
├── paths.rs        # POSIX path resolution (compile-time: Unix only)
├── telemetry.rs    # meminfo_parser + gpu_metrics_v2_2 parsing helpers
├── amdgpu.rs       # AmdGpuTelemetry — /sys/class/drm/card0/device/gpu_metrics_v2_2
├── nvidia.rs       # NvidiaGpuTelemetry — NVML syscalls + nvidia-smi fallback
├── cpuonly.rs      # CpuOnlyTelemetry — always returns None (no GPU)
├── memory.rs       # MemoryTelemetry — /proc/meminfo + per-card VRAM/GTT totals
```

All files are wireframes with `todo!()` stubs. No business logic is implemented yet.

---

## 2. Trait Overview

Traits fall into two categories: **runtime** (trait objects, selected once at startup, reused thereafter)
and **compile-time** (cfg-gated, no monomorphization cost).

### Runtime traits (trait objects, selected once at bootstrap)

| Trait            | Purpose                                              | Dispatch selector         | Selection logic                                    |
| ---------------- | ---------------------------------------------------- | ------------------------- | -------------------------------------------------- |
| `Actuator`       | Stage drop-ins, commit, restart a systemd/OpenRC     | `pick_actuator()`         | Detect systemd vs OpenRC at boot (`systemctl --version`, check `/run/openrc`) |
| `Lock`           | Advisory file lock (`flock` on Unix)                 | `pick_lock()`             | Unix only (compile-time `cfg`)                     |
| `PathResolver`   | Where llmtune state/config files live (e.g., `/etc/llmtune/`) | `pick_path_resolver()` | Unix only (compile-time `cfg`)                     |
| `GpuTelemetry`   | Read GPU metrics (gpu_metrics_v2_2 or NVML/nvidia-smi) | `pick_gpu_telemetry()` | Compile-time `GpuPlatform` enum                    |
| `MemoryTelemetry`| Read system memory + per-card VRAM/GTT budget        | `pick_memory_telemetry()`| Linux only (compile-time `cfg`)                    |

### Hardware-specific traits (compile-time dispatch)

No runtime traits are purely compile-time. All trait objects are selected once at runtime.
The only `cfg` gates are:
- `platform/lock.rs` — Unix lock (`flock`)
- `platform/paths.rs` — POSIX paths
- GPU telemetry: `GpuPlatform` enum set in `build.rs` determines which impl is compiled.

---

## 3. Compile-Time GPU Platform Dispatch

GPU telemetry is selected at **compile time**, not runtime. This means:

- **Amd builds:** use `AmdGpuTelemetry` (reads `/sys/class/drm/card0/device/gpu_metrics_v2_2` binary blob)
- **Nvidia builds:** use `NvidiaGpuTelemetry` (tries NVML syscalls first, then spawns `nvidia-smi` as a fallback)
- **CPU-only builds:** use `CpuOnlyTelemetry` (always returns `None` — no GPU metrics, memory budget still works)

On CPU-only systems, GPU telemetry always returns `None`, but memory budget remains available to `MemoryTelemetry` (still returns system RAM budget).

```rust
// GpuPlatform is set at compile time via build.rs:
//   cargo:rustc-env=GPU_PLATFORM=AmdGpu  → GpuPlatform::AmdGpu
//   cargo:rustc-env=GPU_PLATFORM=NvidiaGpu → GpuPlatform::NvidiaGpu
//   cargo:rustc-env=GPU_PLATFORM=CpuOnly → GpuPlatform::CpuOnly
pub enum GpuPlatform {
    /// AMD GPU reads /sys/class/drm/card0/device/gpu_metrics_v2_2 (binary blob)
    AmdGpu,
    /// Nvidia GPU (NVML syscalls or nvidia-smi fallback)
    NvidiaGpu,
    /// CPU-only: telemetry always returns None
    CpuOnly,
}

// --- build.rs sets this ---
const GPU_PLATFORM: GpuPlatform = GpuPlatform::AmdGpu; // <- set in build.rs

fn pick_gpu_telemetry() -> Box<dyn GpuTelemetry + 'static> {
    match GPU_PLATFORM {
        GpuPlatform::AmdGpu => Box::new(AmdGpuTelemetry::new()),
        GpuPlatform::NvidiaGpu => Box::new(NvidiaGpuTelemetry::new()),
        GpuPlatform::CpuOnly => Box::new(CpuOnlyTelemetry::new()),
    }
}
```

---

## 4. Runtime Init System Dispatch

Init system actuation is selected at **runtime** (one-time detection that's cached):

```rust
static IS_SYSTEMD: OnceLock<bool> = OnceLock::new();

fn is_systemd() -> bool {
    IS_SYSTEMD.get_or_init(|| {
        let result = std::process::Command::new("systemctl")
            .arg("--version")
            .output();
        result.is_ok()
    })
}

fn pick_actuator() -> Box<dyn Actuator + 'static> {
    if is_systemd() {
        Box::new(SystemdActuator::new())
    } else {
        Box::new(OpenrcActuator::new())
    }
}
```

The detection and caching logic originally lived in `src/init.rs` (`init::is_systemd()`).
It moves into `platform/mod.rs` as the `detect_init_system()` function.

---

## 5. Memory Budget

`MemoryTelemetry` reads:

- System memory from `/proc/meminfo` (parse `MemTotal` and `MemAvailable`)
- Per-card GPU VRAM/GTT from each `/sys/class/drm/cardN/device/mem_info_vram_total` (exclude `cardN-eDP` or `cardN-VDisplay` — those are display outputs, not GPUs)
- Per-card GTT totals (`mem_info_gtt_total`)

This is a trait with one Linux implementation (`PlatformMemoryTelemetry`).
If Windows/macOS ever needs memory budget support, they'd add new impls.

---

## 6. How It Integrates

### Before (current, scattered branching everywhere)

```rust
// swap.rs
if init::is_systemd() {
    // ... systemd logic ...
} else {
    // ... dropin logic ...
}

// nodeops.rs
if matches!(fmt, DropinFormat::Systemd) {
    // ... systemd logic ...
} else {
    // ... openrc logic ...
}
```

### After (delegate to dispatchers)

```rust
let actuator = &mut platform::pick_actuator();
actuator.stage("llama.service", dropin_content, platform::DropinFormat::Systemd)?;
actuator.commit()?;
actuator.restart("llama.service")?;
```

### Telemetry access

```rust
let gpu = platform::pick_gpu_telemetry();
let gpu_data = gpu.read_gpu_telemetry();

let mem = platform::pick_memory_telemetry();
let mem_data = mem.read_memory_budget();
```

On a CPU-only server, `gpu_data` is `None`. But `mem_data` still has the budget.

---

## 7. Implementation Checklist

- [x] `platform/systemd.rs` (systemd Actuator, drop-in staging, commit, restart)
- [x] `platform/openrc.rs` (OpenRC Actuator, conf.d staging, rc-service restart)
- [x] `platform/init.rs` (Init system detection, root/sudo elevation, helpers)
- [x] `platform/mod.rs` (Traits, dispatch selectors, `pick_actuator()`, `is_systemd()`)
- [x] Refactor `src/swap.rs` to use `platform` actuators and init/sudo helpers

---

## 8. No-Code Changes Required

- For a CPU-only server (e.g., netboot host): no GPU required. Memory budget
  still works, telemetry always returns `None` for GPU metrics.
- No new dependencies added. GPU telemetry reads from existing sysfs paths.
- `GpuPlatform::AmdGpu` is the default. NVML syscalls are not compiled unless Nvidia builds, but the nvidia-smi fallback is available.
- The `platform/` directory is the only new code in the repo. All existing code stays in `src/` and calls into the `platform/` module.
