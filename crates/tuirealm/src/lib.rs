//! # tui-realm
//!
//! [tui-realm](https://github.com/veeso/tui-realm) is a [tui](https://github.com/fdehau/tui-rs) framework
//! to build applications with a React/Elm inspired-by approach
//!
//! ## Get Started
//!
//! ### Adding `tui-realm` as dependency
//!
//! ```toml
//! tuirealm = "0.1.0"
//! ```
//! or if you want the std components library
//!
//! ```toml
//! tuirealm = { version = "0.1.0", features = [ "with-components" ] }
//! ```
//!
//! Since the library requires crossterm as backend, you will be required also to put `crossterm` as dependency:
//!
//! ```toml
//! crossterm = { version = "0.19" }
//! ```
//!
//!
//! ### Let's implement a View with update and view functions
//!
//! This is the most important part, since we're going to implmenent the
//! main function with all the logic. Let's underline our purpose:
//!
//! 1. we want to declare a view, which will be a container and a façade for our components and will handle focus for us
//! 2. we want to mount some components inside the view
//! 3. we want to periodically refresh the view, through the `view()` function, which will use the `render()` method of each component
//! 4. we want to handle events through messages `Msg` using an `update()` function.
//!
//! ```rust,no_run
//! extern crate crossterm;
//! extern crate tuirealm;
//! extern crate tui;
//!
//! use crossterm::event::{poll, read, Event};
//! use crossterm::event::DisableMouseCapture;
//! use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
//! use crossterm::execute;
//! use crossterm::terminal::{
//!     disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
//! };
//!
//! use std::io::{stdout, Stdout};
//! use std::thread::sleep;
//! use std::time::{Duration, Instant};
//!
//! use tuirealm::components::input;
//! use tuirealm::props::borders::{BorderType, Borders};
//! use tuirealm::{InputType, Msg, Payload, PropsBuilder, Value, View};
//!
//! // tui
//! use tui::backend::CrosstermBackend;
//! use tui::layout::{Constraint, Direction, Layout};
//! use tui::style::Color;
//! use tui::Terminal;
//!
// -- keys
//!
//! pub const MSG_KEY_ESC: Msg = Msg::OnKey(KeyEvent {
//!     code: KeyCode::Esc,
//!     modifiers: KeyModifiers::NONE,
//! });
//!
//! const MY_COMPONENT: &str = "MY_COMPONENT";
//!
//! // Let's setup an input handler
//!
//! pub(crate) struct InputHandler;
//!
//! impl InputHandler {
//!
//!     pub fn new() -> InputHandler {
//!         InputHandler {}
//!     }
//!
//!     pub fn read_event(&self) -> Result<Option<Event>, ()> {
//!         if let Ok(available) = poll(Duration::from_millis(10)) {
//!             match available {
//!                 true => {
//!                     // Read event
//!                     if let Ok(ev) = read() {
//!                         Ok(Some(ev))
//!                     } else {
//!                         Err(())
//!                     }
//!                 }
//!                 false => Ok(None),
//!             }
//!         } else {
//!             Err(())
//!         }
//!     }
//! }
//!
//! // Let's setup a Context
//!
//!
//! pub struct Context {
//!     pub(crate) input_hnd: InputHandler,
//!     pub(crate) terminal: Terminal<CrosstermBackend<Stdout>>,
//! }
//!
//! impl Context {
//!     pub fn new() -> Context {
//!         let _ = enable_raw_mode();
//!         // Create terminal
//!         let mut stdout = stdout();
//!         assert!(execute!(stdout, EnterAlternateScreen).is_ok());
//!         Context {
//!             input_hnd: InputHandler::new(),
//!             terminal: Terminal::new(CrosstermBackend::new(stdout)).unwrap(),
//!         }
//!     }
//!
//!     pub fn enter_alternate_screen(&mut self) {
//!         let _ = execute!(
//!             self.terminal.backend_mut(),
//!             EnterAlternateScreen,
//!             DisableMouseCapture
//!         );
//!     }
//!
//!     pub fn leave_alternate_screen(&mut self) {
//!         let _ = execute!(
//!             self.terminal.backend_mut(),
//!             LeaveAlternateScreen,
//!             DisableMouseCapture
//!         );
//!     }
//!
//!     pub fn clear_screen(&mut self) {
//!         let _ = self.terminal.clear();
//!     }
//! }
//!
//! impl Drop for Context {
//!     fn drop(&mut self) {
//!         // Re-enable terminal stuff
//!         self.leave_alternate_screen();
//!         let _ = disable_raw_mode();
//!     }
//! }
//!
//! // Let's create the model
//!
//! struct Model {
//!     quit: bool,
//!     redraw: bool,
//! }
//!
//! fn main() {
//!     let mut ctx: Context = Context::new();
//!     // We need to setup the terminal, entering alternate screen
//!     ctx.enter_alternate_screen();
//!     ctx.clear_screen();
//!     // Let's create a View
//!     let mut myview: View = View::init();
//!     // Let's mount all the components we need
//!     myview.mount(
//!         MY_COMPONENT,
//!         Box::new(input::Input::new(
//!             input::InputPropsBuilder::default()
//!                 .with_input(InputType::Text)
//!                 .with_foreground(Color::Yellow)
//!                 .with_label(String::from("Input component"))
//!                 .build(),
//!         )),
//!     );
//!     // ...
//!     // Give focus to our component
//!     myview.active(MY_COMPONENT);
//!     // Prepare states
//!     let mut states: Model = Model { quit: false, redraw: true };
//!     // Loop until states.quit is false
//!     /*
//!     while !states.quit {
//!          // Listen for input events
//!          if let Ok(Some(ev)) = ctx.input_hnd.read_event() {
//!             // Pass event to view
//!             let msg = myview.on(ev);
//!             states.redraw = true;
//!             // Call the elm-like update
//!             update(&mut states, &mut myview, msg);
//!         }
//!         // If redraw, draw interface
//!         if states.redraw {
//!             // Call the elm elm-like vie1 function
//!             view(&mut ctx, &myview);
//!             states.redraw = false;
//!         }
//!         sleep(Duration::from_millis(10));
//!     }
//!     */
//!     // Finalize context
//!     drop(ctx);
//! }
//!
//! // -- view
//!
//! fn view(ctx: &mut Context, view: &View) {
//!     let _ = ctx.terminal.draw(|f| {
//!         // Prepare chunks
//!         let chunks = Layout::default()
//!             .direction(Direction::Vertical)
//!             .margin(1)
//!             .constraints([Constraint::Length(3)].as_ref())
//!             .split(f.size());
//!         view.render(MY_COMPONENT, f, chunks[0]);
//!     });
//! }
//!
//! // -- update
//!
//! fn update(
//!     model: &mut Model,
//!     view: &mut View,
//!     msg: Option<(String, Msg)>,
//! ) -> Option<(String, Msg)> {
//!     let ref_msg: Option<(&str, &Msg)> = msg.as_ref().map(|(s, msg)| (s.as_str(), msg));
//!     match ref_msg {
//!         None => None, // Exit after None
//!         Some(msg) => match msg {
//!             (COMPONENT_INPUT, Msg::OnSubmit(Payload::One(Value::Str(input)))) => {
//!                 eprintln!("USER SUBMITTED {}", input);
//!                 None
//!             }
//!             (_, &MSG_KEY_ESC) => {
//!                 // Quit on esc
//!                 model.quit = true;
//!                 None
//!             }
//!             _ => None,
//!         },
//!     }
//! }
//!
//! ```
//!
//! ## Examples
//!
//! See the `examples` directory to check out how to setup tui-realm.
//! If you want a real in-production implementation, check out my project
//! [termscp](https://github.com/veeso/termscp)
//!
//!
//! ## Update
//!
//! The update function is used to handle the messages returned from the view. The name `update` is just a convention, you can call this function as you prefer.
//! I suggest you to use update, though, since it's like in Elm.
//! The usual signature of this function is `fn update(model: &mut MyModel, msg: Option<(String, Msg)>) -> Option<(String, Msg)>`
//! We don't have to mind what model is right now, it may be everything, probably it contains the View and some states, who cares;
//! what is important here is the `msg`. The message is a tuple made up by a String, which is the identifier of the component which raised the event
//! and a `Msg` enum, which contains the message itself.
//! What can we do with this then? Well, we can make a super-cool match case where we match the tuple (ID, Msg) and for each branch, we can make everything we want.
//! And as you've probably already noticed this function returns a Message tuple too. This should already have triggered you! Yes, you can call this function
//! recursively and chain different events.
//! Just a friendly reminder: you can obviously match only the event or only the ID of the component in the match case, just mind of the top-bottom priority ;)
//!
//! ### Match Keys in Update
//!
//! If you're in trouble handling key events, you're not a bad dev, it happened to me too the first time. To handle them, my suggestion is to create a keymap module
//! where you store as const all the key you need. Just follow this example:
//!
//! ```rust,no_run
//! extern crate crossterm;
//! extern crate tuirealm;
//!
//! use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
//! use tuirealm::Msg;
//!
//! // -- keys
//!
//! pub const MSG_KEY_ESC: Msg = Msg::OnKey(KeyEvent {
//!     code: KeyCode::Esc,
//!     modifiers: KeyModifiers::NONE,
//! });
//!
//! pub const MSG_KEY_TAB: Msg = Msg::OnKey(KeyEvent {
//!     code: KeyCode::Tab,
//!     modifiers: KeyModifiers::NONE,
//! });
//! ```
//!
//! Then you'll be able to match them as in the examples: `(MY_COMPONENT, &MSG_KEY_TAB) => {...}`
//!
//! ## Components
//!
//! Components represents a logical layer above a tui widget. Components allows you to handle properties and states for widgets and this will be rendered.
//! The component is implemented through a trait which is called `Component` indeed. For each component you must implement these methods:
//!
//! - render: this method renders the component inside the area passed as argument. Mind that the component should be rendered only if `props.visible` is `true`.
//! - update: update the component properties. You can return a `Msg` if you want and change the states if needed.
//! - get_props: returns a copy of the component properties
//! - on: handle an input event; returns a Msg
//! - get_state: returns the current state to be exposed to the user.
//! - blur: change the focus state for the component. This won't affect the view.
//! - active: enable the focus state for the component. This won't affect the view.
//!
//! Each component should have properties, provided by the `Props` struct and, if required
//! some states, which are defined for each struct. States mustn't be public.
//! I strongly suggest to implement a `PropsBuilder`, via the namesake trait, for each component properties, in order to make easier to define
//! properties for each component, while if you don't want to do so, you can use the `GenericPropsBuilder`.
//!

