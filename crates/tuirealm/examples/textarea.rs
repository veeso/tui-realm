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
mod utils;

use utils::context::Context;
use utils::keymap::*;

use std::thread::sleep;
use std::time::{Duration, Instant};

use tuirealm::components::{label, textarea};
use tuirealm::props::borders::{BorderType, Borders};
use tuirealm::props::TextSpan;
use tuirealm::{Msg, PropsBuilder, Update, View};
// tui
use tui::layout::{Constraint, Direction, Layout};
use tui::style::Color;

const COMPONENT_TEXTAREA: &str = "textarea";
const COMPONENT_TEXTAREA_2: &str = "textarea2";
const COMPONENT_EVENT: &str = "LABEL";

struct Model {
    quit: bool,           // Becomes true when the user presses <ESC>
    redraw: bool,         // Tells whether to refresh the UI; performance optimization
    last_redraw: Instant, // Last time the ui has been redrawed
    view: View,
}

impl Model {
    fn new(view: View) -> Self {
        Model {
            quit: false,
            redraw: true,
            last_redraw: Instant::now(),
            view,
        }
    }

    fn quit(&mut self) {
        self.quit = true;
    }

    fn redraw(&mut self) {
        self.redraw = true;
    }

    fn reset(&mut self) {
        self.redraw = false;
        self.last_redraw = Instant::now();
    }
}

fn main() {
    // let's create a context: the context contains the backend of crossterm and the input handler
    let mut ctx: Context = Context::new();
    // Enter alternate screen
    ctx.enter_alternate_screen();
    // Clear screen
    ctx.clear_screen();
    // let's create a `View`, which will contain the components
    let mut myview: View = View::init();
    // Mount the component you need; we'll use a Label and an Input
    myview.mount(
        COMPONENT_TEXTAREA,
        Box::new(textarea::Textarea::new(
            textarea::TextareaPropsBuilder::default()
                .with_foreground(Color::White)
                .italic()
                .with_borders(Borders::ALL, BorderType::Rounded, Color::LightRed)
                .with_highlighted_str(Some("🎵"))
                .with_max_scroll_step(2)
                .with_title("Scrollable textarea")
                .with_texts(
                    vec![
                        TextSpan::new("About TermSCP").bold().underlined().fg(Color::Yellow),
                        TextSpan::from("TermSCP is basically a porting of WinSCP to terminal. So basically is a terminal utility with an TUI to connect to a remote server to retrieve and upload files and to interact with the local file system. It works both on Linux, MacOS, BSD and Windows and supports SFTP, SCP, FTP and FTPS."),
                        TextSpan::new("Why TermSCP 🤔").bold().underlined().fg(Color::Cyan),
                        TextSpan::from("It happens quite often to me, when using SCP at work to forget the path of a file on a remote machine, which forces me to connect through SSH, gather the file path and finally download it through SCP. I could use WinSCP, but I use Linux and I pratically use the terminal for everything, so I wanted something like WinSCP on my terminal. Yeah, I know there is midnight commander too, but actually I don't like it very much tbh (and hasn't a decent support for scp)."),
                        TextSpan::from("Lorde ~ Green Light"),
                        TextSpan::from("I do my makeup in somebody else's car We order different drinks at the same bars I know about what you did and I wanna scream the truth She thinks you love the beach, you're such a damn liar ")
                    ]
                )
                .build(),
        )),
    );
    myview.mount(
        COMPONENT_TEXTAREA_2,
        Box::new(textarea::Textarea::new(
            textarea::TextareaPropsBuilder::default()
                .with_foreground(Color::Black)
                .with_foreground(Color::White)
                .italic()
                .with_borders(Borders::ALL, BorderType::Rounded, Color::LightRed)
                .with_highlighted_str(Some("🎵"))
                .with_title("Scrollble textarea")
                .with_texts(
                    vec![
                        TextSpan::new("About TermSCP").bold().underlined().fg(Color::Yellow),
                        TextSpan::from("TermSCP is basically a porting of WinSCP to terminal. So basically is a terminal utility with an TUI to connect to a remote server to retrieve and upload files and to interact with the local file system. It works both on Linux, MacOS, BSD and Windows and supports SFTP, SCP, FTP and FTPS."),
                        TextSpan::new("Why TermSCP 🤔").bold().underlined().fg(Color::Cyan),
                        TextSpan::from("It happens quite often to me, when using SCP at work to forget the path of a file on a remote machine, which forces me to connect through SSH, gather the file path and finally download it through SCP. I could use WinSCP, but I use Linux and I pratically use the terminal for everything, so I wanted something like WinSCP on my terminal. Yeah, I know there is midnight commander too, but actually I don't like it very much tbh (and hasn't a decent support for scp)."),
                        TextSpan::from("Lorde ~ Green Light"),
                        TextSpan::from("I do my makeup in somebody else's car We order different drinks at the same bars I know about what you did and I wanna scream the truth She thinks you love the beach, you're such a damn liar ")
                    ]
                )
                .build(),
        )),
    );
    myview.mount(
        COMPONENT_EVENT,
        Box::new(label::Label::new(
            label::LabelPropsBuilder::default()
                .with_foreground(Color::Cyan)
                .build(),
        )),
    );
    // We need to give focus to input then
    myview.active(COMPONENT_TEXTAREA);
    // Now we use the Model struct to keep track of some states
    let mut model: Model = Model::new(myview);
    // let's loop until quit is true
    while !model.quit {
        // Listen for input events
        if let Ok(Some(ev)) = ctx.input_hnd.read_event() {
            // Pass event to view
            let msg = model.view.on(ev);
            model.redraw();
            // Call the elm friend update
            model.update(msg);
        }
        // If redraw, draw interface
        if model.redraw || model.last_redraw.elapsed() > Duration::from_millis(50) {
            // Call the elm friend vie1 function
            view(&mut ctx, &model.view);
            model.reset();
        }
        sleep(Duration::from_millis(10));
    }
    // Let's drop the context finally
    drop(ctx);
}

