# maulingmonkey-console-escape-codes

Unstable API for [ANSI/VT100 escape codes](https://docs.microsoft.com/en-us/windows/console/console-virtual-terminal-sequences).

<!--
[![GitHub](https://img.shields.io/github/stars/MaulingMonkey/maulingmonkey-console-escape-codes.svg?label=GitHub&style=social)](https://github.com/MaulingMonkey/maulingmonkey-console-escape-codes)
[![crates.io](https://img.shields.io/crates/v/maulingmonkey-console-escape-codes.svg)](https://crates.io/crates/maulingmonkey-console-escape-codes)
[![docs.rs](https://docs.rs/maulingmonkey-console-escape-codes/badge.svg)](https://docs.rs/maulingmonkey-console-escape-codes)
[![License](https://img.shields.io/crates/l/maulingmonkey-console-escape-codes.svg)](https://github.com/MaulingMonkey/maulingmonkey-console-escape-codes)
[![Build Status](https://github.com/MaulingMonkey/maulingmonkey-console-escape-codes/workflows/Rust/badge.svg)](https://github.com/MaulingMonkey/maulingmonkey-console-escape-codes/actions?query=workflow%3Arust)
-->
<!-- [![dependency status](https://deps.rs/repo/github/MaulingMonkey/maulingmonkey-console-escape-codes/status.svg)](https://deps.rs/repo/github/MaulingMonkey/maulingmonkey-console-escape-codes) -->



## Quickstart

```toml
# Cargo.toml
[dependencies]
maulingmonkey-console-escape-codes.git = "https://github.com/MaulingMonkey/console-escape-codes"
maulingmonkey-console-winapi-wrappers.git = "https://github.com/MaulingMonkey/console-winapi-wrappers"
```

```rust
// src\main.rs
use maulingmonkey_console_escape_codes::*;
use maulingmonkey_console_winapi_wrappers::*;

fn main() {
    #[cfg(windows)] let _ = change_console_mode(&mut std::io::stdout(), |m| m | ENABLE_VIRTUAL_TERMINAL_PROCESSING);
    println!(
        "Hello, {green}{bold}world{reset}!",
        green = vt100::sgr_foreground_256(2),
        bold  = vt100::sgr_bold(true),
        reset = vt100::sgr_default(),
    );
}
```



<h2 name="license">License</h2>

Licensed under either of

* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.



<h2 name="contribution">Contribution</h2>

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
