belicose:~/llmtune$ doas ./install.sh
doas (infinitevalence@belicose.endlessdelve.com) password:
>> building release binary...
   Compiling llmtune v0.1.0 (/home/infinitevalence/llmtune)
warning: unused import: `detect_init_system`
 --> src/init.rs:5:57
  |
5 |     is_systemd, service_active, service_ctl, service_stop, detect_init_system,
  |                                                            ^^^^^^^^^^^^^^^^^^
  |
  = note: `#[warn(unused_imports)]` (part of `#[warn(unused)]`) on by default
