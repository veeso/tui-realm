//! This Example showcases the use of [`AnyProp`](tuirealm::props::AnyProp) via [`PropPayload`] for use in [`Props`], [`query`](MockComponent::query) and [`attr`](MockComponent::attr).
//!
//! The data structs used in this example are very simple and could be done via other values in [`PropPayload`] / [`AttrValue`],
//! but imagine this for outside sources like [`tuirealm-tree-view`](https://github.com/veeso/tui-realm-treeview)'s Tree data.
//!
//! The main section in this Example is [`StdLabel`] and [`OurLabel`].

use std::time::Duration;

use tuirealm::command::{Cmd, CmdResult};
use tuirealm::event::{Key, KeyEvent};
use tuirealm::props::{Alignment, Color, PropBound, PropPayload, Style, TextModifiers};
use tuirealm::ratatui::layout::{Constraint, Direction, Layout, Rect};
use tuirealm::ratatui::widgets::Paragraph;
use tuirealm::terminal::{CrosstermTerminalAdapter, TerminalAdapter, TerminalBridge};
use tuirealm::{
    Application, AttrValue, Attribute, Component, Event, EventListenerCfg, Frame, MockComponent,
    NoUserEvent, PollStrategy, Props, State, Update,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let event_listener =
        EventListenerCfg::default().crossterm_input_listener(Duration::from_millis(10), 10);

    let mut app: Application<Id, Msg, NoUserEvent> = Application::init(event_listener);

    // subscribe component to clause
    app.mount(Id::Label, Box::new(OurLabel::default()), vec![])?;

    app.active(&Id::Label).expect("failed to active");

    let mut model = Model::new(app, CrosstermTerminalAdapter::new()?);
    // Main loop
    // NOTE: loop until quit; quit is set in update if AppClose is received from counter
    while !model.quit {
        // Tick
        match model.app.tick(PollStrategy::Once) {
            Err(err) => {
                panic!("application error {err}");
            }
            Ok(messages) if !messages.is_empty() => {
                // NOTE: redraw if at least one msg has been processed
                model.redraw = true;
                for msg in messages {
                    let mut msg = Some(msg);
                    while msg.is_some() {
                        msg = model.update(msg);
                    }
                }
            }
            _ => {}
        }
        // Redraw
        if model.redraw {
            model.view();
            model.redraw = false;
        }
    }

    model.terminal.restore()?;

    Ok(())
}

#[derive(Debug, PartialEq)]
pub enum Msg {
    AppClose,
    ForceRedraw,
}

// Let's define the component ids for our application
#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum Id {
    Label,
}

pub struct Model<T>
where
    T: TerminalAdapter,
{
    /// Application
    pub app: Application<Id, Msg, NoUserEvent>,
    /// Indicates that the application must quit
    pub quit: bool,
    /// Tells whether to redraw interface
    pub redraw: bool,
    /// Used to draw to terminal
    pub terminal: TerminalBridge<T>,
}

impl<T> Model<T>
where
    T: TerminalAdapter,
{
    pub fn new(app: Application<Id, Msg, NoUserEvent>, adapter: T) -> Self {
        Self {
            app,
            quit: false,
            redraw: true,
            terminal: TerminalBridge::init(adapter).expect("Cannot initialize terminal"),
        }
    }

    pub fn view(&mut self) {
        assert!(
            self.terminal
                .draw(|f| {
                    let chunks = Layout::default()
                        .direction(Direction::Vertical)
                        .margin(1)
                        .constraints(
                            [
                                Constraint::Length(3), // Label
                            ]
                            .as_ref(),
                        )
                        .split(f.area());
                    self.app.view(&Id::Label, f, chunks[0]);
                })
                .is_ok()
        );
    }
}

// Let's implement Update for model

impl<T> Update<Msg> for Model<T>
where
    T: TerminalAdapter,
{
    fn update(&mut self, msg: Option<Msg>) -> Option<Msg> {
        if let Some(msg) = msg {
            // Set redraw
            self.redraw = true;
            // Match message
            match msg {
                Msg::AppClose => {
                    self.quit = true; // Terminate
                    None
                }
                Msg::ForceRedraw => None,
            }
        } else {
            None
        }
    }
}

