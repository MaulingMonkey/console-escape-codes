#![doc = include_str!("../doc/vt100.md")]

use crate::*;
use std::fmt::Display;



// Simple Cursor Positioning
// https://docs.microsoft.com/en-us/windows/console/console-virtual-terminal-sequences#simple-cursor-positioning

/// Reverse index - like `\n`, but goes up one line instead of down
pub fn reverse_index() -> impl Display { "\x1BM" }

/// Save cursor position & attributes
pub fn save_cursor_position_attributes() -> impl Display { "\x1B7" }

/// Restore cursor position & attributes
pub fn restore_cursor_position_attributes() -> impl Display { "\x1B8" }




// Cursor Positioning
// https://docs.microsoft.com/en-us/windows/console/console-virtual-terminal-sequences#cursor-positioning

/// Move cursor up by `r` rows
pub fn cursor_up(rows: impl Unsigned) -> impl Display { display!("\x1B[{}A", rows) }

/// Move cursor down by `r` rows
pub fn cursor_down(rows: impl Unsigned) -> impl Display { display!("\x1B[{}B", rows) }

/// Move cursor forward by `c` columns
pub fn cursor_forward(columns: impl Unsigned) -> impl Display { display!("\x1B[{}C", columns) }

/// Move cursor backward by `c` columns
pub fn cursor_backward(columns: impl Unsigned) -> impl Display { display!("\x1B[{}D", columns) }

/// Move cursor to start of line `r` rows down
pub fn cursor_next_line(rows: impl Unsigned) -> impl Display { display!("\x1B[{}E", rows) }

/// Move cursor to start of line `r` rows up
pub fn cursor_previous_line(rows: impl Unsigned) -> impl Display { display!("\x1B[{}F", rows) }

/// Move cursor to row `r`
pub fn cursor_horizontal_absolute(row: impl Into<RowNo>) -> impl Display { let row = row.into(); display!("\x1B[{}G", row) }

/// Move cursor to column `c`
pub fn vertical_position_absolute(column: impl Into<ColNo>) -> impl Display { let column = column.into(); display!("\x1B[{}d", column) }

/// Move cursor to row `r`, column `c`
pub fn cursor_position(pos: impl Into<RowColNo>) -> impl Display { let RowColNo(row, column) = pos.into(); display!("\x1B[{};{}H", row, column) }

/// Move cursor to row `r`, column `c`
pub fn horizontal_vertical_position(pos: impl Into<RowColNo>) -> impl Display { let RowColNo(row, column) = pos.into(); display!("\x1B[{};{}H", row, column) }

/// Save cursor position
pub fn save_cursor() -> impl Display { "\x1B[s" }

/// Restore cursor position
pub fn restore_cursor() -> impl Display { "\x1B[u" }



// Cursor Visibility
// https://docs.microsoft.com/en-us/windows/console/console-virtual-terminal-sequences#cursor-visibility

/// Enable/disable text cursor blinking
pub fn cursor_blinking(enable: bool) -> impl Display { display!("\x1B[?12{}", if enable { "h" } else { "l" }) }

/// Enable/disable text cursor show
pub fn cursor_show(enable: bool) -> impl Display { display!("\x1B[?25{}", if enable { "h" } else { "l" }) }



// Viewport Positioning
// https://docs.microsoft.com/en-us/windows/console/console-virtual-terminal-sequences#viewport-positioning

/// Scroll text up / viewport down by `<rows>`
pub fn scroll_text_up(rows: impl Unsigned) -> impl Display { display!("\x1B[{}S", rows) }

/// Scroll text down / viewport up by `<rows>`
pub fn scroll_text_down(rows: impl Unsigned) -> impl Display { display!("\x1B[{}T", rows) }



// Text Modification
// https://docs.microsoft.com/en-us/windows/console/console-virtual-terminal-sequences#text-modification

/// Insert `n` spaces at the current cursor position, shifting all existing text to the right. Text exiting the screen to the right is removed.
pub fn insert_character(n: impl Unsigned) -> impl Display { display!("\x1B[{}@", n) }

/// Delete `n` characters at the current cursor position, shifting in space characters from the right edge of the screen.
pub fn delete_character(n: impl Unsigned) -> impl Display { display!("\x1B[{}P", n) }

/// Erase `n` characters from the current cursor position by overwriting them with a space character.
pub fn erase_character(n: impl Unsigned) -> impl Display { display!("\x1B[{}X", n) }

