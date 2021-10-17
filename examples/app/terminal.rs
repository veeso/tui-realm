//! ## Terminal
//!
//! terminal helper

/**
 * MIT License
 *
 * tui-realm - Copyright (C) 2021 Christian Visintin
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

// Includes
#[cfg(target_family = "unix")]
use crossterm::{
    event::DisableMouseCapture,
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io::{stdout, Stdout};
use tuirealm::tui::backend::CrosstermBackend;
use tuirealm::Terminal;

pub fn init() -> Terminal {
    Terminal::new(CrosstermBackend::new(get_stdout())).unwrap()
}

/// ### enter_alternate_screen
///
/// Enter alternate screen (gui window)
#[cfg(target_family = "unix")]
#[allow(dead_code)]
pub fn enter_alternate_screen(t: &mut Terminal) {
    if let Err(err) = execute!(t.backend_mut(), EnterAlternateScreen, DisableMouseCapture) {
        panic!("Failed to enter alternate screen: {}", err);
    }
}

/// ### enter_alternate_screen
///
/// Enter alternate screen (gui window)
#[allow(dead_code)]
#[cfg(target_family = "windows")]
pub fn enter_alternate_screen(_t: &mut Terminal) {}

/// ### leave_alternate_screen
///
/// Go back to normal screen (gui window)
#[cfg(target_family = "unix")]
pub fn leave_alternate_screen(t: &mut Terminal) {
    if let Err(err) = execute!(t.backend_mut(), LeaveAlternateScreen, DisableMouseCapture) {
        panic!("Failed to leave alternate screen: {}", err);
    }
}

/// ### leave_alternate_screen
///
/// Go back to normal screen (gui window)
#[cfg(target_family = "windows")]
pub fn leave_alternate_screen(_t: &mut Terminal) {}

/// ### clear_screen
///
/// Clear terminal screen
pub fn clear_screen(t: &mut Terminal) {
    let _ = t.clear();
}

#[cfg(target_family = "unix")]
fn get_stdout() -> Stdout {
    let mut stdout = stdout();
    assert!(execute!(stdout, EnterAlternateScreen).is_ok());
    stdout
}

#[cfg(target_family = "windows")]
fn get_stdout() -> Stdout {
    stdout()
}
