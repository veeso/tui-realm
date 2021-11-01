# Advanced concepts

- [Advanced concepts](#advanced-concepts)
  - [Introduction](#introduction)
  - [Subscriptions](#subscriptions)
    - [Handle subscriptions](#handle-subscriptions)
    - [Event clauses in details](#event-clauses-in-details)
    - [Sub clauses in details](#sub-clauses-in-details)
  - [Tick Event](#tick-event)
  - [Ports](#ports)
  - [Implementing new components](#implementing-new-components)
  - [Best practices](#best-practices)
    - [Over states](#over-states)

---

## Introduction

TODO: youtube link
> 👀 If you prefer there's also a crash course of tui-realm on my [Youtube Channel]() where I cover all of these arguments

This guide will introduce you to all the advanced concepts of tui-realm, that haven't been covered in the [get-started guide](get-started.md). Altough tui-realm is quite simple, it can also get quiet powerful, thanks to all these features that we're gonna cover in this document.

What you will learn:

- How to handle subscriptions, making some components to listen to certain events under certain circumstances.
- What is the `Event::Tick`
- How to use custom source for events through `Ports`.
- tui-realm best practice: how tui-realm is meant to be used (trust me, I designed it 😉)

---

## Subscriptions

> A subscription is a ruleset which tells the **application** to forward events to other components even if they're not active, based on some rules.

As we've already covered in the base concepts of tui-realm, the application takes care of forwarding events from ports to components.
By default events are forwarded only to the current active component, but this can be be quite annoying:

- First, we may need a component always listening for incoming events. Imagine some loaders polling a remote server. They can't get updated only when they've got focus, they probably needs to be updated each time an event coming from the *Port* is received by the *Event listener*. Without *Subscriptions* this would be impossible.
- Sometimes is just a fact of "it's boring" and scope: in the example I had two counters, and both of them were listening for `<ESC>` key to quit the application returning a `AppClose` message. But is that their responsiblity to tell whether the application should terminate? I mean, they're just counter, so they shouldn't know whether to close the app right? Besides of that, it's also really annoying to write a case for `<ESC>` for each component to return `AppClose`. Having an invisible component always listening for `<ESC>` to return `AppClose` would be much more comfy.

So what is a subscription actually, and how we can create them?

The subscription is defined as:

```rust
pub struct Sub<UserEvent>(EventClause<UserEvent>, SubClause)
where
    UserEvent: Eq + PartialEq + Clone + PartialOrd;
```

So it's a tupled structure, which takes an `EventClause` and a `SubClause`, let's dive deeper:

- An **Event clause** is a match clause the incoming event must satisfy. As we said before the application must know whether to forward a certain *event* to a certain component. So the first thing it must check, is whether it is listening for that kind of event.

    The event clause is declared as follows:

    ```rust
    pub enum EventClause<UserEvent>
    where
        UserEvent: Eq + PartialEq + Clone + PartialOrd,
    {
        /// Forward, no matter what kind of event
        Any,
        /// Check whether a certain key has been pressed
        Keyboard(KeyEvent),
        /// Check whether window has been resized
        WindowResize,
        /// The event will be forwarded on a tick
        Tick,
        /// Event will be forwarded on this specific user event.
        /// The way user event is matched, depends on its partialEq implementation
        User(UserEvent),
    }
    ```

- A **Sub clause** is an additional condition that must be satisfied by the component associated to the subscription in order to forward the event:

    ```rust
    pub enum SubClause {
        /// Always forward event to component
        Always,
        /// Forward event if target component has provided attribute with the provided value
        /// If the attribute doesn't exist on component, result is always `false`.
        HasAttrValue(Attribute, AttrValue),
        /// Forward event if target component has provided state
        HasState(State),
        /// Forward event if the inner clause is `false`
        Not(Box<SubClause>),
        /// Forward event if both the inner clauses are `true`
        And(Box<SubClause>, Box<SubClause>),
        /// Forward event if at least one of the inner clauses is `true`
        Or(Box<SubClause>, Box<SubClause>),
    }
    ```

So when an event is received, if a component, **that is not active** satisfies the event clause and the sub clause, then the event will be forwarded to that component too.

> ❗ In order to forward an event, both the `EventClause` and the `SubClause` must be satisfied

Let's see in details how to handle subscriptions and how to use clauses.

### Handle subscriptions

You can create subscriptions both on component mounting and whenever you want.

To subscribe a component on `mount` it will be enough to provide a vector of `Sub` to `mount()`:

```rust
app.mount(
    Id::Clock,
    Box::new(
        Clock::new(SystemTime::now())
            .alignment(Alignment::Center)
            .background(Color::Reset)
            .foreground(Color::Cyan)
            .modifiers(TextModifiers::BOLD)
    ),
    vec![Sub::new(SubEventClause::Tick, SubClause::Always)]
);
```

or you can create new subscriptions whenever you want:

```rust
app.subscribe(&Id::Clock, Sub::new(SubEventClause::Tick, SubClause::Always));
```

and if you need to remove a subscription you can unsubscribe simply with:

```rust
app.unsubscribe(&Id::Clock, SubEventClause::Tick);
```

### Event clauses in details

Event clauses are used to define for which kind of event the subscription should be set.
Once the application checks whether to forward an event, it must check the event clause first and verify whether it satisfies the bounds with the incoming event. The event clauses are:

- `Any`: the event clause is satisfied, no matter what kind of event is. Everything depends on the result of the `SubClause` then.
- `Keyboard(KeyEvent)`: in order to satisfy the clause, the incoming event must be of type `Keyboard` and the `KeyEvent` must exactly be the same.
- `WindowResize`: in order to satisfy the clause, the incoming event must be of type `WindowResize`, no matter which size the window has.
- `Tick`: in order to satisfy the clause, the incoming event must be of type `Tick`.
- `User(UserEvent)`: in order to be satisfied the incoming event must be of type of `User`. The value of `UserEvent` must match, according on how `PartialEq` is implemented for this type.

### Sub clauses in details

Sub clauses are verified once the event clause is satisfied, and they define some clauses that must be satisfied on the **target** component (which is the component associated to the subscription).
In particular sub clauses are:

- `Always`: the clause is always satisfied
- `HasAttrValue(Attribute, AttrValue)`: the clause is satisfied if the target component has `Attribute` with `AttrValue` in its `Props`.
- `HasState(State)`: the clause is satisfied if the target component has `State` equal to provided state.

In addition to these, it is also possible to combine Sub clauses using expressions:

- `Not(SubClause)`: the clause is satisfied if the inner clause is NOT satisfied (negates the result)
- `And(SubClause, SubClause)`: the clause is satisfied if both clause are satisfied
- `Or(SubClause, SubClause)`: the clause is satisfied if at least one of the two clauses is satisfied.

Using `And` and `Or` you can create even long expression and keep in mind that they are evaluated recursively, so for example:

`And(Or(A, And(B, C)), And(D, Or(E, F)))` is evaluated as `(A || (B && C)) && (D && (E || F))`

---

## Tick Event

The tick event is a special kind of event, which is raised by the **Application** with a specified interval.
Whenevever initializing the **Applcation** you can specify the tick interval, as in the following example:

```rust
let mut app: Application<Id, Msg, NoUserEvent> = Application::init(
    EventListenerCfg::default()
        .tick_interval(Duration::from_secs(1)),
);
```

with the `tick_interval()` method, we specify the tick interval.
Each time the tick interval elapses, the application runtime will throw a `Event::Tick` which will be forwarded on `tick()` to the
current active component and to all the components subscribed to the `Tick` event.

The purpose of the tick event is to schedule actions based on a certain interval.

---

## Ports

Ports are basically **Event producer** which are handled by the application *Event listener*.
Usually a tui-realm application will consume only input events, or the tick event, but what if we need *some more* events?

We may for example need a worker which fetches a remote server for data. Ports allow you to create automatized workers which will produce the events and if you set up everything correctly, your model and components will be updated.

Let's see now how to setup a *Port*:

1. First we need to define the `UserEvent` type for our application:

    ```rust
    #[derive(PartialEq, Clone, PartialOrd)]
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
    The poll trait tells the application event listener how to poll for events on a *port*:

    ```rust
    impl Poll<UserEvent> for MyHttpClient {
        fn poll(&mut self) -> ListenerResult<Option<Event<UserEvent>>> {
            // ... do something ...
            Ok(Some(Event::User(UserEvent::GotData(data))))
        }
    }
    ```

3. Port setup in application

    ```rust
    let mut app: Application<Id, Msg, UserEvent> = Application::init(
        EventListenerCfg::default()
            .default_input_listener(Duration::from_millis(10))
            .port(
                Box::new(MyHttpClient::new(/* ... */)),
                Duration::from_millis(100),
            ),
    );
    ```

    On the event listener constructor you can define how many ports you want. When you declare a port you need to pass a
    box containing the type implementing the *Poll* trait and an interval.
    The interval defines the interval between each poll to the port.

---

## Implementing new components

---

## Best practices

### Over states

Stati della comp che agiscono sulla mock
