# COMPLETE — Diff Summary (src/swap.rs)

## What Changed

### New: Init system detection

Added three new functions/vars to detect whether the host runs systemd vs OpenRC:

| Line | Item | Notes |
|------|------|-------|
| 19-26 | `detect_init_system()` | Runs `systemctl --version`; success = systemd, failure = OpenRC |
| 29-31 | `is_openrc()` | Returns `!is_systemd()` |
| 34-46 | `IS_SYSTEMD` + `is_systemd()` | Cached global bool, set on first call to `is_systemd()` |

### Formatting: spaces → tabs

The remainder of the diff (all sections below) is whitespace normalization: every block converted from 4-space indentation to tab indentation. No logic changed.

## What Still Needs OpenRC Migration

*(All systemd references beyond what's been addressed. This is the HANDOFF.md items that remain actionable.)*

### swap.rs — Core systemd integration (highest impact) — REMAINING

### Constants — NOT REMEDIATED

These constants still hardcode systemd paths. The diff added a comment but left the values untouched.

| Line | Constant | Notes |
|------|----------|-------|
| 507 | `ETC_UNIT_DIR` | Still `"/etc/systemd/system"` → needs `"/etc/llmtune"` or OpenRC equivalent |
| 510 | `RUN_UNIT_DIR` | Still `"/run/systemd/system"` → needs OpenRC runtime dir |
| 512 | `VENDOR_UNIT_DIRS` | Still sysvend dirs → needs OpenRC vendor locations |

### Structs / Types

| Line | Item | Notes |
|------|------|-------|
| 919 | `SystemdActuator` | Implements `Actuator` trait → rename to `OpenrcActuator` |
| 920 | `SystemdActuator::staged_dropin` | Path of the drop-in |
| 922 | `SystemdActuator::prior_dropin` | Prior drop-in for rollback |

### Drop-in rendering

| Line | Function | Notes |
|------|------|-------|
| 75 | `render_dropin()` | Outputs systemd INI format → needs OpenRC rc-service format |
| 148 | `collect_base_env()` | Extracts systemd `Environment=` → no equivalent in OpenRC |
| 168 | `render_env_line()` | `Environment=` line format → OpenRC `export` lines |
| 201 | `render_kv_lines()` | systemd-style quoted assignments → OpenRC env format |

### Drop-in path / scan functions

| Line | Function | Notes |
|------|------|-------|
| 518 | `unit_install_dir()` | Returns `ETC_UNIT_DIR` or `RUN_UNIT_DIR` → needs OpenRC dirs |
| 580 | `find_dropin()` | Searches all drop-in roots systemd merges → OpenRC uses different dirs |
| 595 | `unit_dropin_roots()` | Collects all drop-in dirs systemd merges → `[/etc/openrc]` |
| 632 | `collect_dropins()` | Union scan across all systemd drop-in dirs → OpenRC scan |
| 651 | `classify_dropins()` | Classifies llmtune-owned vs foreign → same logic, different paths |
| 674 | `separate_dropins()` | Splits llmtune-owned from foreign → same logic |
| 697 | `read_dropin()` | Reads drop-in content → same logic |
| 716 | `dropin_filename()` | Filename guaranteed to sort last → OpenRC ordering |

### Functions that call systemd binaries

| Line | Function | Command(s) | Notes |
|------|----------|----------|-------|
| 185 | `unit_base_env()` | `systemctl show` → `rc-service <unit> show` equivalent |
| 801 | `sudo()` | Wraps `systemctl`, `kill`, etc. → needs OpenRC wrappers |
| 943 | `Actuator::stage()` | `systemctl daemon-reload`, `restart` → `rc-service <unit> restart` |
| 988 | `Actuator::rollback()` | `systemctl daemon-reload`, `restart` → OpenRC restart |
| 494 | `uninstall_dropins()` | `systemctl daemon-reload`, `reset-failed`, `restart` → OpenRC cleanup |

### Comments / Docs referencing systemd (all files)

See HANDOFF.md tables for exact lines. Every mention of systemd, systemctl, journald, drop-in, vendor dirs, NixOS `/run/systemd/system`, `systemd-run`, `systemctl enable`, `systemctl is-active`, `systemctl stop/start/restart/daemon-reload/reset-failed` is actionable.

### Tests referencing systemd

All test assertions on `systemctl` args, drop-in paths (`/etc/systemd/system/**/*.d`), journald assertions, NixOS `/nix/store` paths → need OpenRC equivalents or be skipped when `!is_systemd()`.
