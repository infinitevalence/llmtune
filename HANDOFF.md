belicose:~/llmtune$ doas ./install.sh
doas (infinitevalence@belicose.endlessdelve.com) password:
>> building release binary...
Compiling llmtune v0.1.0 (/home/infinitevalence/llmtune)
warning: unused import: `bail`
--> src/swap.rs:12:14
	|
 12 | use anyhow::{bail, Result};
	|              ^^^^
	|
= note: `#[warn(unused_imports)]` (part of `#[warn(unused)]`) on by default

warning: unused import: `std::io::Write`
--> src/swap.rs:14:5
	|
 14 | use std::io::Write;
	|     ^^^^^^^^^^^^^

warning: unused import: `Stdio`
--> src/swap.rs:16:29
	|
 16 | use std::process::{Command, Stdio};
	|                             ^^^^^

warning: unused import: `std::sync::OnceLock`
--> src/../platform/mod.rs:9:5
	|
  9 | use std::sync::OnceLock;
	|     ^^^^^^^^^^^^^^^^^^^

warning: unused imports: `MemoryBudget` and `TelemetryData`
--> src/../platform/telemetry.rs:4:23
	|
  4 | use crate::platform::{MemoryBudget, TelemetryData};
	|                       ^^^^^^^^^^^^  ^^^^^^^^^^^^^

warning: use of deprecated type alias `libc::time_t`: This type is changed to 64-bit in musl 1.2.0, we'll follow that change in the future release. See #1848 for more info.
--> src/proxy.rs:452:32
	|
452 |         tv_sec: t.as_secs() as libc::time_t,
	|                                      ^^^^^^
	|
= note: `#[warn(deprecated)]` on by default

warning: use of deprecated type alias `libc::suseconds_t`: This type is changed to 64-bit in musl 1.2.0, we'll follow that change in the future release. See #1848 for more info.
--> src/proxy.rs:453:39
	|
453 |         tv_usec: t.subsec_micros() as libc::suseconds_t,
	|                                            ^^^^^^^^^^^