fn view(ctx: &mut Context, view: &View) {
    let _ = ctx.terminal.draw(|f| {
        // Prepare chunks
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints(
                [
                    Constraint::Length(10),
                    Constraint::Length(6),
                    Constraint::Length(1),
                ]
                .as_ref(),
            )
            .split(f.size());
        view.render(COMPONENT_TEXTAREA, f, chunks[0]);
        view.render(COMPONENT_TEXTAREA_2, f, chunks[1]);
        view.render(COMPONENT_EVENT, f, chunks[2]);
    });
}

impl Update for Model {
    fn update(&mut self, msg: Option<(String, Msg)>) -> Option<(String, Msg)> {
        let ref_msg: Option<(&str, &Msg)> = msg.as_ref().map(|(s, msg)| (s.as_str(), msg));
        match ref_msg {
            None => None, // Exit after None
            Some(msg) => match msg {
                (COMPONENT_TEXTAREA, &MSG_KEY_TAB) => {
                    self.view.active(COMPONENT_TEXTAREA_2);
                    None
                }
                (COMPONENT_TEXTAREA_2, &MSG_KEY_TAB) => {
                    self.view.active(COMPONENT_TEXTAREA);
                    None
                }
                (_, &MSG_KEY_ESC) => {
                    // Quit on esc
                    self.quit();
                    None
                }
                (component, event) => {
                    // Update span
                    let props = label::LabelPropsBuilder::from(
                        self.view.get_props(COMPONENT_EVENT).unwrap(),
                    )
                    .with_text(format!("{} => '{:?}'", component, event))
                    .build();
                    // Report submit
                    let _ = self.view.update(COMPONENT_EVENT, props);
                    None
                }
            },
        }
    }
}
