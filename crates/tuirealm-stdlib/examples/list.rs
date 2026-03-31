//! `Demo` shows how to use tui-realm in a real case

use std::error::Error;
use std::time::Duration;

use tui_realm_stdlib::components::List;
use tuirealm::application::PollStrategy;
use tuirealm::command::{Cmd, CmdResult, Direction, Position};
use tuirealm::component::{AppComponent, Component};
use tuirealm::event::{Event, Key, KeyEvent, NoUserEvent};
use tuirealm::props::{BorderType, Borders, Color, HorizontalAlignment, Style, Title};
use tuirealm::ratatui::layout::{Constraint, Direction as LayoutDirection, Layout};
use tuirealm::ratatui::style::Stylize;
use tuirealm::ratatui::text::Span;
use tuirealm::terminal::TerminalAdapter;

mod utils;
use utils::Model;

#[derive(Debug, PartialEq)]
pub enum Msg {
    AppClose,
    ListAlfaBlur,
    ListBetaBlur,
    Redraw,
}

// Let's define the component ids for our application
#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum Id {
    ListAlfa,
    ListBeta,
}

impl Model<Id, Msg> {
    /// Draw all components.
    fn view(&mut self) {
        self.terminal
            .draw(|f| {
                // Prepare chunks
                let chunks = Layout::default()
                    .direction(LayoutDirection::Vertical)
                    .margin(1)
                    .constraints(
                        [
                            Constraint::Length(10),
                            Constraint::Length(6),
                            Constraint::Length(1),
                        ]
                        .as_ref(),
                    )
                    .split(f.area());
                self.app.view(&Id::ListAlfa, f, chunks[0]);
                self.app.view(&Id::ListBeta, f, chunks[1]);
            })
            .expect("Drawing to the terminal failed");
    }

    /// Handle messages
    fn update(&mut self, msg: Msg) {
        self.redraw = true;
        match msg {
            Msg::AppClose => {
                self.quit = true;
            }
            Msg::ListAlfaBlur => {
                assert!(self.app.active(&Id::ListBeta).is_ok());
            }
            Msg::ListBetaBlur => {
                assert!(self.app.active(&Id::ListAlfa).is_ok());
            }
            Msg::Redraw => (),
        }
    }

    /// Mount all main components for initial app stage.
    fn mount_main(&mut self) -> Result<(), Box<dyn Error>> {
        self.app
            .mount(Id::ListAlfa, Box::new(ListAlfa::default()), vec![])?;
        self.app
            .mount(Id::ListBeta, Box::new(ListBeta::default()), vec![])?;
        // We need to give focus to input then
        self.app.active(&Id::ListAlfa)?;

        Ok(())
    }
}

fn main() {
    let mut model = Model::new();
    model.mount_main().expect("Mount all main components");

    // let's loop until quit is true
    while !model.quit {
        // Tick
        if let Ok(messages) = model
            .app
            .tick(PollStrategy::Once(Duration::from_millis(10)))
        {
            for msg in messages {
                model.update(msg);
            }
        }
        // Redraw
        if model.redraw {
            model.view();
            model.redraw = false;
        }
    }
}

#[derive(Component)]
struct ListAlfa {
    component: List,
}