warning: unused variable: `unit`
--> src/../platform/paths.rs:37:27
	|
 37 |     fn write_api_key_file(unit: &str, api_key: &str) -> Result<PathBuf> {
	|                           ^^^^ help: if this is intentional, prefix it with an underscore: `_unit`
	|
= note: `#[warn(unused_variables)]` (part of `#[warn(unused)]`) on by default

warning: function `collect_dropins` is never used
--> src/swap.rs:814:4
	|
814 | fn collect_dropins(unit: &str) -> (Vec<PathBuf>, Vec<String>) {
	|    ^^^^^^^^^^^^^^^
	|
= note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: function `winning_name` is never used
--> src/swap.rs:900:4
	|
900 | fn winning_name(foreign: &[String]) -> String {
	|    ^^^^^^^^^^^^

warning: trait `GpuTelemetry` is never used
--> src/../platform/mod.rs:40:11
	|
 40 | pub trait GpuTelemetry {
	|           ^^^^^^^^^^^^

warning: trait `MemoryTelemetry` is never used
--> src/../platform/mod.rs:47:11
	|
 47 | pub trait MemoryTelemetry {
	|           ^^^^^^^^^^^^^^^

warning: trait `Lock` is never used
--> src/../platform/mod.rs:65:11
	|
 65 | pub trait Lock {
	|           ^^^^

warning: trait `PathResolver` is never used
--> src/../platform/mod.rs:72:11
	|
 72 | pub trait PathResolver {
	|           ^^^^^^^^^^^^

warning: enum `GpuPlatform` is never used
--> src/../platform/mod.rs:87:10
	|
 87 | pub enum GpuPlatform {
	|          ^^^^^^^^^^^

warning: function `detect_init_system` is never used
--> src/../platform/mod.rs:104:8
	|
104 | pub fn detect_init_system() -> InitSystemType {
	|        ^^^^^^^^^^^^^^^^^^

warning: function `is_systemd` is never used
--> src/../platform/mod.rs:112:8
	|
112 | pub fn is_systemd() -> bool {
	|        ^^^^^^^^^^

warning: function `pick_actuator` is never used
--> src/../platform/mod.rs:116:8
	|
116 | pub fn pick_actuator() -> Box<dyn Actuator + 'static> {
	|        ^^^^^^^^^^^^^

warning: function `pick_lock` is never used
--> src/../platform/mod.rs:124:8
	|
124 | pub fn pick_lock() -> Box<dyn Lock + 'static> {
	|        ^^^^^^^^^

warning: function `pick_path_resolver` is never used
--> src/../platform/mod.rs:128:8
	|
128 | pub fn pick_path_resolver() -> &'static dyn PathResolver {
	|        ^^^^^^^^^^^^^^^^^^

warning: function `pick_gpu_telemetry` is never used
--> src/../platform/mod.rs:136:8
	|
136 | pub fn pick_gpu_telemetry() -> Box<dyn GpuTelemetry + 'static> {
	|        ^^^^^^^^^^^^^^^^^^
warning: function `pick_memory_telemetry` is never used

--> src/../platform/mod.rs:144:8
	|
144 | pub fn pick_memory_telemetry() -> Box<dyn MemoryTelemetry + 'static> {
	|        ^^^^^^^^^^^^^^^^^^^^^

warning: enum `InitSystemType` is never used
--> src/../platform/mod.rs:153:10
	|
153 | pub enum InitSystemType {
	|          ^^^^^^^^^^^^^^

warning: struct `TelemetryData` is never constructed
--> src/../platform/mod.rs:165:12
	|
165 | pub struct TelemetryData {
	|            ^^^^^^^^^^^^^
warning: struct `MemoryBudget` is never constructed

--> src/../platform/mod.rs:178:12
	|
178 | pub struct MemoryBudget {
	|            ^^^^^^^^^^^^

warning: enum `LockResult` is never used
--> src/../platform/mod.rs:187:10
	|
187 | pub enum LockResult {
	|          ^^^^^^^^^^

warning: struct `AmdGpuTelemetry` is never constructed
--> src/../platform/amdgpu.rs:7:12
	|
  7 | pub struct AmdGpuTelemetry;
	|            ^^^^^^^^^^^^^^^

warning: associated function `new` is never used
--> src/../platform/amdgpu.rs:10:12
	|
  9 | impl AmdGpuTelemetry {
	| -------------------- associated function in this implementation
 10 |     pub fn new() -> Self {
	|            ^^^

warning: struct `NvidiaGpuTelemetry` is never constructed
--> src/../platform/nvidia.rs:7:12
	|
  7 | pub struct NvidiaGpuTelemetry;
	|            ^^^^^^^^^^^^^^^^^^

warning: associated function `new` is never used
--> src/../platform/nvidia.rs:10:12
	|
  9 | impl NvidiaGpuTelemetry {
	| ----------------------- associated function in this implementation
 10 |     pub fn new() -> Self {
	|            ^^^

warning: struct `CpuOnlyTelemetry` is never constructed
--> src/../platform/cpuonly.rs:7:12
	|
  7 | pub struct CpuOnlyTelemetry;
	|            ^^^^^^^^^^^^^^^^

warning: associated function `new` is never used
--> src/../platform/cpuonly.rs:10:12
	|
  9 | impl CpuOnlyTelemetry {
	| --------------------- associated function in this implementation
 10 |     pub fn new() -> Self {
	|            ^^^

warning: struct `PlatformMemoryTelemetry` is never constructed
--> src/../platform/memory.rs:6:12
	|
  6 | pub struct PlatformMemoryTelemetry;
	|            ^^^^^^^^^^^^^^^^^^^^^^^

warning: associated function `new` is never used
--> src/../platform/memory.rs:9:12
	|
  8 | impl PlatformMemoryTelemetry {
	| ---------------------------- associated function in this implementation
  9 |     pub fn new() -> Self {
	|            ^^^
warning: function `meminfo_parser` is never used

--> src/../platform/memory.rs:29:4
	|
 29 | fn meminfo_parser(text: &str) -> Option<(u64, u64)> {
	|    ^^^^^^^^^^^^^^

warning: static `IS_SYSTEMD` is never used
--> src/../platform/init.rs:11:8
	|
 11 | static IS_SYSTEMD: OnceLock<bool> = OnceLock::new();
	|        ^^^^^^^^^^

warning: function `is_systemd` is never used
--> src/../platform/init.rs:14:8
	|
 14 | pub fn is_systemd() -> bool {
	|        ^^^^^^^^^^

warning: function `service_active` is never used
--> src/../platform/init.rs:27:8
	|
 27 | pub fn service_active(unit: &str) -> bool {
	|        ^^^^^^^^^^^^^^

warning: function `service_ctl` is never used
--> src/../platform/init.rs:44:8
	|
 44 | pub fn service_ctl(unit: &str, action: &str) {
	|        ^^^^^^^^^^^

warning: function `service_stop` is never used
--> src/../platform/init.rs:58:8
	|
 58 | pub fn service_stop(unit: &str) {
	|        ^^^^^^^^^^^^

warning: function `detect_init_system` is never used
--> src/../platform/init.rs:62:8
	|
 62 | pub fn detect_init_system() -> &'static str {
	|        ^^^^^^^^^^^^^^^^^^

warning: function `sudo_failure_message` is never used
--> src/../platform/init.rs:169:8
	|
169 | pub fn sudo_failure_message(
	|        ^^^^^^^^^^^^^^^^^^^^

warning: constant `LLMTUNE_PREFIX` is never used
--> src/../platform/systemd.rs:73:7
	|
 73 | const LLMTUNE_PREFIX: &str = "AUTO-GENERATED by llmtune";
	|       ^^^^^^^^^^^^^^

warning: struct `PlatformLock` is never constructed
--> src/../platform/lock.rs:8:12
	|
  8 | pub struct PlatformLock {
	|            ^^^^^^^^^^^^

warning: associated function `new` is never used
--> src/../platform/lock.rs:13:12
	|
 12 | impl PlatformLock {
	| ----------------- associated function in this implementation
 13 |     pub fn new() -> Self {
	|            ^^^

warning: struct `PlatformPathResolver` is never constructed
--> src/../platform/paths.rs:8:12
	|
  8 | pub struct PlatformPathResolver;
	|            ^^^^^^^^^^^^^^^^^^^^

warning: associated function `new` is never used
--> src/../platform/paths.rs:11:12
	|
 10 | impl PlatformPathResolver {
	| ------------------------- associated function in this implementation
 11 |     pub fn new() -> Self {
	|            ^^^

warning: function `parse_meminfo` is never used
--> src/../platform/telemetry.rs:6:8
	|
  6 | pub fn parse_meminfo(text: &str) -> Option<(u32, u32)> {
	|        ^^^^^^^^^^^^^

warning: `llmtune` (bin "llmtune") generated 48 warnings (run `cargo fix --bin "llmtune" -p llmtune` to apply 6 suggestions)
Finished `release` profile [optimized] target(s) in 2m 34s
>> installing /usr/local/bin/llmtune (doas)...
doas (root@belicose.endlessdelve.com) password:
>> installed: llmtune 0.1.0

Done. Launch the TUI:      llmtune
preflight the box:       llmtune doctor
first-run setup:  doas llmtune setup
add and serve a model:   llmtune models add <url> && llmtune node load <name>
belicose:~/llmtune$ llmtune setup
== preflight (localhost) ==
[ok]   BC-250 APU       PCI 1002:13fe present
[ok]   amdgpu           module loaded
[ok]   DRM render node  /dev/dri/renderD* present
[ok]   Vulkan ICD       ICD manifest dir present
[ok]   build toolchain  compiler, cmake, git, Vulkan headers and a shader compiler present
[ok]   models dir       /var/lib/llmtune/models - 1 model(s)
[ok]   profile bins     all launch binaries present
[warn] rpc-server       build(s) without rpc-server: prism-vulkan - this node cannot join a cluster; rebuild with RPC: `llmtune build install prism-vulkan`
[warn] crash-safe fallback missing - run `llmtune setup` (prevents crash-loops when no model is loaded)
[warn] crash-loop guard missing - run `llmtune setup` (a bad model could crash-loop and wedge the GPU)
[ok]   llama-server     http://127.0.0.1:8080 healthy, serving none

[ok]   models dir /var/lib/llmtune/models exists

Models directory: /var/lib/llmtune/models
First-time setup creates it (owned by you): run `llmtune setup`.
Then drop GGUF model files here - llmtune auto-detects `*.gguf` by architecture:
huggingface-cli download unsloth/Qwen3-8B-GGUF Qwen3-8B-Q4_K_M.gguf --local-dir /var/lib/llmtune/models
# or just copy/move any .gguf into that folder
Put models on a bigger disk: set `models_dir` in ~/.config/llmtune/fleet.toml, export $LLMTUNE_MODELS_DIR, or symlink /var/lib/llmtune/models at an existing folder.

[ok]   unit llama-server.service already present (/etc/init.d/llama-server)
/usr/bin/doas
doas (infinitevalence@belicose.endlessdelve.com) password:
error: doas ["systemctl", "daemon-reload"] failed (exit status: 1): doas: systemctl: command not found
                                                                              	                	                   	         	        	        	        	        	          	           	           	           	           	           	           	           	           	           	           	               	   	              	   	              	   	           	  	       	                	        	        	        	        	          	               	       	   	             	   }
                                                                              	                	                   	         	        	        	        	        	          	           	           	           	           	           	           	           	           	           	           	               	   	              	   	              	   	           	  	       	                	        	        	        	        	          	               	       	   	             }
                                                                              	                	                   	         	        	        	        	        	          	           	           	           	           	           	           	           	           	           	           	               	   	              	   	              	   	           	  	       	                	        	        	        	        	          	               	       	   }
                                                                              	                	                   	         	        	        	        	        	          	           	           	           	           	           	           	           	           	           	           	               	   	              	   	              	   	           	  	       	                	        	        	        	        	          	               	       }
                                                                              	                	                   	         	        	        	        	        	          	           	           	           	           	           	           	           	           	           	           	               	   	              	   	              	   	           	  	       	                	        	        	        	        	          	               }
                                                                              	                	                   	         	        	        	        	        	          	           	           	           	           	           	           	           	           	           	           	               	   	              	   	              	   	           	  	       	                	        	        	        	        	          )
                                                                              	                	                   	         	        	        	        	        	          	           	           	           	           	           	           	           	           	           	           	               	   	              	   	              	   	           	  	       	                	        	        	        	        }'
                                                                              	                	                   	         	        	        	        	        	          	           	           	           	           	           	           	           	           	           	           	               	   	              	   	              	   	           	  	       	                	        	        	        }
                                                                              	                	                   	         	        	        	        	        	          	           	           	           	           	           	           	           	           	           	           	               	   	              	   	              	   	           	  	       	                	        	        }
                                                                              	                	                   	         	        	        	        	        	          	           	           	           	           	           	           	           	           	           	           	               	   	              	   	              	   	           	  	       	                	        }
                                                                              	                	                   	         	        	        	        	        	          	           	           	           	           	           	           	           	           	           	           	               	   	              	   	              	   	           	  	       	                }
                                                                              	                	                   	         	        	        	        	        	          	           	           	           	           	           	           	           	           	           	           	               	   	              	   	              	   	           	  	       }
                                                                              	                	                   	         	        	        	        	        	          	           	           	           	           	           	           	           	           	           	           	               	   	              	   	              	   	           	  }
                                                                              	                	                   	         	        	        	        	        	          	           	           	           	           	           	           	           	           	           	           	               	   	              	   	              	   	           }
                                                                              	                	                   	         	        	        	        	        	          	           	           	           	           	           	           	           	           	           	           	               	   	              	   	              	   }
                                                                              	                	                   	         	        	        	        	        	          	           	           	           	           	           	           	           	           	           	           	               	   	              	   	              }
                                                                              	                	                   	         	        	        	        	        	          	           	           	           	           	           	           	           	           	           	           	               	   	              	   }
                                                                              	                	                   	         	        	        	        	        	          	           	           	           	           	           	           	           	           	           	           	               	   	              }
                                                                              	                	                   	         	        	        	        	        	          	           	           	           	           	           	           	           	           	           	           	               	   }
                                                                              	                	                   	         	        	        	        	        	          	           	           	           	           	           	           	           	           	           	           	               }
                                                                              	                	                   	         	        	        	        	        	          	           	           	           	           	           	           	           	           	           	           }
                                                                              	                	                   	         	        	        	        	        	          	           	           	           	           	           	           	           	           	           }
                                                                              	                	                   	         	        	        	        	        	          	           	           	           	           	           	           	           	           }
                                                                              	                	                   	         	        	        	        	        	          	           	           	           	           	           	           	           }
                                                                              	                	                   	         	        	        	        	        	          	           	           	           	           	           	           }'
                                                                              	                	                   	         	        	        	        	        	          	           	           	           	           	           }'
                                                                              	                	                   	         	        	        	        	        	          	           	           	           	           }'
                                                                              	                	                   	         	        	        	        	        	          	           	           	           }'
                                                                              	                	                   	         	        	        	        	        	          	           	           }'
                                                                              	                	                   	         	        	        	        	        	          	           }
                                                                              	                	                   	         	        	        	        	        	          }
                                                                              	                	                   	         	        	        	        	        }
                                                                              	                	                   	         	        	        	        }
                                                                              	                	                   	         	        	        }
                                                                              	                	                   	         	        }
                                                                              	                	                   	         }
                                                                              	                	                   }
                                                                              	                }
                                                                              }
