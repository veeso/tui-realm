//! # legacy API
//!
//! This module contains the legacy tui-realm API

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
// Ext
use std::collections::{HashMap, LinkedList};
use std::io::Stdout;
use tuirs::{backend::CrosstermBackend, layout::Rect, Frame as TuiFrame};

// Modules
pub mod props;
pub mod view;
// Export use
pub use self::view::View;
pub use props::{
    borders, texts, GenericPropsBuilder, InputType, PropPayload, PropValue, Props, PropsBuilder,
};

// locals
use crossterm::event::{Event, KeyEvent};

// -- Types

/// ## Frame
///
/// Frame represents the Frame where the view will be displayed in
pub type Frame<'a> = TuiFrame<'a, CrosstermBackend<Stdout>>;

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
    Tup4((Value, Value, Value, Value)),
    Vec(Vec<Value>),
    Map(HashMap<String, Value>),
    Linked(LinkedList<Payload>),
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
    fn render(&self, frame: &mut Frame, area: Rect);

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

// -- update

/// ## Update
pub trait Update {
    /// ### update
    ///
    /// update the current state handling a message from the view.
    /// This function may return a Message, so this function has to be intended to be call recursively.
    fn update(&mut self, msg: Option<(String, Msg)>) -> Option<(String, Msg)>;
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_lib_payload_and_values() {
        Payload::One(Value::Usize(2));
        Payload::Tup2((Value::Bool(true), Value::Usize(128)));
        Payload::Tup3((
            Value::Bool(true),
            Value::Usize(128),
            Value::Str(String::from("omar")),
        ));
        Payload::Tup4((
            Value::Bool(true),
            Value::U8(128),
            Value::Str(String::from("pippo")),
            Value::Isize(-2),
        ));
        Payload::Vec(vec![
            Value::U16(1),
            Value::U32(2),
            Value::U64(3),
            Value::U128(4),
        ]);
        let mut map: HashMap<String, Value> = HashMap::new();
        map.insert(String::from("a"), Value::I8(4));
        map.insert(String::from("b"), Value::I16(-8));
        map.insert(String::from("c"), Value::I32(16));
        map.insert(String::from("d"), Value::I64(-32));
        map.insert(String::from("e"), Value::I128(64));
        Payload::Map(map);
        let mut link: LinkedList<Payload> = LinkedList::new();
        link.push_back(Payload::One(Value::Usize(1)));
        link.push_back(Payload::Tup2((Value::Usize(2), Value::Usize(4))));
        Payload::Linked(link);
        drop(Payload::None);
    }
}