impl Default for ListAlfa {
    fn default() -> Self {
        Self {
            component: List::default()
                .borders(
                    Borders::default()
                        .modifiers(BorderType::Rounded)
                        .color(Color::Yellow),
                )
                .title(Title::from("Lorem ipsum (scrollable)").alignment(HorizontalAlignment::Center))
                .scroll(true)
                .highlight_style(Style::new().fg(Color::LightYellow))
                .highlighted_str("🚀")
                .rewind(true)
                .step(4)
                .rows([
                    vec![
                        Span::from("01").fg(Color::Cyan).italic(),
                        Span::from(" "),
                        Span::from("Lorem ipsum dolor sit amet, consectetur adipiscing elit"),
                    ],
                    vec![
                        Span::from("02").fg(Color::Cyan).italic(),
                        Span::from(" "),
                        Span::from("Maecenas tincidunt dui ut gravida fringilla"),
                    ],
                    vec![
                        Span::from("03").fg(Color::Cyan).italic(),
                        Span::from(" "),
                        Span::from("Duis est neque, fringilla sit amet enim id, congue hendrerit mauris"),
                    ],
                    vec![
                        Span::from("04").fg(Color::Cyan).italic(),
                        Span::from(" "),
                        Span::from("Nulla facilisi. Vestibulum tincidunt tempor orci, in pellentesque lacus placerat id."),
                    ],
                    vec![
                        Span::from("05").fg(Color::Cyan).italic(),
                        Span::from(" "),
                        Span::from("Integer at nisl scelerisque, egestas ipsum in, iaculis tellus. Pellentesque tincidunt vestibulum nisi, ut vehicula augue scelerisque at"),
                    ],
                    vec![
                        Span::from("06").fg(Color::Cyan).italic(),
                        Span::from(" "),
                        Span::from("Quisque quis tincidunt tellus. Nam accumsan leo non nunc finibus feugiat."),
                    ],
                    vec![
                        Span::from("07").fg(Color::Cyan).italic(),
                        Span::from(" "),
                        Span::from("non lacus ac orci fermentum aliquam ut feugiat libero. Suspendisse eget nunc in erat molestie egestas eu at massa"),
                    ],
                    vec![
                        Span::from("08").fg(Color::Cyan).italic(),
                        Span::from(" "),
                        Span::from("Donec feugiat dui quis libero ornare, vel sodales mauris ornare."),
                    ],
                    vec![
                        Span::from("09").fg(Color::Cyan).italic(),
                        Span::from(" "),
                        Span::from("Aenean tempor porta nisi, at sodales eros semper ut. Vivamus sit amet commodo risus"),
                    ],
                    vec![
                        Span::from("10").fg(Color::Cyan).italic(),
                        Span::from(" "),
                        Span::from("Etiam urna nisi, ullamcorper at justo et, rhoncus pellentesque dui. Nunc ante velit, ultrices a ornare sit amet, sagittis in ex. Nam pulvinar tellus tortor. Praesent ac accumsan nunc, ac consectetur nisi."),
                    ],
                    vec![
                        Span::from("11").fg(Color::Cyan).italic(),
                        Span::from(" "),
                        Span::from("Proin non elit fermentum, pretium diam eget, facilisis mi"),
                    ],
                    vec![
                        Span::from("12").fg(Color::Cyan).italic(),
                        Span::from(" "),
                        Span::from("Duis suscipit nibh lacus, quis porta enim accumsan vel"),
                    ],
                    vec![
                        Span::from("13").fg(Color::Cyan).italic(),
                        Span::from(" "),
                        Span::from("Etiam volutpat magna tortor, a laoreet ex accumsan sit amet"),
                    ]
                ])
                .selected_line(2),
        }
    }
}

impl AppComponent<Msg, NoUserEvent> for ListAlfa {
    fn on(&mut self, ev: &Event<NoUserEvent>) -> Option<Msg> {
        match ev.as_keyboard()? {
            KeyEvent {
                code: Key::Down, ..
            } => self.perform(Cmd::Move(Direction::Down)),
            KeyEvent { code: Key::Up, .. } => self.perform(Cmd::Move(Direction::Up)),
            KeyEvent {
                code: Key::PageDown,
                ..
            } => self.perform(Cmd::Scroll(Direction::Down)),
            KeyEvent {
                code: Key::PageUp, ..
            } => self.perform(Cmd::Scroll(Direction::Up)),
            KeyEvent {
                code: Key::Home, ..
            } => self.perform(Cmd::GoTo(Position::Begin)),
            KeyEvent { code: Key::End, .. } => self.perform(Cmd::GoTo(Position::End)),
            KeyEvent { code: Key::Tab, .. } => return Some(Msg::ListAlfaBlur),
            KeyEvent { code: Key::Esc, .. } => return Some(Msg::AppClose),
            _ => CmdResult::NoChange,
        };
        Some(Msg::Redraw)
    }
}

#[derive(Component)]
struct ListBeta {
    component: List,
}