/// Inserts `n` lines into the buffer at the cursor position. The line the cursor is on, and lines below it, will be shifted downwards.
pub fn insert_line(n: impl Unsigned) -> impl Display { display!("\x1B[{}L", n) }

/// Deletes `n` lines from the buffer, starting with the row the cursor is on.
pub fn delete_line(n: impl Unsigned) -> impl Display { display!("\x1B[{}M", n) }

/// Replace all text in the current viewport/screen specified by `pos` with space characters
pub fn erase_in_display(pos: impl BeforeAfterAllCursor) -> impl Display { display!("\x1B[{}J", pos.n012()) }

/// Replace all text on the line with the cursor specified by `pos` with space characters
pub fn erase_in_line(pos: impl BeforeAfterAllCursor) -> impl Display { display!("\x1B[{}K", pos.n012()) }



// Text Formatting
// https://docs.microsoft.com/en-us/windows/console/console-virtual-terminal-sequences#text-formatting

pub fn sgr_default  () -> impl Display { "\x1B[0m" }
pub fn sgr_bold     (enable: bool) -> impl Display { display!("\x1B[{}m", if enable { "1" } else { "22" }) }
pub fn sgr_underline(enable: bool) -> impl Display { display!("\x1B[{}m", if enable { "4" } else { "24" }) }
pub fn sgr_negative (enable: bool) -> impl Display { display!("\x1B[{}m", if enable { "7" } else { "27" }) }
// TODO: 3/4-bit colors



// Extended Colors
// https://docs.microsoft.com/en-us/windows/console/console-virtual-terminal-sequences#extended-colors

pub fn sgr_foreground_rgb(rgb: impl Into<RGB>) -> impl Display { let RGB(r,g,b) = rgb.into(); display!("\x1B[38;2;{};{};{}m", r, g, b) }
pub fn sgr_background_rgb(rgb: impl Into<RGB>) -> impl Display { let RGB(r,g,b) = rgb.into(); display!("\x1B[48;2;{};{};{}m", r, g, b) }
pub fn sgr_foreground_256(pal: u8) -> impl Display { display!("\x1B[38;5;{}m", pal) }
pub fn sgr_background_256(pal: u8) -> impl Display { display!("\x1B[48;5;{}m", pal) }



// Screen Colors
// https://docs.microsoft.com/en-us/windows/console/console-virtual-terminal-sequences#screen-colors

pub fn set_screen_color(pal: u8, rgb: impl Into<RGB>) -> impl Display { let RGB(r,g,b) = rgb.into(); display!("\x1B]4;{};rgb:{:x}/{:x}/{:x}\x1B", pal, r, g, b) }



// Mode Changes
// https://docs.microsoft.com/en-us/windows/console/console-virtual-terminal-sequences#mode-changes

pub fn keypad_application_mode() -> impl Display { "\x1B=" }

pub fn keypad_numeric_mode() -> impl Display { "\x1B>" }

pub fn cursor_keys_application_mode(enable: bool) -> impl Display { display!("\x1B[?1{}", if enable { "h" } else { "l" }) }



// Query State
// https://docs.microsoft.com/en-us/windows/console/console-virtual-terminal-sequences#query-state

pub fn query_cursor_position() -> impl Display { "\x1B[6n" }

pub fn query_device_attributes() -> impl Display { "\x1B[0c" }



// Tabs
// https://docs.microsoft.com/en-us/windows/console/console-virtual-terminal-sequences#tabs

/// Set a tab stop in the cursor's current column
pub fn tab_set(_: Cursor) -> impl Display { "\x1Bh" }

/// Moves the cursor forwards to the next tab stop
pub fn tab_forward(tabs: impl Unsigned) -> impl Display { display!("\x1B[{}l", tabs) }

/// Moves the cursor backwards to the previous tab stop
pub fn tab_backward(tabs: impl Unsigned) -> impl Display { display!("\x1B[{}Z", tabs) }

/// Clears the tab stop in the cursor's current column, or at all columns
pub fn tab_clear(tabs: impl CursorOrAll) -> impl Display { display!("{}", if tabs.is_all() { "\x1B[3g" } else { "\x1B[0g" }) }



