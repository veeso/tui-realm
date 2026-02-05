# Advanced concepts

📍<u>**English**</u> | <a href="../zh-cn/advanced.md">简体中文</a>

- [Advanced concepts](#advanced-concepts)
  - [Introduction](#introduction)
  - [Subscriptions](#subscriptions)
    - [Handle Subscriptions](#handle-subscriptions)
    - [Event clauses in detail](#event-clauses-in-detail)
    - [Sub clauses in detail](#sub-clauses-in-detail)
    - [Subscriptions lock](#subscriptions-lock)
  - [Tick Event](#tick-event)
  - [Ports](#ports)
  - [Implementing new components](#implementing-new-components)
    - [What the component should look like](#what-the-component-should-look-like)
    - [Defining the component properties](#defining-the-component-properties)
    - [Defining the component states](#defining-the-component-states)
    - [Defining the Command API](#defining-the-command-api)
    - [Rendering the component](#rendering-the-component)
  - [Properties Injectors](#properties-injectors)
  - [What's next](#whats-next)

---

## Introduction

This guide will introduce you to all the advanced concepts of `tui-realm`, that haven't been covered in the [Get Started guide](get-started.md).
Altough `tui-realm` is quite simple, it can also get quite powerful, thanks to all these features that we're gonna cover in this document.

What you will learn:

- How to handle **Subscriptions**, making some components to listen to certain events under certain circumstances, even when not focused.
- How to use custom sources for events with `Ports`.
- More details on how to design reusable custom components than the [Get Started guide](get-started.md)

---

## Subscriptions

> A subscription is a ruleset which tells the **Application** to forward events to other components even if they're not active.

As we've already covered in the base concepts of `tui-realm`, the *Application* takes care of forwarding events from *Ports* to *Components*.
By default events are forwarded only to the currently active component, but this can be be quite annoying:

- First, we may need a component to always listen for specific incoming events. Imagine some *Components* need some data from a remote server.
  They can't get updated only when they've got focus, they probably needs to be updated each time an event is coming from the *Port* and is received by the *Event listener*.
  Without *Subscriptions* this would be impossible to do easily.
- Sometimes it is just repetetive and a choice of scoping: in the example from [Get Started](get-started.md#our-first-application) we had two counters,
  and both of them were listening for the `<ESC>` key to quit the application, returning a `AppClose` message.
  But should it really be their responsiblity to tell whether the application should terminate?
  I mean, they're just counters, so they shouldn't know whether to close the app right?
  Besides that, it's also really annoying to write a case for `<ESC>` key for each component to return `AppClose`.
  Having an invisible component that always listens for `<ESC>` and returns `AppClose` would be much more comfy.

So what is a subscription actually, and how we can create them?

The subscription is defined as:

```rust
pub struct Sub<ComponentId, UserEvent>(SubEventClause<UserEvent>, Arc<SubClause<ComponentId>>)
where
    ComponentId: Eq + PartialEq + Clone + Hash,
    UserEvent: Eq + PartialEq + Clone;
```

It takes 2 parameters:
- `SubEventClause<UserEvent>`: The **Event Clause**, which determines what type of event to forward. This practically mirrors `Event`'s Variants directly.
- `SubClause<ComponentId>`: The actual *ruleset* to determine, well, *when* to forward a given Event to the target Component.

**`SubClause`** has many variants to allow a broad range of possibilities, for example it has `Always`, which will always forward a event, no specific rules; it has `IsMounted`, which is like `Always`, only that it only forwards to **Id** if there is a component mounted to it; finally there are logical combinators like `And`, `Or` and `Not`. There are some other variants, which should be self-describing. More on that in a later section.

So when an event is received, if a component, **that is not active**, satisfies the *event clause* and the *sub clause*, then the event will be forwarded to that component too.

> ❗ In order to forward an event, both the `SubEventClause` and the `SubClause` must be satisfied

Note that if a given Component is active, it will never get the event twice through being focused and through a subscription it may also have.

Let's see in details how to handle subscriptions and how to use clauses.

### Handle Subscriptions

You can create subscriptions both on component mounting and dynamically afterward.

To subscribe a component on `mount` it will be enough to provide a vector of `Sub` to `mount()`:

```rust
app.mount(
    Id::Clock,
    Box::new(
        Clock::new(SystemTime::now())
            .alignment(Alignment::Center)
    ),
    vec![Sub::new(SubEventClause::Tick, SubClause::Always)]
);
```

Or you can create new subscriptions whenever you want:

```rust
app.subscribe(&Id::Clock, Sub::new(SubEventClause::Tick, SubClause::Always));
```

And if you need to remove a subscription you can unsubscribe simply with:

```rust
app.unsubscribe(&Id::Clock, SubEventClause::Tick);
```

> ❗ If you have multiple rules for a given `SubEventClause`, `unsubscribe` will remove *all* subscriptions matching that clause.

### Event clauses in detail

Event clauses are used to define which kind of event the subscription should apply to.
Once the application checks whether to forward an event, it must check the event clause first and verify whether it satisfies the bounds with the incoming event. The event clauses are:

- `Any`: the event clause is satisfied, no matter what kind of event is. Everything depends on the result of the `SubClause` then.
- `Keyboard(KeyEvent)`: in order to satisfy the clause, the incoming event must be of type `Keyboard` and the `KeyEvent` must exactly be the same.
- `WindowResize`: in order to satisfy the clause, the incoming event must be of type `WindowResize`, no matter which size the window has.
- `Tick`: in order to satisfy the clause, the incoming event must be of type `Tick`.
- `User(UserEvent)`: in order to be satisfied, the incoming event must be of type of `User`. The value of `UserEvent` must match, according on how `PartialEq` is implemented for this type.
- `Discriminant(UserEvent)`: in order to be statisfied, the incoming event must be of type `User`. Then the `UserEvent`'s `std::mem::discriminant` must match.

> ❗ `Discriminant` will only check the top-level variants of `UserEvent`, meaning that something akin to `UserEvent::VariantA(OtherEnum::A)` and `UserEvent::Variant(OtherEnum::B)` **will match**. If this is not behavior you want, use `User(UserEvent)` and implement a custom `PartialEq` matching.

### Sub clauses in detail

Sub clauses are verified once the event clause is satisfied. They define some clauses that must be satisfied to actually forward the event.
In particular sub clauses are:

- `Always`: the clause is always satisfied.
- `HasAttrValue(Id, Attribute, AttrValue)`: the clause is satisfied if the Component at **Id** has `Attribute` with `AttrValue`.
- `HasState(Id, State)`: the clause is satisfied if the Component at **Id** has `State` equal to provided state.
- `IsMounted(Id)`: the clause is satisfied if the Component at **Id** is mounted in the View.

> If the current Component for the subscription does not depend on another Component being mounted, then `Always` should be used over `IsMounted(Self)`.

In addition to these, it is also possible to combine Sub clauses using logical expressions:

- `Not(SubClause)`: the clause is satisfied if the inner clause is NOT satisfied (inverts the result) (locial NOT)
- `And(SubClause, SubClause)`: the clause is satisfied if both clause are satisfied (logical AND)
- `Or(SubClause, SubClause)`: the clause is satisfied if at least one of the two clauses is satisfied. (logical OR)

Using `And` and `Or` you can create even longer expression and keep in mind that they are evaluated recursively, so for example:

`And(Or(A, And(B, C)), And(D, Or(E, F)))` is evaluated as `(A || (B && C)) && (D && (E || F))`.

Short-circuiting is supported where possible. For example if in `Or(A, B)` `A` evaluates to `true`, then `B` is **NOT** evaluated.
Similarly in `And(A, B)`, if `A` evaluates to `false`, then `B` is **NOT** evaluated.

Additionally, to better optimize memory locality, additional variants are available:

- `AndMany`: basically the same as `And`, but which allows more (or less) than 2 clauses at once (also supports short-circuiting)
- `OrMany`: basically the same as `Or`, but which allos more (or less) than 2 clauses at once (also supports short-circuiting)

### Subscriptions lock

It is possible to temporarily disable the event forwarding the subscriptions allow.
To do so, you just need to call `Application::lock_subs()`.

Whenever you want to restore event propagation, just call `Application::unlock_subs()`.
Events inbetween those 2 calls will *not* be forwarded once unlocked again.

---

## Tick Event

The tick event is a special kind of event, which is raised by the **Application** with a specified interval.
Whenevever initializing the **Applcation** you can specify the tick interval, as in the following example:

```rust
let app = Application::init(
    EventListenerCfg::default()
        .tick_interval(Duration::from_secs(1)),
);
```

With the `tick_interval()` method, we specify the tick interval.
Each time the tick interval elapses, the application runtime will provide a `Event::Tick` which will be forwarded on `tick()` to the
currently active component and to all the components subscribed to the `Tick` event.

The purpose of the tick event is to schedule actions based on a certain interval. For example step a spinner consistently.

---

## Ports

Ports are basically **Event producers** which are handled by the applications *Event listener*.
Simple `tui-realm` applications will only consume the core provided events, but what if we need *more* events?

We may for example need a worker which fetches data from a remote server. You dont want the TUI to block (not process any events) until that data fetching has finished, right?
Ports allow you to create workers which will produce the events and if you set up everything correctly, your model and components will be updated.

Let's see now how to setup a custom *Port*:

1. First we need to define the `UserEvent` type for our application:

    ```rust
    #[derive(PartialEq, Clone)]
    pub enum UserEvent {
        GotData(Data)
        // ... other events if you need
    }

    impl Eq for UserEvent {}
    ```

2. Implement the *Port*, that I named `MyHttpClient`

    ```rust
    pub struct MyHttpClient {
        // ...
    }
    ```

    Now we need to implement the `Poll` trait for the *Port*.
    If you have ever worked with async rust, this trait is quite similar to `std::future::Future`.
    The poll trait tells the application event listener how to poll for events on a *port*:

    ```rust
    impl Poll<UserEvent> for MyHttpClient {
        fn poll(&mut self) -> PortResult<Option<Event<UserEvent>>> {
            // ... do something ...
            Ok(Some(Event::User(UserEvent::GotData(data))))
        }
    }
    ```

3. Add the Port to the Application

    ```rust
    let mut app: Application<Id, Msg, UserEvent> = Application::init(
        EventListenerCfg::default()
            /* ...Other ports like "crossterm_input_listener" */
            .port(
                Box::new(MyHttpClient::new()),
                Duration::from_millis(100),
            ),
    );
    ```

    On the event listener constructor you can define how many ports you want. When you declare a port you need to pass a
    box containing the type implementing the *Poll* trait and an interval.
    The interval defines the interval between each poll to the port.

Ports in `tui-realm` can also be described as the `Actor Pattern`. You can read more about this pattern in rust in the [Actors with Tokio](https://ryhl.io/blog/actors-with-tokio/) blog post.

`tui-realm` also supports async ports (though only via `tokio` for now), which can be enabled with feature `async-ports`.

If your application already makes use of async, it is recommended you use async ports over sync-ports.

---

## Implementing new components

Implementing components is quite simple in tui-realm. This example will implement a more complex Component that what was shown in [Get Started](get-started.md#the-mock-component) and requires the knowledge of the difference between *Mock Component* and *Component* and at least a little knowledge about *ratatui widgets*.

That said, let's see how to implement a more complex component. For this example I will implement a simplified version of the `Radio` component of the stdlib.

### What the component should look like

The first thing we need to define is what the component should look like (as in output to display).
In this case, the component should be a box with all options listed horizontally. For this we will use the ratatui widget `Tabs`.

Next, we need to define what the component interaction should look like.
We want the user to be able to move left and right to select one and then be able to submit the currently selected option.

### Defining the component properties

Once we've defined what the component should look like, we can start defining the component properties to expose:

- `Foreground(Color)`: will define the foreground color for the component
- `Background(Color)`: will define the background color for the component
- `HighlightedColor(Color)`: will define the background color for the highlighted choice
- `Borders(Borders)`: will define the border properties for the component
- `Content(Payload(Vec(String)))`: will define the possible options for the radio group
- `Title(Title)`: will define the box title
- `Value(Payload(One(Usize)))`: will work as a prop, but will update the state too, for the current selected option.

```rust
pub struct Radio {
    props: Props,
    state: RadioState
}

impl Radio {
    // Constructors...

    pub fn foreground(mut self, fg: Color) -> Self {
        self.attr(Attribute::Foreground, AttrValue::Color(fg));
        self
    }

    // Other builder functions for all the properties...
}

impl MockComponent for Radio {
    // ...

    fn query(&self, attr: Attribute) -> Option<AttrValue> {
        self.props.get(attr)
    }

    fn attr(&mut self, attr: Attribute, value: AttrValue) {
        match attr {
            Attribute::Content => {
                // Overwrite choices, but keep index if possible
                let choices: Vec<String> = value
                    .unwrap_payload()
                    .unwrap_vec()
                    .into_iter()
                    .map(|x| x.unwrap_str())
                    .collect();
                self.state.set_choices(choices);
            }
            Attribute::Value => {
                let index = value.unwrap_payload().unwrap_one().unwrap_usize();
                self.state.select(index);
            }
            attr => {
                self.props.set(attr, value);
            }
        }
    }

    // ...
}
```

### Defining the component states

Since this component can be interactive and the user must be able to select a certain option, we must implement some state.
The component states must track the current selected item. For practical reasons, we also use the available choices as a state.

```rust
struct RadioState {
    choice: usize,        // Selected option
    choices: Vec<String>, // Available choices
}

impl RadioState {
    /// Move choice index to next choice
    pub fn next_choice(&mut self) {
        if self.choice + 1 < self.choices.len() {
            self.choice += 1;
        }
    }

    /// Move choice index to previous choice
    pub fn prev_choice(&mut self) {
        if self.choice > 0 {
            self.choice -= 1;
        }
    }

    /// Select a specific index
    pub fn select(&mut self, i: usize) {
        if i < self.choices.len() {
            self.choice = i;
        }
    }

    /// Set RadioState choices from a vector of text spans.
    /// In addition resets current selection and keep index if possible or set it to the first value
    /// available.
    pub fn set_choices<S: Into<Vec<String>>>(&mut self, spans: S) {
        self.choices = spans.into();
        // Keep index if possible
        if self.choice >= self.choices.len() {
            self.choice = self.choices.len().saturating_sub(1);
        }
    }
}
```

Then we can define the `state()` method of `MockComponent`:

```rust
impl MockComponent for Radio {
    // ...

    fn state(&self) -> State {
        State::One(StateValue::Usize(self.states.choice))
    }

    // ...
}
```

### Defining the Command API

Once we've defined the component states, we can start thinking of the Command API. The Command API defines how the component
behaves in front of incoming commands and what kind of result it should return.

For this component we'll handle the following commands:

- When the user moves to the right, the current choice is incremented
- When the user moves to the left, the current choice is decremented
- When the user submits, the current choice is returned

```rust
impl MockComponent for Radio {
    // ...

    fn perform(&mut self, cmd: Cmd) -> CmdResult {
        match cmd {
            Cmd::Move(Direction::Right) => {
                // Increment choice
                self.states.next_choice();
                // Return CmdResult On Change
                CmdResult::Changed(self.state())
            }
            Cmd::Move(Direction::Left) => {
                // Decrement choice
                self.states.prev_choice();
                // Return CmdResult On Change
                CmdResult::Changed(self.state())
            }
            Cmd::Submit => {
                // Return Submit
                CmdResult::Submit(self.state())
            }
            _ => CmdResult::None,
        }
    }

    // ...
}
```

### Rendering the component

Finally, we can implement the component `view()` method of `MockComponent` which will render the component:

```rust
impl MockComponent for Radio {
    fn view(&mut self, render: &mut Frame, area: Rect) {
        if self.props.get_or(Attribute::Display, AttrValue::Flag(true)) == AttrValue::Flag(true) {
            // Make choices
            let choices: Vec<Spans> = self
                .state
                .choices
                .iter()
                .map(|x| Spans::from(x))
                .collect();

            // Fetch all other style properties
            let foreground = self
                .props
                .get_or(Attribute::Foreground, AttrValue::Color(Color::Reset))
                .unwrap_color();
            let background = self
                .props
                .get_or(Attribute::Background, AttrValue::Color(Color::Reset))
                .unwrap_color();
            let highlight_bg = self
                .props
                .get_or(Attribute::HighlightedColor, AttrValue::Color(Color::Reset))
                .unwrap_color();

            let normal_style = Style::default()
                .fg(foreground)
                .bg(background);

            let highlight_style = style.patch(Style::default().bg(highlight_bg));

            // assemble the Block (borders)
            let borders = self
                .props
                .get_or(Attribute::Borders, AttrValue::Borders(Borders::default()))
                .unwrap_borders();
            let title = self.props.get(Attribute::Title).map(|x| x.unwrap_title());
            let focus = self
                .props
                .get_or(Attribute::Focus, AttrValue::Flag(false))
                .unwrap_flag();

            let block = Block::default()
                .title_top(title.content)
                .borders(borders.sides)
                .border_style(if focus { borders.style() } else { Style::default().fg(Color::DarkGrey) });

            // Finally, use a ratatui widget to draw the contents
            let tabs = Tabs::new(choices)
                .block(block)
                .select(self.state.choice)
                .style(style)
                .highlight_style(highlight_style);
            render.render_widget(tabs, area);
        }
    }

    // ...
}
```

---

## Properties Injectors

Properties injectors are trait objects, which must implement the `Injector` trait, which can provide some property (defined as a tuple of `Attribute` and `AttrValue`) for components when they're mounted.
The Injector trait is defined as follows:

```rs
pub trait Injector<ComponentId>
where
    ComponentId: Eq + PartialEq + Clone + Hash,
{
    fn inject(&self, id: &ComponentId) -> Vec<(Attribute, AttrValue)>;
}
```

Then you can add an injector to your application with the `add_injector()` method.

Whenever you mount a new component into your view, the `inject()` method is called for each injector defined in your application providing as argument the id of the mounted component.

---

## What's next

This is the end of the currently available guides for `tui-realm`, but consider reading further into:

- documentation of [`tuirealm`](https://docs.rs/tuirealm/latest/tuirealm/)
- documentation of [`tui-realm-stdlib`](https://docs.rs/tui-realm-stdlib/latest/tui_realm_stdlib/)

If you have any question, feel free to open an issue with the `question` label and I will answer you ASAP 🙂.
