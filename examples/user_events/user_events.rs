mod components;
mod model;

use std::time::{Duration, SystemTime};

use tuirealm::{
    listener::{ListenerResult, Poll},
    Application, Event, EventListenerCfg, PollStrategy, Sub, SubClause, SubEventClause, Update,
};

use components::Label;

use crate::model::Model;

#[derive(Debug, PartialEq)]
pub enum Msg {
    AppClose,
    None,
}

// Let's define the component ids for our application
#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum Id {
    Label,
    Other,
}

#[derive(Debug, Eq, Clone, Copy, PartialOrd, Ord)]
pub enum UserEvent {
    GotData(SystemTime),
    None,
}

impl PartialEq for UserEvent {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (UserEvent::GotData(_), UserEvent::GotData(_)) => true,
            _ => false,
        }
    }
}

#[derive(Default)]
struct UserDataPort;

impl Poll<UserEvent> for UserDataPort {
    fn poll(&mut self) -> ListenerResult<Option<Event<UserEvent>>> {
        ListenerResult::Ok(Some(Event::User(UserEvent::GotData(SystemTime::now()))))
    }
}

fn main() {
    let mut app: Application<Id, Msg, UserEvent> = Application::init(
        EventListenerCfg::default()
            .default_input_listener(Duration::from_millis(10))
            .port(
                Box::new(UserDataPort::default()),
                Duration::from_millis(1000),
            ),
    );

    // subscribe component to clause
    app.mount(
        Id::Label,
        Box::new(Label::default()),
        vec![Sub::new(
            SubEventClause::User(UserEvent::GotData(SystemTime::UNIX_EPOCH)),
            SubClause::Always,
        )],
    )
    .expect("failed to mount");
    app.mount(
        Id::Other,
        Box::new(Label::default()),
        vec![Sub::new(
            SubEventClause::User(UserEvent::GotData(SystemTime::UNIX_EPOCH)),
            SubClause::Always,
        )],
    )
    .expect("failed to mount");

    app.active(&Id::Label).expect("failed to active");

    let mut model = Model::new(app);
    let _ = model.terminal.enter_alternate_screen();
    let _ = model.terminal.enable_raw_mode();
    // Main loop
    // NOTE: loop until quit; quit is set in update if AppClose is received from counter
    while !model.quit {
        // Tick
        match model.app.tick(PollStrategy::Once) {
            Err(err) => {
                panic!("application error {err}");
            }
            Ok(messages) if messages.len() > 0 => {
                // NOTE: redraw if at least one msg has been processed
                model.redraw = true;
                for msg in messages.into_iter() {
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
    // Terminate terminal
    let _ = model.terminal.leave_alternate_screen();
    let _ = model.terminal.disable_raw_mode();
    let _ = model.terminal.clear_screen();
}
