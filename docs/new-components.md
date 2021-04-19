# Implement new components

- [Implement new components](#implement-new-components)
  - [Introduction](#introduction)
  - [Setup](#setup)
  - [States](#states)
  - [Properties](#properties)
  - [Implement Component trait](#implement-component-trait)
    - [Render](#render)
    - [Update](#update)
    - [Get Props](#get-props)
    - [On](#on)
    - [Get State](#get-state)
    - [Focus](#focus)
    - [To summarize](#to-summarize)
  - [What's next](#whats-next)

---

## Introduction

This document describes how to implement a new component in tui-realm. This procedure is both valid to extend the standard library in a pull request and to implement your own private components.
Tui-Realm has been designed to make as simpler as possible to implement new components, so let's see in what this procedure consists:

- Define the states of your component if you need any
- Define the properties you're going to use
- Implement the prop builder for your component (*optional - you can use the GenericPropsBuilder, but I strongly suggest to implement your own*)
- Implement the **Component** trait for your component

If you're new to tui-realm, I strongly suggest to read the [component lifecycle guide](lifecycle.md), which also explains each part of a component in details ☺.

Okay, let's start then!

## Setup

We'll only need one file for this, so let's say we want to implement a simple `Counter` component: this component will increment a state when the user presses Submit and will show a button with a customizable text and the counter value.
So let's say we're going to work in a file `counter.rs`.
The first thing we need to do is to import what we need:

```rust
extern crate crossterm;
extern crate tui;
extern crate tuirealm;

use tuirealm::{Canvas, Component, Event, Msg, Payload};
use tuirealm::components::utils::get_block;
use tuirealm::props::{BordersProps, PropValue, Props, PropsBuilder, TextParts, TextSpan};

use crossterm::event::KeyCode;
use tui::{
    layout::Rect,
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, Paragraph},
};
```

## States

Let's define the states we need. In this case we only need to define a counter value which will be an `usize`.

```rust
// -- states

struct OwnStates {
    counter: usize,
    focus: bool,
}

impl Default for OwnStates {

    fn default() -> Self {
        OwnStates {
            counter: 0,
            focus: false,
        }
    }

}

impl OwnStates {

    pub fn incr(&mut self) {
        self.counter += 1;
    }

}

```

The struct must be private, and by convention is called `OwnStates`. We define a counter value and the focus state. The focus flag is used to change the component color when is active, in order to give the user the feedback it is enabled.

## Properties

Let's define the properties we'll be using and after that we'll make the props builder for it.
I don't want to make it complicated, so let's say we only need these properties:

- foreground color
- background color
- label string
- border properties
- initial value

First of all we always need to implement these traits for the props builder:

```rust
// -- Props

pub struct CounterPropsBuilder {
    props: Option<Props>,
}

impl Default for CounterPropsBuilder {
    fn default() -> Self {
        let mut builder = CounterPropsBuilder {
            props: Some(Props::default()),
        };
        builder.with_inverted_color(Color::Black);
        builder
    }
}

impl PropsBuilder for CounterPropsBuilder {
    fn build(&mut self) -> Props {
        self.props.take().unwrap()
    }

    fn hidden(&mut self) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            props.visible = false;
        }
        self
    }

    fn visible(&mut self) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            props.visible = true;
        }
        self
    }
}

impl From<Props> for CounterPropsBuilder {
    fn from(props: Props) -> Self {
        CounterPropsBuilder { props: Some(props) }
    }
}

```

Then let's define the property setters:

```rust
impl CounterPropsBuilder {
    pub fn with_foreground(&mut self, color: Color) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            props.foreground = color;
        }
        self
    }

    pub fn with_background(&mut self, color: Color) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            props.background = color;
        }
        self
    }

    pub fn with_borders(
        &mut self,
        borders: Borders,
        variant: BorderType,
        color: Color,
    ) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            props.borders = BordersProps {
                borders,
                variant,
                color,
            }
        }
        self
    }

    pub fn with_label(&mut self, label: String) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            props.texts = TextParts::new(Some(label), None);
        }
    }

    pub fn with_value(&mut self, counter: usize) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            props.value = PropValue::Unsigned(counter);
        }
        self
    }
}
```

## Implement Component trait

Finally we just need to implement the Component trait for our component, but first we need to define it:

```rust
// -- Component

pub struct Counter {
    props: Props,
    states: OwnStates,
}

impl Counter {
    pub fn new(props: Props) -> Self {
        let mut states: OwnStates = OwnStates::default();
        // Init counter
        if let PropValue::Unsigned(val) = &props.value {
            states.counter = *val;
        }
        Counter { props, states }
    }
}
```

Then let's define each method for the component

```rust
impl Component for Counter {
  // ...
}
```

### Render

The render method must, indeed, render the component into the canvas.
To do so, we'll need to define colors etc. I will make it very simple here. Unfortunately I cannot help you to implement yours, but try to give a look at the tui documentation 😄.

```rust
    fn render(&self, render: &mut Canvas, area: Rect) {
        // Make a Span - THIS IS VERY IMPORTANT!!!
        if self.props.visible {
            // Make text
            let prefix: String = match self.props.texts.title.as_ref() {
                None => String::new(),
                Some(t) => t.clone(),
            };
            let text: String = format!("{} ({})", prefix, self.states.counter);
            let block: Block = super::utils::get_block(
                &self.props.borders,
                &None,
                self.states.focus,
            );
            render.render_widget(
                Paragraph::new(title).block(block).style(
                    Style::default()
                        .fg(self.props.foreground)
                        .bg(self.props.background)
                        .add_modifier(self.props.modifiers),
                ),
                area,
            );
        }
    }
```

### Update

Update, as you might know must update the component properties. This also returns a Msg; in our case we'll return a `OnChange` message if the value of the counter is changed.

```rust
    fn update(&mut self, props: Props) -> Msg {
        let prev_value = self.states.counter;
        // Get value
        if let PropValue::Unsigned(val) = &props.value {
            self.states.counter = *val;
        }
        self.props = props;
        // Msg none
        if prev_value != *self.states.counter {
            Msg::OnChange(self.get_state())
        } else {
            Msg::None
        }
    }
```

### Get Props

Get props is kinda standard, since just clones the properties.

```rust
    fn get_props(&self) -> Props {
        self.props.clone()
    }
```

### On

On must handle an input event received from crossterm. In this case I'm just going to return a OnChange after `Enter` is pressed, while `OnKey` will be returned for all the other keys. The counter will be obviously incremented before.

```rust
    fn on(&mut self, ev: Event) -> Msg {
        // Match event
        if let Event::Key(key) = ev {
            match key.code {
                KeyCode::Enter => {
                    // Increment first
                    self.states.incr();
                    // Return OnChange
                    Msg::OnChange(self.get_state())
                }
                _ => {
                    // Return key event to activity
                    Msg::OnKey(key)
                }
            }
        } else {
            // Ignore event
            Msg::None
        }
    }
```

### Get State

Get state just exposes a meaningful state to the application. In this case it is obviously the counter value, so we'll return a `Payload::Unsigned`:

```rust
    fn get_state(&self) -> Payload {
        Payload::Unsigned(self.states.counter)
    }
```

### Focus

Finally we just need to implement `blur` and `active` for the component:

```rust
    fn blur(&mut self) {
        self.states.focus = false;
    }

    fn active(&mut self) {
        self.states.focus = true;
    }
```

### To summarize

... TODO: complete

---

## What's next

... TODO: complete
