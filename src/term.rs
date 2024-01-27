use super::config::Colorname;
use super::errors::FxError;

use crossterm::cursor::{Hide, MoveLeft, MoveRight, MoveTo, Show};
use crossterm::style::{Color, ResetColor, SetBackgroundColor, SetForegroundColor};
use crossterm::terminal::Clear;

pub enum TermColor<'a> {
    ForeGround(&'a Colorname),
    BackGround(&'a Colorname),
}

/// Puts the terminal into raw mode. Requires calling `leave_raw_mode` on program exit!
///
/// **Warning!** Not calling `leave_raw_mode` will leave *nix terminals in an unusable state!
///
/// Changing the underlying terminal to raw mode is needed to allow for direct input.
/// This change is not undone automatically on program exit and must be managed gracefully to avoid
/// leaving the user with a broken terminal.
///
pub fn enter_raw_mode() {
    crossterm::terminal::enable_raw_mode().ok();
    hide_cursor();
}

pub fn leave_raw_mode() {
    show_cursor();
    crossterm::terminal::disable_raw_mode().ok();
}

pub fn terminal_size() -> Result<(u16, u16), FxError> {
    crossterm::terminal::size().map_err(|_| FxError::TerminalSizeDetection)
}

pub fn cursor_pos() -> Result<(u16, u16), FxError> {
    Ok(crossterm::cursor::position()?)
}

pub fn move_to(x: u16, y: u16) {
    print!("{}", MoveTo(x - 1, y - 1));
}

pub fn to_info_line() {
    move_to(2, 2);
}

pub fn clear_current_line() {
    print!("{}", Clear(crossterm::terminal::ClearType::CurrentLine));
}

pub fn go_to_info_line_and_reset() {
    to_info_line();
    clear_current_line();
}

pub fn clear_until_newline() {
    print!("{}", Clear(crossterm::terminal::ClearType::UntilNewLine));
}

pub fn clear_all() {
    print!("{}", Clear(crossterm::terminal::ClearType::All));
}

pub fn move_left(x: u16) {
    print!("{}", MoveLeft(x));
}

pub fn move_left_command_line(
    input: &mut [char],
    current_char_pos: &mut usize,
    current_pos: &mut u16,
) {
    if current_char_pos == &0 {
        return;
    };
    if let Some(to_be_skipped) =
        unicode_width::UnicodeWidthChar::width(input[*current_char_pos - 1])
    {
        *current_char_pos -= 1;
        *current_pos -= to_be_skipped as u16;
        move_left(to_be_skipped as u16);
    }
}

pub fn move_right(x: u16) {
    print!("{}", MoveRight(x));
}

pub fn move_right_command_line(
    input: &mut [char],
    current_char_pos: &mut usize,
    current_pos: &mut u16,
) {
    if *current_char_pos == input.len() {
        return;
    };
    if let Some(to_be_skipped) = unicode_width::UnicodeWidthChar::width(input[*current_char_pos]) {
        *current_char_pos += 1;
        *current_pos += to_be_skipped as u16;
        move_right(to_be_skipped as u16);
    }
}

pub fn hide_cursor() {
    print!("{}", Hide);
}

pub fn show_cursor() {
    print!("{}", Show);
}

pub fn print_pointer() {
    print!(">");
    move_left(1);
}

pub fn delete_pointer() {
    print!(" ");
    move_left(1);
}

pub fn set_color(c: &TermColor) {
    match c {
        TermColor::ForeGround(c) => match c {
            Colorname::Black => print!("{}", SetForegroundColor(Color::Black)),
            Colorname::Red => print!("{}", SetForegroundColor(Color::DarkRed)),
            Colorname::Green => print!("{}", SetForegroundColor(Color::DarkGreen)),
            Colorname::Yellow => print!("{}", SetForegroundColor(Color::DarkYellow)),
            Colorname::Blue => print!("{}", SetForegroundColor(Color::DarkBlue)),
            Colorname::Magenta => print!("{}", SetForegroundColor(Color::DarkMagenta)),
            Colorname::Cyan => print!("{}", SetForegroundColor(Color::DarkCyan)),
            Colorname::White => print!("{}", SetForegroundColor(Color::Grey)),
            Colorname::LightBlack => print!("{}", SetForegroundColor(Color::DarkGrey)),
            Colorname::LightRed => print!("{}", SetForegroundColor(Color::Red)),
            Colorname::LightGreen => print!("{}", SetForegroundColor(Color::Green)),
            Colorname::LightYellow => print!("{}", SetForegroundColor(Color::Yellow)),
            Colorname::LightBlue => print!("{}", SetForegroundColor(Color::Blue)),
            Colorname::LightMagenta => print!("{}", SetForegroundColor(Color::Magenta)),
            Colorname::LightCyan => print!("{}", SetForegroundColor(Color::Cyan)),
            Colorname::LightWhite => print!("{}", SetForegroundColor(Color::White)),
            Colorname::Rgb(r, g, b) => print!(
                "{}",
                SetForegroundColor(Color::Rgb {
                    r: *r,
                    g: *g,
                    b: *b
                })
            ),
            Colorname::AnsiValue(n) => print!("{}", SetForegroundColor(Color::AnsiValue(*n))),
        },
        TermColor::BackGround(c) => match c {
            Colorname::Black => print!("{}", SetBackgroundColor(Color::Black)),
            Colorname::Red => print!("{}", SetBackgroundColor(Color::DarkRed)),
            Colorname::Green => print!("{}", SetBackgroundColor(Color::DarkGreen)),
            Colorname::Yellow => print!("{}", SetBackgroundColor(Color::DarkYellow)),
            Colorname::Blue => print!("{}", SetBackgroundColor(Color::DarkBlue)),
            Colorname::Magenta => print!("{}", SetBackgroundColor(Color::DarkMagenta)),
            Colorname::Cyan => print!("{}", SetBackgroundColor(Color::DarkCyan)),
            Colorname::White => print!("{}", SetBackgroundColor(Color::Grey)),
            Colorname::LightBlack => print!("{}", SetBackgroundColor(Color::DarkGrey)),
            Colorname::LightRed => print!("{}", SetBackgroundColor(Color::Red)),
            Colorname::LightGreen => print!("{}", SetBackgroundColor(Color::Green)),
            Colorname::LightYellow => print!("{}", SetBackgroundColor(Color::Yellow)),
            Colorname::LightBlue => print!("{}", SetBackgroundColor(Color::Blue)),
            Colorname::LightMagenta => print!("{}", SetBackgroundColor(Color::Magenta)),
            Colorname::LightCyan => print!("{}", SetBackgroundColor(Color::Cyan)),
            Colorname::LightWhite => print!("{}", SetBackgroundColor(Color::White)),
            Colorname::Rgb(r, g, b) => print!(
                "{}",
                SetBackgroundColor(Color::Rgb {
                    r: *r,
                    g: *g,
                    b: *b
                })
            ),
            Colorname::AnsiValue(n) => print!("{}", SetBackgroundColor(Color::AnsiValue(*n))),
        },
    }
}

pub fn set_color_current_dir() {
    set_color(&TermColor::ForeGround(&Colorname::Cyan));
}

pub fn set_color_read_only() {
    set_color(&TermColor::ForeGround(&Colorname::Red));
}

pub fn set_color_git_repo() {
    set_color(&TermColor::ForeGround(&Colorname::LightMagenta));
}

pub fn reset_color() {
    print!("{}", ResetColor);
}