/// This is our custom data we want to store on [`Props`] and communicate via [`query`](MockComponent::query) and [`attr`](MockComponent::attr).
#[derive(Debug, Clone, PartialEq)]
struct CustomState {
    text: String,
}

impl Default for CustomState {
    fn default() -> Self {
        Self {
            text: "Default text".to_string(),
        }
    }
}

/// Simple label component; just renders a text
/// NOTE: since I need just one label, I'm not going to use different object; I will directly implement Component for Label.
/// This is not ideal actually and in a real app you should differentiate Mock Components from Application Components.
#[derive(Debug)]
pub struct StdLabel {
    props: Props,
}

impl Default for StdLabel {
    fn default() -> Self {
        let mut props = Props::default();
        props.set(
            Attribute::Value,
            AttrValue::Payload(PropPayload::Any(CustomState::default().to_any_prop())),
        );
        Self { props }
    }
}

impl MockComponent for StdLabel {
    fn view(&mut self, frame: &mut Frame, area: Rect) {
        // Check if visible
        if self.props.get_or(Attribute::Display, AttrValue::Flag(true)) == AttrValue::Flag(true) {
            // Get properties
            let text = self
                .props
                .get_ref(Attribute::Value)
                .and_then(AttrValue::as_payload)
                .and_then(PropPayload::as_any)
                .and_then(|v| v.downcast_ref::<CustomState>())
                .map(|v| v.text.as_str())
                .unwrap_or("Unavailable; this is a bug");
            let alignment = self
                .props
                .get_or(Attribute::TextAlign, AttrValue::Alignment(Alignment::Left))
                .unwrap_alignment();
            let foreground = self
                .props
                .get_or(Attribute::Foreground, AttrValue::Color(Color::Reset))
                .unwrap_color();
            let background = self
                .props
                .get_or(Attribute::Background, AttrValue::Color(Color::Reset))
                .unwrap_color();
            let modifiers = self
                .props
                .get_or(
                    Attribute::TextProps,
                    AttrValue::TextModifiers(TextModifiers::empty()),
                )
                .unwrap_text_modifiers();

            let [chunk1, chunk2] = Layout::new(
                Direction::Vertical,
                [Constraint::Length(1), Constraint::Min(1)],
            )
            .areas(area);

            frame.render_widget(
                Paragraph::new("The following text should be changing when pressing <TAB>:"),
                chunk1,
            );

            frame.render_widget(
                Paragraph::new(text)
                    .style(
                        Style::default()
                            .fg(foreground)
                            .bg(background)
                            .add_modifier(modifiers),
                    )
                    .alignment(alignment),
                chunk2,
            );
        }
    }

    fn query(&self, attr: Attribute) -> Option<AttrValue> {
        self.props.get(attr)
    }

    fn attr(&mut self, attr: Attribute, value: AttrValue) {
        self.props.set(attr, value);
    }

    fn state(&self) -> State {
        State::None
    }

    fn perform(&mut self, _: Cmd) -> CmdResult {
        CmdResult::None
    }
}

#[derive(Debug, MockComponent, Default)]
struct OurLabel {
    component: StdLabel,
}

impl Component<Msg, NoUserEvent> for OurLabel {
    fn on(&mut self, ev: &Event<NoUserEvent>) -> Option<Msg> {
        // Does nothing
        match ev {
            Event::Keyboard(KeyEvent { code: Key::Esc, .. }) => Some(Msg::AppClose),
            Event::Keyboard(KeyEvent { code: Key::Tab, .. }) => {
                let mut existing_attr = self.query(Attribute::Value).unwrap_or_else(|| {
                    AttrValue::Payload(PropPayload::Any(CustomState::default().to_any_prop()))
                });
                let tmp = existing_attr
                    .as_payload_mut()
                    .and_then(|v| v.as_any_mut())
                    .and_then(|v| v
                    .downcast_mut::<CustomState>())
                    .expect("Unexpected type in Attribute::Value! Expected PropPayload::Any + CustomState!");
                tmp.text = match tmp.text.as_str() {
                    "Default text" => "Some other text".to_string(),
                    _ => CustomState::default().text,
                };
                self.attr(Attribute::Value, existing_attr);
                Some(Msg::ForceRedraw)
            }
            _ => None,
        }
    }
}
