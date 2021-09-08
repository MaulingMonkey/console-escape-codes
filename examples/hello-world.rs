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