impl Default for ListBeta {
    fn default() -> Self {
        Self {
            component: List::default()
                .borders(
                    Borders::default()
                        .modifiers(BorderType::Rounded)
                        .color(Color::Green),
                )
                .foreground(Color::Green)
                .title(Title::from("Lorem ipsum (unscrollable)").alignment(HorizontalAlignment::Center))
                .scroll(false)
                .rows([
                    vec![
                        Span::from("01").fg(Color::Cyan).italic(),
                        Span::from(" "),
                        Span::from("Lorem ipsum dolor sit amet, consectetur adipiscing elit"),
                    ],
                    vec![
                        Span::from("02").fg(Color::Cyan).italic(),
                        Span::from(" "),
                        Span::from("Maecenas tincidunt dui ut gravida fringilla"),
                    ],
                    vec![
                        Span::from("03").fg(Color::Cyan).italic(),
                        Span::from(" "),
                        Span::from("Duis est neque, fringilla sit amet enim id, congue hendrerit mauris"),
                    ],
                    vec![
                        Span::from("04").fg(Color::Cyan).italic(),
                        Span::from(" "),
                        Span::from("Nulla facilisi. Vestibulum tincidunt tempor orci, in pellentesque lacus placerat id."),
                    ],
                    vec![
                        Span::from("05").fg(Color::Cyan).italic(),
                        Span::from(" "),
                        Span::from("Integer at nisl scelerisque, egestas ipsum in, iaculis tellus. Pellentesque tincidunt vestibulum nisi, ut vehicula augue scelerisque at"),
                    ],
                    vec![
                        Span::from("06").fg(Color::Cyan).italic(),
                        Span::from(" "),
                        Span::from("Quisque quis tincidunt tellus. Nam accumsan leo non nunc finibus feugiat."),
                    ],
                    vec![
                        Span::from("07").fg(Color::Cyan).italic(),
                        Span::from(" "),
                        Span::from("non lacus ac orci fermentum aliquam ut feugiat libero. Suspendisse eget nunc in erat molestie egestas eu at massa"),
                    ],
                    vec![
                        Span::from("08").fg(Color::Cyan).italic(),
                        Span::from(" "),
                        Span::from("Donec feugiat dui quis libero ornare, vel sodales mauris ornare."),
                    ],
                    vec![
                        Span::from("09").fg(Color::Cyan).italic(),
                        Span::from(" "),
                        Span::from("Aenean tempor porta nisi, at sodales eros semper ut. Vivamus sit amet commodo risus"),
                    ],
                    vec![
                        Span::from("10").fg(Color::Cyan).italic(),
                        Span::from(" "),
                        Span::from("Etiam urna nisi, ullamcorper at justo et, rhoncus pellentesque dui. Nunc ante velit, ultrices a ornare sit amet, sagittis in ex. Nam pulvinar tellus tortor. Praesent ac accumsan nunc, ac consectetur nisi."),
                    ],
                    vec![
                        Span::from("11").fg(Color::Cyan).italic(),
                        Span::from(" "),
                        Span::from("Proin non elit fermentum, pretium diam eget, facilisis mi"),
                    ],
                    vec![
                        Span::from("12").fg(Color::Cyan).italic(),
                        Span::from(" "),
                        Span::from("Duis suscipit nibh lacus, quis porta enim accumsan vel"),
                    ],
                    vec![
                        Span::from("13").fg(Color::Cyan).italic(),
                        Span::from(" "),
                        Span::from("Etiam volutpat magna tortor, a laoreet ex accumsan sit amet"),
                    ]
                ]),
        }
    }
}

impl AppComponent<Msg, NoUserEvent> for ListBeta {
    fn on(&mut self, ev: &Event<NoUserEvent>) -> Option<Msg> {
        match ev.as_keyboard()? {
            KeyEvent { code: Key::Tab, .. } => return Some(Msg::ListBetaBlur),
            KeyEvent { code: Key::Esc, .. } => return Some(Msg::AppClose),
            _ => CmdResult::NoChange,
        };
        Some(Msg::Redraw)
    }
}
