//! ## Demo
//!
//! `Demo` shows how to use tui-realm in a real case

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
extern crate tuirealm;

use std::time::{Duration, SystemTime};
use tuirealm::application::PollStrategy;
use tuirealm::props::{Alignment, Color, TextModifiers};
use tuirealm::{
    event::NoUserEvent, Application, AttrValue, Attribute, EventListenerCfg, Sub, SubClause,
    SubEventClause,
};
// -- internal
mod app;
mod components;
use app::model::Model;
use components::{Clock, DigitCounter, Label, LetterCounter};

// Let's define the messages handled by our app. NOTE: it must derive `PartialEq`
#[derive(Debug, PartialEq)]
pub enum Msg {
    AppClose,
    Clock,
    DigitCounterChanged(isize),
    DigitCounterBlur,
    LetterCounterChanged(isize),
    LetterCounterBlur,
}

// Let's define the component ids for our application
#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum Id {
    Clock,
    DigitCounter,
    LetterCounter,
    Label,
}

fn main() {
    // Setup model
    let mut model = Model::default();
    // Setup application
    // NOTE: NoUserEvent is a shorthand to tell tui-realm we're not going to use any custom user event
    // NOTE: the event listener is configured to use the default crossterm input listener and to raise a Tick event each second
    // which we will use to update the clock
    let mut app: Application<Id, Msg, NoUserEvent> = Application::init(
        EventListenerCfg::default()
            .default_input_listener(Duration::from_millis(20))
            .poll_timeout(Duration::from_millis(10))
            .tick_interval(Duration::from_secs(1)),
    );

    // Mount components
    assert!(app
        .mount(
            Id::Label,
            Box::new(
                Label::default()
                    .text("Waiting for a Msg...")
                    .alignment(Alignment::Left)
                    .background(Color::Reset)
                    .foreground(Color::LightYellow)
                    .modifiers(TextModifiers::BOLD),
            ),
            Vec::default(),
        )
        .is_ok());
    // Mount clock, subscribe to tick
    assert!(app
        .mount(
            Id::Clock,
            Box::new(
                Clock::new(SystemTime::now())
                    .alignment(Alignment::Center)
                    .background(Color::Reset)
                    .foreground(Color::Cyan)
                    .modifiers(TextModifiers::BOLD)
            ),
            vec![Sub::new(SubEventClause::Tick, SubClause::Always)]
        )
        .is_ok());
    // Mount counters
    assert!(app
        .mount(
            Id::LetterCounter,
            Box::new(LetterCounter::new(0)),
            Vec::new()
        )
        .is_ok());
    assert!(app
        .mount(
            Id::DigitCounter,
            Box::new(DigitCounter::new(5)),
            Vec::default()
        )
        .is_ok());
    // Active letter counter
    assert!(app.active(&Id::LetterCounter).is_ok());
    // Enter alternate screen
    let _ = model.terminal.enter_alternate_screen();
    let _ = model.terminal.enable_raw_mode();
    // Main loop
    // NOTE: loop until quit; quit is set in update if AppClose is received from counter
    while !model.quit {
        // Tick
        match app.tick(&mut model, PollStrategy::Once) {
            Err(err) => {
                assert!(app
                    .attr(
                        &Id::Label,
                        Attribute::Text,
                        AttrValue::String(format!("Application error: {}", err)),
                    )
                    .is_ok());
            }
            Ok(sz) if sz > 0 => {
                // NOTE: redraw if at least one msg has been processed
                model.redraw = true;
            }
            _ => {}
        }
        // Redraw
        if model.redraw {
            model.view(&mut app);
            model.redraw = false;
        }
    }
    // Terminate terminal
    let _ = model.terminal.leave_alternate_screen();
    let _ = model.terminal.disable_raw_mode();
    let _ = model.terminal.clear_screen();
}