#![doc(html_playground_url = "https://play.rust-lang.org")]
#![doc(
    html_favicon_url = "https://raw.githubusercontent.com/veeso/tui-realm/main/docs/images/cargo/tui-realm-128.png"
)]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/veeso/tui-realm/main/docs/images/cargo/tui-realm-512.png"
)]

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
extern crate tui as tuirs;

// Ext
use std::collections::HashMap;
use std::io::Stdout;
use tuirs::{backend::CrosstermBackend, layout::Rect, Frame};

// Modules
#[cfg(feature = "with-components")]
pub mod components;
pub mod event;
pub mod props;
pub mod tui;
pub mod view;
// Export use
pub use self::props::{
    borders, texts, GenericPropsBuilder, InputType, PropValue, Props, PropsBuilder,
};
pub use self::view::View;

// locals
use event::{Event, KeyEvent};

// -- Types

/// ## Canvas
///
/// Canvas represents the Frame where the view will be displayed in
pub type Canvas<'a> = Frame<'a, CrosstermBackend<Stdout>>;

// -- Msg

/// ## Msg
///
/// Msg is an enum returned after an event is raised for a certain component
/// Yep, I took inspiration from Elm.
#[derive(Debug, PartialEq, Eq)]
pub enum Msg {
    OnSubmit(Payload),
    OnChange(Payload),
    OnKey(KeyEvent),
    None,
}