// Designate Character Set
// https://docs.microsoft.com/en-us/windows/console/console-virtual-terminal-sequences#designate-character-set

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CharacterSet(&'static str);
pub const DEC_LINE_DRAWING  : CharacterSet = CharacterSet("\x1B(0");
pub const US_ASCII          : CharacterSet = CharacterSet("\x1B(B");

pub fn designate_character_set(cs: CharacterSet) -> impl Display { display!("{}", cs.0) }



// Scrolling Margins
// https://docs.microsoft.com/en-us/windows/console/console-virtual-terminal-sequences#scrolling-margins

pub fn set_scrolling_region(range: std::ops::Range<impl Unsigned>) -> impl Display { display!("\x1B[{};{}r", range.start, range.end) }



// Window Title
// https://docs.microsoft.com/en-us/windows/console/console-virtual-terminal-sequences#window-title

/// Sets the console window's title to `title`.
pub fn set_window_title(title: impl Display) -> impl Display { display!("\x1B]2;{}\x07", title) }



// Alternate Screen Buffer
// https://docs.microsoft.com/en-us/windows/console/console-virtual-terminal-sequences#alternate-screen-buffer

pub fn alternate_screen_buffer() -> impl Display { "\x1B[?1049h" }

pub fn main_screen_buffer() -> impl Display { "\x1B[?1049l" }



// Window Width
// https://docs.microsoft.com/en-us/windows/console/console-virtual-terminal-sequences#window-width

pub fn set_columns_132() -> impl Display { "\x1B[?3h" }

pub fn set_columns_80() -> impl Display { "\x1B[?3l" }



// Soft Reset
// https://docs.microsoft.com/en-us/windows/console/console-virtual-terminal-sequences#soft-reset

/// Reset cursor position/attributes, key modes, margins, charsets, graphics rendition, etc.
pub fn soft_reset() -> impl Display { "\x1B[!p" }



/// Shorthand identifiers
pub mod short {
    #[doc(inline)] pub use super::reverse_index as ri;
    #[doc(inline)] pub use super::save_cursor_position_attributes as decsc;
    #[doc(inline)] pub use super::restore_cursor_position_attributes as decsr;
    #[doc(inline)] pub use super::cursor_up as cuu;
    #[doc(inline)] pub use super::cursor_down as cud;
    #[doc(inline)] pub use super::cursor_forward as cuf;
    #[doc(inline)] pub use super::cursor_backward as cub;
    #[doc(inline)] pub use super::cursor_next_line as cnl;
    #[doc(inline)] pub use super::cursor_previous_line as cpl;
    #[doc(inline)] pub use super::cursor_horizontal_absolute as cha;
    #[doc(inline)] pub use super::vertical_position_absolute as vpa;
    #[doc(inline)] pub use super::cursor_position as cup;
    #[doc(inline)] pub use super::horizontal_vertical_position as hvp;
    #[doc(inline)] pub use super::save_cursor as ansisyssc;
    #[doc(inline)] pub use super::restore_cursor as ansisysrc;

    #[doc(inline)] pub use super::cursor_blinking as att160;
    #[doc(inline)] pub use super::cursor_show as dectcem;

    #[doc(inline)] pub use super::scroll_text_up as su;
    #[doc(inline)] pub use super::scroll_text_down as sd;

    #[doc(inline)] pub use super::insert_character as ich;
    #[doc(inline)] pub use super::delete_character as dch;
    #[doc(inline)] pub use super::erase_character as ech;
    #[doc(inline)] pub use super::insert_line as il;
    #[doc(inline)] pub use super::delete_line as dl;
    #[doc(inline)] pub use super::erase_in_display as ed;
    #[doc(inline)] pub use super::erase_in_line as el;

    #[doc(inline)] pub use super::keypad_application_mode as deckpam;
    #[doc(inline)] pub use super::keypad_application_mode as deckpnm;
    #[doc(inline)] pub use super::cursor_keys_application_mode as decckm;

    #[doc(inline)] pub use super::query_cursor_position as decxcpr;
    #[doc(inline)] pub use super::query_device_attributes as da;

    #[doc(inline)] pub use super::tab_set as hts;
    #[doc(inline)] pub use super::tab_forward as cht;
    #[doc(inline)] pub use super::tab_backward as cbt;
    #[doc(inline)] pub use super::tab_clear as tbc;

    #[doc(inline)] pub use super::set_scrolling_region as decstbm;

    #[doc(inline)] pub use super::set_columns_132 as deccolm_132;
    #[doc(inline)] pub use super::set_columns_80 as deccolm_80;

    #[doc(inline)] pub use super::soft_reset as decstr;
}
