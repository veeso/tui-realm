use std::sync::Arc;
use std::time::Duration;

use tempfile::NamedTempFile;
use tokio::io::AsyncWriteExt as _;
use tokio::runtime::Handle;
use tuirealm::command::{Cmd, CmdResult};
use tuirealm::event::{Key, KeyEvent};
use tuirealm::listener::{ListenerResult, PollAsync};
use tuirealm::props::{Alignment, Color, Style, TextModifiers};
use tuirealm::ratatui::layout::{Constraint, Direction, Layout, Rect};
use tuirealm::ratatui::widgets::Paragraph;
use tuirealm::terminal::{CrosstermTerminalAdapter, TerminalAdapter, TerminalBridge};
use tuirealm::{
    Application, AttrValue, Attribute, Component, Event, EventListenerCfg, Frame, MockComponent,
    PollStrategy, Props, State, Sub, SubClause, SubEventClause, Update,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let handle = Handle::current();

    let event_listener = EventListenerCfg::default()
        .with_handle(handle)
        .async_crossterm_input_listener(Duration::default(), 3)
        .add_async_port(Box::new(AsyncPort::new()), Duration::from_millis(1000), 1);

    let mut app: Application<Id, Msg, UserEvent> = Application::init(event_listener);

    // subscribe component to clause
    app.mount(
        Id::Label,
        Box::new(Label::default()),
        vec![Sub::new(
            SubEventClause::User(UserEvent::WroteFile(Duration::ZERO)),
            SubClause::Always,
        )],
    )?;

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
    None,
}

// Let's define the component ids for our application
#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum Id {
    Label,
}

#[derive(Debug, Eq, Clone, Copy, PartialOrd, Ord)]
pub enum UserEvent {
    WroteFile(Duration),
    None,
}

impl PartialEq for UserEvent {
    fn eq(&self, other: &Self) -> bool {
        std::mem::discriminant(self) == std::mem::discriminant(other)
    }
}

pub struct Model<T>
where
    T: TerminalAdapter,
{
    /// Application
    pub app: Application<Id, Msg, UserEvent>,
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
    pub fn new(app: Application<Id, Msg, UserEvent>, adapter: T) -> Self {
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
                Msg::None => None,
            }
        } else {
            None
        }
    }
}

struct AsyncPort {
    tempfile: Arc<NamedTempFile>,
}

impl AsyncPort {
    pub fn new() -> Self {
        let tempfile = Arc::new(NamedTempFile::new().unwrap());
        Self { tempfile }
    }

    pub async fn write_file(&self) -> Duration {
        let t_start = std::time::Instant::now();
        // Write to file
        let mut file = tokio::fs::File::create(self.tempfile.path()).await.unwrap();
        file.write_all(b"Hello, world!").await.unwrap();

        t_start.elapsed()
    }
}

#[tuirealm::async_trait]
impl PollAsync<UserEvent> for AsyncPort {
    async fn poll(&mut self) -> ListenerResult<Option<Event<UserEvent>>> {
        let result = self.write_file().await;

        Ok(Some(Event::User(UserEvent::WroteFile(result))))
    }
}

/// Simple label component; just renders a text
/// NOTE: since I need just one label, I'm not going to use different object; I will directly implement Component for Label.
/// This is not ideal actually and in a real app you should differentiate Mock Components from Application Components.
#[derive(Default)]
pub struct Label {
    props: Props,
}

impl MockComponent for Label {
    fn view(&mut self, frame: &mut Frame, area: Rect) {
        // Check if visible
        if self.props.get_or(Attribute::Display, AttrValue::Flag(true)) == AttrValue::Flag(true) {
            // Get properties
            let text = self
                .props
                .get_or(Attribute::Text, AttrValue::String(String::default()))
                .unwrap_string();
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
            frame.render_widget(
                Paragraph::new(text)
                    .style(
                        Style::default()
                            .fg(foreground)
                            .bg(background)
                            .add_modifier(modifiers),
                    )
                    .alignment(alignment),
                area,
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

impl Component<Msg, UserEvent> for Label {
    fn on(&mut self, ev: Event<UserEvent>) -> Option<Msg> {
        // Does nothing
        match ev {
            Event::Keyboard(KeyEvent { code: Key::Esc, .. }) => Some(Msg::AppClose),
            Event::User(UserEvent::WroteFile(duration)) => {
                // set text
                self.attr(
                    Attribute::Text,
                    AttrValue::String(format!("file wrote in {} nanos", duration.as_nanos())),
                );

                Some(Msg::None)
            }
            _ => None,
        }
    }
}