/// ## Payload
///
/// Payload describes a component value
#[derive(Debug, PartialEq, Eq)]
pub enum Payload {
    One(Value),
    Tup2((Value, Value)),
    Tup3((Value, Value, Value)),
    Tup4((Value, Value, Value)),
    Vec(Vec<Value>),
    Map(HashMap<String, Value>),
    None,
}

/// ## Value
///
/// Value describes the value contained in a Payload
#[derive(Debug, PartialEq, Eq)]
pub enum Value {
    Bool(bool),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
    Usize(usize),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
    Isize(isize),
    Str(String),
}

// -- Component

/// ## Component
///
/// Component is a trait which defines the behaviours for a View component.
pub trait Component {
    /// ### render
    ///
    /// Based on the current properties and states, renders the component in the provided area frame
    #[cfg(not(tarpaulin_include))]
    fn render(&self, frame: &mut Canvas, area: Rect);

    /// ### update
    ///
    /// Update component properties
    /// Properties should first be retrieved through `get_props` which returns
    /// the current properties, which can be used to create new properties.
    /// Returns a Msg to the view
    fn update(&mut self, props: Props) -> Msg;

    /// ### get_props
    ///
    /// Returns the current component properties.
    /// The returned properties can then be used to create a new PropsBuilder,
    /// which can lately be used to update the component's properties.
    fn get_props(&self) -> Props;

    /// ### on
    ///
    /// Handle input event and update internal states.
    /// Returns a Msg to the view
    fn on(&mut self, ev: Event) -> Msg;

    /// ### get_state
    ///
    /// Get current state from component
    fn get_state(&self) -> Payload;

    // -- state changers

    /// ### blur
    ///
    /// Blur component; basically remove focus
    fn blur(&mut self);

    /// ### active
    ///
    /// Active component; basically give focus
    fn active(&mut self);
}
