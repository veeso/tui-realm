use std::sync::Arc;
use std::time::Duration;

use tempfile::NamedTempFile;
use tokio::io::AsyncWriteExt as _;
use tokio::runtime::Handle;
use tuirealm::application::{Application, PollStrategy};
use tuirealm::command::{Cmd, CmdResult};
use tuirealm::component::{AppComponent, Component};
use tuirealm::event::{Event, Key, KeyEvent};
use tuirealm::listener::{EventListenerCfg, PollAsync, PortResult};
use tuirealm::props::{
    AttrValue, Attribute, Color, HorizontalAlignment, Props, QueryResult, Style,
};
use tuirealm::ratatui::Frame;
use tuirealm::ratatui::layout::{Constraint, Direction, Layout, Rect};
use tuirealm::ratatui::widgets::Paragraph;
use tuirealm::state::State;
use tuirealm::subscription::{EventClause, Sub, SubClause};
use tuirealm::terminal::{CrosstermTerminalAdapter, TerminalAdapter, TerminalResult};

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
            EventClause::User(UserEvent::WroteFile(Duration::ZERO)),
            SubClause::Always,
        )],
    )?;

    app.active(&Id::Label).expect("failed to active");

    let mut model = Model::new(app)?;
    // Main loop
    // NOTE: loop until quit; quit is set in update if AppClose is received from counter
    while !model.quit {
        // Tick
        match model
            .app
            .tick(PollStrategy::Once(Duration::from_millis(10)))
        {
            Err(err) => {
                panic!("application error {err}");
            }
            Ok(messages) if !messages.is_empty() => {
                for msg in messages {
                    model.update(msg);
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
    Redraw,
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

pub struct Model {
    /// Application
    pub app: Application<Id, Msg, UserEvent>,
    /// Indicates that the application must quit
    pub quit: bool,
    /// Tells whether to redraw interface
    pub redraw: bool,
    /// Used to draw to terminal
    pub terminal: CrosstermTerminalAdapter,
}

impl Model {
    fn init_adapter() -> TerminalResult<CrosstermTerminalAdapter> {
        let mut adapter = CrosstermTerminalAdapter::new()?;
        adapter.enable_raw_mode()?;
        adapter.enter_alternate_screen()?;

        Ok(adapter)
    }

    pub fn new(app: Application<Id, Msg, UserEvent>) -> TerminalResult<Self> {
        Ok(Self {
            app,
            quit: false,
            redraw: true,
            terminal: Self::init_adapter()?,
        })
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

impl Model {
    fn update(&mut self, msg: Msg) {
        // Set redraw
        self.redraw = true;
        // Match message
        match msg {
            Msg::AppClose => {
                self.quit = true; // Terminate
            }
            Msg::Redraw => (),
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
    async fn poll(&mut self) -> PortResult<Option<Event<UserEvent>>> {
        let result = self.write_file().await;

        Ok(Some(Event::User(UserEvent::WroteFile(result))))
    }
}

/// Simple label component; just renders a text
/// NOTE: since I need just one label, I'm not going to use different object; I will directly implement Component for Label.
/// This is not ideal actually and in a real app you should differentiate Components from Application Components.
#[derive(Default)]
pub struct Label {
    props: Props,
}

impl Component for Label {
    fn view(&mut self, frame: &mut Frame, area: Rect) {
        // Check if visible
        if matches!(
            self.props.get(Attribute::Display),
            Some(AttrValue::Flag(false))
        ) {
            return;
        }

        // Get properties
        let text = self
            .props
            .get(Attribute::Text)
            .and_then(AttrValue::as_string)
            .map(String::as_str)
            .unwrap_or_default();
        let alignment = self
            .props
            .get(Attribute::TextAlign)
            .and_then(AttrValue::as_alignment_horizontal)
            .unwrap_or(HorizontalAlignment::Left);
        let foreground = self
            .props
            .get(Attribute::Foreground)
            .and_then(AttrValue::as_color)
            .unwrap_or(Color::Reset);
        let background = self
            .props
            .get(Attribute::Background)
            .and_then(AttrValue::as_color)
            .unwrap_or(Color::Reset);
        let modifiers = self
            .props
            .get(Attribute::TextProps)
            .and_then(AttrValue::as_text_modifiers)
            .unwrap_or_default();
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

    fn query<'a>(&'a self, attr: Attribute) -> Option<QueryResult<'a>> {
        self.props.get_for_query(attr)
    }

    fn attr(&mut self, attr: Attribute, value: AttrValue) {
        self.props.set(attr, value);
    }

    fn state(&self) -> State {
        State::None
    }

    fn perform(&mut self, cmd: Cmd) -> CmdResult {
        CmdResult::Invalid(cmd)
    }
}

impl AppComponent<Msg, UserEvent> for Label {
    fn on(&mut self, ev: &Event<UserEvent>) -> Option<Msg> {
        // Does nothing
        match ev {
            Event::Keyboard(KeyEvent { code: Key::Esc, .. }) => Some(Msg::AppClose),
            Event::User(UserEvent::WroteFile(duration)) => {
                // set text
                self.attr(
                    Attribute::Text,
                    AttrValue::String(format!("file wrote in {} nanos", duration.as_nanos())),
                );

                Some(Msg::Redraw)
            }
            _ => None,
        }
    }
}
