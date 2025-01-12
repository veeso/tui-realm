# Get Started 🏁

- [Get Started 🏁](#get-started-)
  - [An introduction to realm](#an-introduction-to-realm)
  - [Key Concepts](#key-concepts)
  - [MockComponent Vs. Component](#mockcomponent-vs-component)
    - [The Mock Component](#the-mock-component)
    - [The Component](#the-component)
    - [Properties Vs. States](#properties-vs-states)
    - [Events Vs. Commands](#events-vs-commands)
  - [Application, Model and View](#application-model-and-view)
    - [The View](#the-view)
      - [Focus](#focus)
    - [Model](#model)
    - [The Application](#the-application)
  - [Lifecycle (or "tick")](#lifecycle-or-tick)
  - [Our first application](#our-first-application)
    - [Let's implement the Counter](#lets-implement-the-counter)
    - [Let's define the message type](#lets-define-the-message-type)
    - [Let's define the component identifiers](#lets-define-the-component-identifiers)
    - [Implementing the two counter components](#implementing-the-two-counter-components)
    - [Implementing the model](#implementing-the-model)
    - [Application setup and main loop](#application-setup-and-main-loop)
  - [What's next](#whats-next)

---

## An introduction to realm

What you will learn:

- The key concepts of tui-realm
- How to code a tui-realm application from scratch
- What makes tui-realm cool

tui-realm is a ratatui **framework** which provides an easy way to implement stateful application.
First of all, let's give a look to the main features of tui-realm and why you should opt for this framework when building
terminal user interfaces:

- ⌨️ **Event-driven**

    tui-realm uses the `Event -> Msg` approach, taken from Elm. **Events** are produced by some entities called `Port`, which work as event listener (such as a stdin reader or an HTTP client), which produce Events. These are then forwarded to **Components**, which will produce a **Message**. The message will cause then a certain behaviour on your application model, based on its variant.
    Kinda simple and everything in your application will work around this logic, so it's really easy to implement whatever you want.

- ⚛️ Based on **React** and **Elm**

    tui-realm is based on [React](https://reactjs.org/) and [Elm](https://elm-lang.org/). These two are kinda different as approach, but I decided to take the best from each of them to combine them in **Realm**. From React I took the **Component** concept. In realm each component represents a single graphic instance, which could potentially include some children; each component then has a **State** and some **Properties**.
    From Elm I basically took every other concept implemented in Realm. I really like Elm as a language, in particular the **TEA**.
    Indeed, as in Elm, in realm the lifecycle of the application is `Event -> Msg -> Update -> View -> Event -> ...`

- 🍲 **Boilerplate** code

    tui-realm may look hard to work with at the beginning, but after a while you'll be start realizing how the code you're implementing is just boilerplate code you're copying from your previous components.

- 🚀 Quick-setup

    Since the newest tui-realm API (1.x) tui-realm has become really easy to learn and to setup, thanks to the new `Application` data type, event listeners and to the `Terminal` helper.

- 🎯 Single **focus** and **states** management

    Instead of managing focus and states by yourself, in realm everything is automatically managed by the **View**, which is where all components are mounted. With realm you don't have to worry about the application states and focus anymore.

- 🙂 Easy to learn

    Thanks to the few data types exposed to the user and to the guides, it's really easy to learn tui-realm, even if you've never worked with tui or Elm before.

- 🤖 Adaptable to any use case

    As you will learn through this guide, tui-realm exposes some advanced concepts to create your own event listener, to work with your own event and to implement complex components.

---

## Key Concepts

Let's see now what are the key concepts of tui-realm. In the introduction you've probably read about some of them in **bold**, but let's see them in details now. Key concepts are really important to understand, luckily they're easy to understand and there aren't many of them:

- **MockComponent**: A Mock component represents a re-usable UI component, which can have some **properties** for rendering or to handle commands. It can also have its own **states**, if necessary. In practice it is a trait which exposes some methods to render and to handle properties, states and events. We'll see it in details in the next chapter.
- **Component**: A component is a wrapper around a mock component which represents a single component in your application. It directly takes events and generates messages for the application consumer. Underneath it relies on its Mock component for properties/states management and rendering.
- **State**: The state represents the current state for a component (e.g. the current text in a text input). The state depends on how the user (or other sources) interacts with the component (e.g. the user press 'a', and the char is pushed to the text input).
- **Attribute**: An attribute describes a single property in a component. The attribute shouldn't depend on the component state, but should only be configured by the user when the component is initialized. Usually a mock component exposes many attributes to be configured, and the component using the mock, sets them based on what the user requires.
- **Event**: an event is a **raw** entity describing an event caused mainly by the user (such as a keystroke), but could also be generated by an external source (we're going to talk about these last in the "advanced concepts").
- **Message** (or usually called `Msg`): A message is a Logic event that is generated by the Components, after an **Event**.

    While the Event is *raw* (such as a keystroke), the message is application-oriented. The message is later consumed by the **Update routine**. I think an example would explain it better: let's say we have a popup component, that when `ESC` is pressed, it must report to the application to hide it. Then the event will be `Key::Esc`, it will consume it, and will return a `PopupClose` message. The mesage are totally user-defined through template types, but we'll see that later in this guide.

- **Command** (or usually called `Cmd`): Is an entity generated by the **Component** when it receives an **Event**. It is used by the component to operate on its **MockComponent**. We'll see why of these two entities later.
- **View**: The view is where all the components are stored. The view has basically three tasks:
  - **Managing components mounting/unmounting**: components are mounted into the view when they're created. The view prevents to mount duplicated components and will warn you when you try to operate on unexisting component.
  - **Managing focus**: the view guarantees that only one component at a time is active. The active component is enabled with a dedicated attribute (we'll see that later) and all the events will be forwarded to it. The view keeps track of all the previous active component, so if the current active component loses focus, the previous active one is active if there's no other component to active.
  - **Providing an API to operate on components**: Once components are mounted into the view, they must be accessible to the outside, but in a safe way. That's possible thanks to the bridge methods the view exposes. Since each component must be uniquely identified to be accessed, you'll have to define some IDs for your components.
- **Model**: The model is a structure you'll define for your application to implement the **Update routine**.
- **Subscription** or *Sub*: A subscription is a ruleset which tells the **application** to forward events to other components even if they're not active, based on some rules. We'll talk about subscription in advanced concepts.
- **Port**: A port is an event listener which will use a trait called `Poll` to fetch for incoming events. A port defines both the trait to call and an interval which must elapse between each call. The events are then forwarded to the subscribed components. The input listener is a port, but you may also implement for example an HTTP client, which fetches for some data. We'll see ports in advanced concepts anyway, since they're kinda uncommon to be used.
- **Event Listener**: It is a thread which polls ports to read for incoming events. The events are then reported to the **Application**.
- **Application**: The application is a super wrapper around the *View*, the *Subscriptions* and the *Event Listener*. It exposes a bridge to the view, some shorthands to the *subscriptions*; but is main function, though, is called `tick()`. As we'll see later, tick is where all the framework magic happens.
- **Update routine**: The update routine is a function, which must be implemented by the **Model** and is part of the *Update trait*. This function is as simple as important. It takes as parameter a mutable ref to the *Model*, a mutable ref to the *View* and the incoming **Message**. Based on the value of the *Message*, it provoke a certain behaviour on the Model or on the view. It is just a *match case* if you ask and it can return a *Message*, which will cause the routine to be called recursively by the application. Later, when we'll see the example you'll see how this is just cool.

---

## MockComponent Vs. Component

We've already roughly said what these two entities are, but now it's time to see them in practice.
The first thing we should remind, is that both of them are **Traits** and that by design a *Component* is also a *MockComponent*.
Let's see their definition in details:

### The Mock Component

The mock component is meant to be *generic* (but not too much) and *re-usable*, but at the same time with *one responsibility*.
For instance:

- ✅ A Label which shows a single line of text makes a good mock component.
- ✅ An Input component like `<input>` in HTML is a good mock component. Even if it can handle many input types, it still has one responsibility, is generic and is re-usable.
- ❌ An input which can handle both text, radio buttons and checks is a bad mock component. It is too generic.
- ❌ An input which takes the remote address for a server is a bad mock component. It is not generic.

These are only guidelines, but just to give you the idea of what a mock component is.

A mock component also handles **States** and **Props**, which are totally user-defined based on your needs. Sometimes you may even have component which don't handle any state (e.g. a label).

In practice a mock component is a trait, with these methods to be implmented:

```rust
pub trait MockComponent {
    fn view(&mut self, frame: &mut Frame, area: Rect);
    fn query(&self, attr: Attribute) -> Option<AttrValue>;
    fn attr(&mut self, attr: Attribute, value: AttrValue);
    fn state(&self) -> State;
    fn perform(&mut self, cmd: Cmd) -> CmdResult;
}
```

the trait requires you to implement:

- *view*: a method which renders the component in the provided area. You must use `ratatui` widgets to render your component based on its properties and states.
- *query*: returns the value for a certain attribute in the component properties.
- *attr*: assign a certain attribute to the component properties.
- *state*: get the current component state. If has no state will return `State::None`.
- *perform*: Performs the provided **command** on the component. This method is called by the **Component** as we'll see later. The command should change the component states. Once the action has been processed, it must return the `CmdResult` to the **Component**.

### The Component

So, apparently the mock component defines everything we need handle properties, states and rendering. So why we're not done yet and we need a component trait too?

1. MockComponent must be **generic**: mock components are distribuited in library (e.g. `tui-realm-stdlib`) and because of that, they cannot consume `Event` or produce `Message`.
2. Because of point 1, we need an entity which produces `Msg` and consume `Event`. These two entities are totally or partially user-defined, which means, they are different for each realm application. This means the component must fit to the application.
3. **It's impossible to fit a component to everybody's needs**: I tried to in tui-realm 0.x, but it was just impossible. At a certain point I just started to add properties among other properties, but eventually I ended up re-implementing stdlib components from scratch just to have some different logics. Mock Components are good because they're generic, but not too much; they must behave as dummies to us. Components are exactly what we want for the application. We want an input text, but we want that when we type 'a' it changes color. You can do it with component, you can't do it with mocks. Oh, and I almost forgot the worst thing about generalizing mocks: **keybindings**.

Said so, what is a component?

A component is an application specific unique implementation of a mock. Let's think for example of a form and let's say the first field is an input text which takes the username. If we think about it in HTML, it will be for sure a `<input type="text" />` right? And so it's for many other components in your web page. So the input text will be the `MockComponent` in tui-realm. But *THAT* username input field, will be your **username input text**. The `UsernameInput` will wrapp a `Input` mock component, but based on incoming events it will operate differently on the mock and will produce different **Messages** if compared for instance to a `EmailInput`.

So, let me state the most important thing you must keep in mind from now on: **Components are unique ❗** in your application. You **should never use the same Component more than once**.

Let's see what a component is in practice now:

```rust
pub trait Component<Msg, UserEvent>: MockComponent
where
    Msg: PartialEq,
    UserEvent: Eq + PartialEq + Clone + PartialOrd,
{
    fn on(&mut self, ev: Event<UserEvent>) -> Option<Msg>;
}
```

Quite simple uh? Yep, it was my intention to make them the lighter as possible, since you'll have to implement one for each component in your view. As you can also notice, a Component requires to impl a `MockComponent` so in practice we'll also have something like:

```rust
pub struct UsernameInput {
    component: Input, // Where input implements `MockComponent`
}

impl Component for UsernameInput { ... }
```

Another thing you may have noticed and that may frighten some of you are the two generic types that Component takes.
Let's see what these two types are:

- `Msg`: defines the type of the **message** your application will handle in the **Update routine**. Indeed, in tui-realm the message are not defined in the library, but are defined by the user. We'll see this in details later in "the making of the first application". The only requirements for Message, is that it must implement `PartialEq`, since you must be able to match it in the **Update**.
- `UserEvent`: The user event defines a custom event your application can handle. As we said before tui-realm usually will send events concerning user input or terminal events, plus a special event called `Tick` (but we'll talk about it later). In addition to these though, we've seen there are other special entities called `Port`, which may return events from other source. Since tui-realm needs to know what these events are, you need to provide the type your ports will produce.

    If we give a look to the `Event` enum, everything will become clear.

    ```rust
    pub enum Event<UserEvent>
    where
        UserEvent: Eq + PartialEq + Clone + PartialOrd,
    {
        /// A keyboard event
        Keyboard(KeyEvent),
        /// This event is raised after the terminal window is resized
        WindowResize(u16, u16),
        /// A ui tick event (should be configurable)
        Tick,
        /// Unhandled event; Empty event
        None,
        /// User event; won't be used by standard library or by default input event listener;
        /// but can be used in user defined ports
        User(UserEvent),
    }
    ```

    As you can see there is a special variant for `Event` called `User` which takes a special type `UserEvent`, which can be indeed used to use user-defined events.

    > ❗If you don't have any `UserEvent` in your application, you can declare events passing `Event<NoUserEvent>`, which is an empty enum

### Properties Vs. States

All components are described by properties and quite often by states as well. But what is the difference between them?

Basically **Properties** describe how the component is rendered and how it should behave.

For example, properties are **styles**, **color** or some properties such as "should this list scroll?".
Properties are always present in a component.

States, on the other hand, are optional and *usually* are used only by components which the user can interact with.
The state won't describe styles or how a component behaves, but the current state of a component. The state, also will usually change after the user performs a certain **Command**.

Let's see for example how to distinguish properties from states on a component and let's say this component is a *Checkbox*:

- The checkbox foreground and background are **Properties** (doesn't change on interaction)
- The checkbox options are **Properties**
- The current selected options are **States**. (they change on user interaction)
- The current highlighted item is a **State**.

### Events Vs. Commands

We've almost seen all of the aspects behind components, but we still need to talk about an important concept, which is the difference between `Event` and `Cmd`.

If we give a look to the **Component** trait, we'll see that the method `on()` has the following signature:

```rust
fn on(&mut self, ev: Event<UserEvent>) -> Option<Msg>;
```

and we know that the `Component::on()` will call the `perform()` method of its **MockComponent**, in order to update its states. The perform method has this signature instead:

```rust
fn perform(&mut self, cmd: Cmd) -> CmdResult;
```

As you can see, the **Component** consumes an `Event` and produces a `Msg`, while the mock, which is called by the component, consumes a `Cmd` and produces a `CmdResult`.

If we give a look to the two type declarations, we'll see there is a difference in terms of scope, let's give a look:

```rust
pub enum Event<UserEvent>
where
    UserEvent: Eq + PartialEq + Clone + PartialOrd,
{
    /// A keyboard event
    Keyboard(KeyEvent),
    /// This event is raised after the terminal window is resized
    WindowResize(u16, u16),
    /// A ui tick event (should be configurable)
    Tick,
    /// Unhandled event; Empty event
    None,
    /// User event; won't be used by standard library or by default input event listener;
    /// but can be used in user defined ports
    User(UserEvent),
}

pub enum Cmd {
    /// Describes a "user" typed a character
    Type(char),
    /// Describes a "cursor" movement, or a movement of another kind
    Move(Direction),
    /// An expansion of `Move` which defines the scroll. The step should be defined in props, if any.
    Scroll(Direction),
    /// User submit field
    Submit,
    /// User "deleted" something
    Delete,
    /// User toggled something
    Toggle,
    /// User changed something
    Change,
    /// A user defined amount of time has passed and the component should be updated
    Tick,
    /// A user defined command type. You won't find these kind of Command in the stdlib, but you can use them in your own components.
    Custom(&'static str),
    /// `None` won't do anything
    None,
}
```

For some aspects, they both look similiar, but something immediately appears clear:

- Event is strictly bounded to the "hardware", it takes key event, terminal events or event from other sources.
- Cmd is completely independent from the hardware and terminal, and it's all about UI logic. We still have `KeyEvent`, but we've also got `Type`, `Move`, `Submit`, custom events (but not with generics) and etc.

The reason behind this, is quite simple: **MockComponent** must be application-independent. You can create your components library and distribuite it on Github, or wherever you want, and it still must be able to work. If they took events as parameters, this couldn't be possible, since event takes in a type, which is application-dependent.

And there's also another reason: let's imagine we have a component with a list you can scroll on and view different elements. You can scroll up/down with keys. If I wanted to create a library of components and we had events only, it wouldn't be possible to use different keybindings. Think about, with mock components I expect that in perform(), when we receive a `Cmd::Scroll(Direction::Up)` the list scrolls up, then I can implement my `Component` which will send a `Cmd::Scroll(Direction::Up)` when `W` is typed and another component which will send the same event when `<UP>` is pressed. Thanks to this mechanism, tui-realm mock components are also totally independent from key-bindings, which in tui-realm 0.x, was just a hassle.

So whenever you implement a MockComponent, you must keep in mind that you should make it application-independent, so you must define its **Command API** and define what kind of **CmdResult** it'll produce. Then, your components must generate on whatever kind of events the `Cmd` accepted by the API, and handle the `CmdResult`, and finally, based on the value of the `CmdResult` return a certain kind of **Message** based on your application.

We're then, finally starting to define the lifecycle of the tui-realm. This segment of the cycle, is described as `Event -> (Cmd -> CmdResult) -> Msg`.

---

## Application, Model and View

Now that we have defined what Components are, we can finally start talking about how all these components can be put together to create an application.

In order to put everything together, we'll use three different entities, we've already briefly seen before, which are:

- The **Application**
- The **Model**
- The **View**

First, starting from components, the first thing we need to talk about, is the **View**.

### The View

The view is basically a box for all the components. All the components which are part of the same "view" (in terms of UI) must be *mounted* in the same **View**.
Each component in the view, **Must** be identified uniquely by an *identifier*, where the identifier is a type you must define (you can use an enum, you can use a String, we'll see that later).
Once a component is mounted, it won't be directly usable anymore. The view will store it as a generic `Component` and will expose a bridge to operate on all the components in the view, querying them with their identifier.

The component will be part of the view, until you unmount the component. Once the component is unmounted, it won't be usable anymore and it'll be destroyed.

The view is not just a list of components though, it also plays a fundamental role in the UI development, indeed, it will handle focus. Let's talk about it in the next chapter

#### Focus

Whenever you interact with components in a UI, there must always be a way to determine which component will handle the interaction. If I press a key, the View must be able whether to type a character in an input field or into another and this is resolved through **focus**.
Focus is just a state the view tracks. At any time, the view must know which component is currently *active* and what to do, in case that component is unmounted.

In tui-realm, I decided to define the following rules, when working with focus:

1. Only one component at a time can have focus
2. All events will be forwarded to the component that currently owns focus.
3. A componet to become active, must get focus via the `active()` method.
4. If a component gets focus, then its `Attribute::Focus` property becomes `AttrValue::Flag(true)`
5. If a component loses focus, then its `Attribute::Focus` property becomes `AttrValue::Flag(false)`
6. Each time a component gets focus, the previous active component, is tracked into a `Stack` (called *focus stack*) holding all the previous components owning focus.
7. If a component owning focus, is **unmounted**, the first component in the **Focus stack** becomes active
8. If a component owning focus, gets disabled via the `blur()` method, the first component in the **Focus stack** becomes active, but the *blurred* component, is not pushed into the **Focus stack**.

Follow the following table to understand how focus works:

| Action   | Focus | Focus Stack | Components |
|----------|-------|-------------|------------|
| Active A | A     |             | A, B, C    |
| Active B | B     | A           | A, B, C    |
| Active C | C     | B, A        | A, B, C    |
| Blur C   | B     | A           | A, B, C    |
| Active C | C     | B, A        | A, B, C    |
| Active A | A     | C, B        | A, B, C    |
| Umount A | C     | B           | B, C       |
| Mount D  | C     | B           | B, C, D    |
| Umount B | C     |             | C, D       |

### Model

The model is a struct which is totally defined by the developer implementing a tui-realm application. Its purpose is basically to update its states, perform some actions or update the view, after the components return messages.
This is done through the **Update routine**, which is defined in the **Update** trait. We'll soon see this in details, when we'll talk about the *application*, but for now, what we need to know, is what the update routine does:

first of all your *model* must implement the **Update** trait:

```rust
pub trait Update<ComponentId, Msg, UserEvent>
where
    ComponentId: Eq + PartialEq + Clone + Hash,
    Msg: PartialEq,
    UserEvent: Eq + PartialEq + Clone + PartialOrd,
{

    /// update the current state handling a message from the view.
    /// This function may return a Message,
    /// so this function has to be intended to be call recursively if necessary
    fn update(
        &mut self,
        view: &mut View<ComponentId, Msg, UserEvent>,
        msg: Option<Msg>,
    ) -> Option<Msg>;
}
```

Here finally we can see almost everything put together: we have the view and we have all the 3 different custom types, defining how components are identified in the view (*ComponentId*), the *Msg* and the *UserEvent* for the *Event* type.
The update method, receives a mutable reference to the model, a mutable reference to the view and the incoming message from the component, which processed a certain type of event.
Inside the update, we'll match the msg, to perform certain operation on the model or on the view and we'll return `None` or another message, if necessary. As we'll see, if we return `Some(Msg)`, the *Application*, will re-call the routine passing as argument the last generated message.

### The Application

Finally we're ready to talk about the core struct of tui-realm, the **Application**. Let's see which tasks it takes care of:

- It contains the view and exposes a bridge to it: the application contains the view itself, and provides a way to operate on it, as usual using the component identifiers.
- It handles subscriptions: as we've already seen before, subscriptions are special rules which tells the application to forward events to other components if some clauses are satisfied.
- It reads incoming events from **Ports**

indeed as we can see, the application is a container for all these entities:

```rust
pub struct Application<ComponentId, Msg, UserEvent>
where
    ComponentId: Eq + PartialEq + Clone + Hash,
    Msg: PartialEq,
    UserEvent: Eq + PartialEq + Clone + PartialOrd + Send + 'static,
{
    listener: EventListener<UserEvent>,
    subs: Vec<Subscription<ComponentId, UserEvent>>,
    view: View<ComponentId, Msg, UserEvent>,
}
```

so the application will be the sandbox for the all the entities a tui-realm app needs (and that's why is called *Application*).

But the coolest thing here, is that all the application can be run, using a single method! This method is called `tick()` and as we'll see in the next chapter it performs all what is necessary to complete a single cycle of the application lifecycle:

```rust
pub fn tick(&mut self, strategy: PollStrategy) -> ApplicationResult<Vec<Msg>> {
    // Poll event listener
    let events = self.poll(strategy)?;
    // Forward to active element
    let mut messages: Vec<Msg> = events
        .iter()
        .map(|x| self.forward_to_active_component(x.clone()))
        .flatten()
        .collect();
    // Forward to subscriptions and extend vector
    messages.extend(self.forward_to_subscriptions(events));
    Ok(messages)
}
```

As we can quickly see, the tick method has the following workflow:

1. The event listener is fetched according to the provided `PollStrategy`

    > ❗The poll strategy tells how to poll the event listener. You can fetch One event for cycle, or up to `n` or for a maximum amount of time

2. All the incoming events are immediately forwarded to the current *active* component in the *view*, which may return some *messages*
3. All the incoming events are sent to all the components subscribed to that event, which satisfied the clauses described in the subscription. They, as usual, will may return some *messages*
4. The messages are returned

Along to the tick() routine, the application provides many other functionalities, but we'll see later in the example and don't forget to checkout the documentation on rust docs.

---

## Lifecycle (or "tick")

We're finally ready to put it all together to see the entire lifecycle of the application.
Once the application is set up, the cycle of our application will be the following one:

![lifecycle](/docs/images/lifecycle.png)

in the image, we can see there are all the entities we've talked about earlier, which are connected through two kind of arrows, the *black* arrows defines the flow you have to implement, while the *red* arrows, follows what is already implemented and implicitly called by the application.

So the tui-realm lifecycle consists in:

1. the `tick()` routine is called on **Application**
   1. Ports are polled for incoming events
   2. event is forwarded to active component in the view
   3. subscriptions are queried to know whether the event should be forwarded to other components
   4. incoming messages are collected
2. Messages are returned to the caller
3. the `update()` routine is called on **Model** providing each message from component
4. The model gets updated thanks to the `update()` method
5. The `view()` function is called to render the UI

This simple 4 steps cycle is called **Tick**, because it defines the interval between each UI refresh in fact.
Now that we know how a tui-realm application works, let's see how to implement one.

---

## Our first application

We're finally ready to set up a realm tui-realm application. In this example we're going to start with simple very simple.
The application we're going to implement is really simple, we've got two **counters**, one will track when an alphabetic character is pressed by the user and the other when a digit is pressed by the user. Both of them will track events, only when active. The active component will switch between the two counters pressing `<TAB>`, while pressing `<ESC>` the application will terminate.

> ❗ Want to see something more complex? Check out [tuifeed](https://github.com/veeso/tuifeed)

### Let's implement the Counter

So we've said we have two Counters, one tracking alphabetic characters and one digits, so we've found a potential mock component: the **Counter**. The counter will just have a state keeping track of "times" as a number and will increment each time a certain command will be sent.
Said so, let's implement the counter:

```rust
struct Counter {
    props: Props,
    states: OwnStates,
}

impl Default for Counter {
    fn default() -> Self {
        Self {
            props: Props::default(),
            states: OwnStates::default(),
        }
    }
}
```

so the counter, as all components must have the `props` which defines its properties and in this case the counter is a stateful component, so, we need to declare its states:

```rust
struct OwnStates {
    counter: isize,
}

impl Default for OwnStates {
    fn default() -> Self {
        Self { counter: 0 }
    }
}

impl OwnStates {
    fn incr(&mut self) {
        self.counter += 1;
    }
}
```

Then, we'll implement an easy-to-use constructor for our mock component:

```rust
impl Counter {
    pub fn label<S>(mut self, label: S) -> Self
    where
        S: AsRef<str>,
    {
        self.attr(
            Attribute::Title,
            AttrValue::Title((label.as_ref().to_string(), Alignment::Center)),
        );
        self
    }

    pub fn value(mut self, n: isize) -> Self {
        self.attr(Attribute::Value, AttrValue::Number(n));
        self
    }

    pub fn alignment(mut self, a: Alignment) -> Self {
        self.attr(Attribute::TextAlign, AttrValue::Alignment(a));
        self
    }

    pub fn foreground(mut self, c: Color) -> Self {
        self.attr(Attribute::Foreground, AttrValue::Color(c));
        self
    }

    pub fn background(mut self, c: Color) -> Self {
        self.attr(Attribute::Background, AttrValue::Color(c));
        self
    }

    pub fn modifiers(mut self, m: TextModifiers) -> Self {
        self.attr(Attribute::TextProps, AttrValue::TextModifiers(m));
        self
    }

    pub fn borders(mut self, b: Borders) -> Self {
        self.attr(Attribute::Borders, AttrValue::Borders(b));
        self
    }
}
```

finally we can implement `MockComponent` for `Counter`

```rust
impl MockComponent for Counter {
    fn view(&mut self, frame: &mut Frame, area: Rect) {
        // Check if visible
        if self.props.get_or(Attribute::Display, AttrValue::Flag(true)) == AttrValue::Flag(true) {
            // Get properties
            let text = self.states.counter.to_string();
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
            let title = self
                .props
                .get_or(
                    Attribute::Title,
                    AttrValue::Title((String::default(), Alignment::Center)),
                )
                .unwrap_title();
            let borders = self
                .props
                .get_or(Attribute::Borders, AttrValue::Borders(Borders::default()))
                .unwrap_borders();
            let focus = self
                .props
                .get_or(Attribute::Focus, AttrValue::Flag(false))
                .unwrap_flag();
            frame.render_widget(
                Paragraph::new(text)
                    .block(get_block(borders, title, focus))
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
        State::One(StateValue::Isize(self.states.counter))
    }

    fn perform(&mut self, cmd: Cmd) -> CmdResult {
        match cmd {
            Cmd::Submit => {
                self.states.incr();
                CmdResult::Changed(self.state())
            }
            _ => CmdResult::None,
        }
    }
}
```

so as state, we return the current value for the counter and on perform we handle the `Cmd::Submit` to increment the current value for the counter. As result we return `CmdResult::Changed()` with the state.

So our Mock component is ready, we can now implement our two components.

### Let's define the message type

Before implementing the two `Component` we first need to define the messages our application will handle.
So, in on top of our application we define an enum `Msg`:

```rust
#[derive(Debug, PartialEq)]
pub enum Msg {
    AppClose,
    DigitCounterChanged(isize),
    DigitCounterBlur,
    LetterCounterChanged(isize),
    LetterCounterBlur,
    /// Used to unwrap on update()
    None,
}
```

where:

- `AppClose` will tell to terminate the app
- `DigitCounterChanged` tells the digit counter value has changed
- `DigitCounterBlur` tells that the digit counter shall lose focus
- `LetterCounterChanged` tells the letter counter value has changed
- `LetterCounterBlur` tells that the letter counter shall lose focus

### Let's define the component identifiers

We need also to define the ids for our components, that will be used by the view to query mounted components.
So on top of our application, as we did for `Msg`, let's define `Id`:

```rust
// Let's define the component ids for our application
#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum Id {
    DigitCounter,
    LetterCounter,
}
```

### Implementing the two counter components

We'll have two type of counters, so we'll call them `LetterCounter` and `DigitCounter`. Let's implement them!

First we define the `LetterCounter` with the mock component within. Since we don't need any particular behaviour for the `MockComponent` trait, we can simply derive `MockComponent`, which will implement the default implementation for MockComponent. If you want to read more read see [tuirealm_derive](https://github.com/veeso/tuirealm_derive).

```rust
#[derive(MockComponent)]
pub struct LetterCounter {
    component: Counter,
}
```

then we implement the constructor for the counter, that accepts the initial value and construct a `Counter` using the mock component constructor:

```rust
impl LetterCounter {
    pub fn new(initial_value: isize) -> Self {
        Self {
            component: Counter::default()
                .alignment(Alignment::Center)
                .background(Color::Reset)
                .borders(
                    Borders::default()
                        .color(Color::LightGreen)
                        .modifiers(BorderType::Rounded),
                )
                .foreground(Color::LightGreen)
                .modifiers(TextModifiers::BOLD)
                .value(initial_value)
                .label("Letter counter"),
        }
    }
}
```

Finally we implement the `Component` trait for the `LetterCounter`, were we first convert the incoming `Event` to a consumable `Cmd`, then we call `perform()` on the mock to get the `CmdResult` in order to produce a `Msg`.
When event is `Esc` or `Tab` we directly return the `Msg` to close app or to change focus.

```rust
impl Component<Msg, NoUserEvent> for LetterCounter {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        // Get command
        let cmd = match ev {
            Event::Keyboard(KeyEvent {
                code: Key::Char(ch),
                modifiers: KeyModifiers::NONE,
            }) if ch.is_alphabetic() => Cmd::Submit,
            Event::Keyboard(KeyEvent {
                code: Key::Tab,
                modifiers: KeyModifiers::NONE,
            }) => return Some(Msg::LetterCounterBlur), // Return focus lost
            Event::Keyboard(KeyEvent {
                code: Key::Esc,
                modifiers: KeyModifiers::NONE,
            }) => return Some(Msg::AppClose),
            _ => Cmd::None,
        };
        // perform
        match self.perform(cmd) {
            CmdResult::Changed(State::One(StateValue::Isize(c))) => {
                Some(Msg::LetterCounterChanged(c))
            }
            _ => None,
        }
    }
}
```

We'll do the same for the `DigitCounter`, but on `on()` it will check whether char is a digit, instead of alphabetic.

### Implementing the model

Now that we have the components, we're almost done. We can finally implement the `Model`. I made a very simple model for this example:

```rust
pub struct Model {
    /// Application
    pub app: Application<Id, Msg, NoUserEvent>,
    /// Indicates that the application must quit
    pub quit: bool,
    /// Tells whether to redraw interface
    pub redraw: bool,
    /// Used to draw to terminal
    pub terminal: TerminalBridge,
}
```

> ❗ the terminal bridge is a helper struct implemented in tui-realm to interface with ratatui terminal with some helper functions.
> It also is totally backend-independent, so you won't have to know how to setup the terminal for your backend.

Now, we'll implement the `view()` method, which will render the GUI after updating the model:

```rust
impl Model {
    pub fn view(&mut self, app: &mut Application<Id, Msg, NoUserEvent>) {
        assert!(self
            .terminal
            .raw_mut()
            .draw(|f| {
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(1)
                    .constraints(
                        [
                            Constraint::Length(3), // Letter Counter
                            Constraint::Length(3), // Digit Counter
                        ]
                        .as_ref(),
                    )
                    .split(f.size());
                app.view(&Id::LetterCounter, f, chunks[0]);
                app.view(&Id::DigitCounter, f, chunks[1]);
            })
            .is_ok());
    }
}
```

> ❗ If you're not familiar with the `draw()` function, please read the [ratatui](https://ratatui.rs/) documentation.

and finally we can implement the `Update` trait:

```rust
impl Update<Msg> for Model {
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
                Msg::Clock => None,
                Msg::DigitCounterBlur => {
                    // Give focus to letter counter
                    assert!(self.app.active(&Id::LetterCounter).is_ok());
                    None
                }
                Msg::DigitCounterChanged(v) => {
                    // Update label
                    assert!(self
                        .app
                        .attr(
                            &Id::Label,
                            Attribute::Text,
                            AttrValue::String(format!("DigitCounter has now value: {}", v))
                        )
                        .is_ok());
                    None
                }
                Msg::LetterCounterBlur => {
                    // Give focus to digit counter
                    assert!(self.app.active(&Id::DigitCounter).is_ok());
                    None
                }
                Msg::LetterCounterChanged(v) => {
                    // Update label
                    assert!(self
                        .app
                        .attr(
                            &Id::Label,
                            Attribute::Text,
                            AttrValue::String(format!("LetterCounter has now value: {}", v))
                        )
                        .is_ok());
                    None
                }
            }
        } else {
            None
        }
    }
}
```

### Application setup and main loop

We're almost done, let's just setup the Application in our `main()`:

```rust
fn init_app() -> Application<Id, Msg, NoUserEvent> {
    // Setup application
    // NOTE: NoUserEvent is a shorthand to tell tui-realm we're not going to use any custom user event
    // NOTE: the event listener is configured to use the default crossterm input listener and to raise a Tick event each second
    // which we will use to update the clock
    let mut app: Application<Id, Msg, NoUserEvent> = Application::init(
        EventListenerCfg::default()
            .default_input_listener(Duration::from_millis(20))
            .poll_timeout(Duration::from_millis(10))
            .tick_interval(Duration::from_secs(1)),
    );
}
```

The app requires the configuration for the `EventListener` which will poll `Ports`. We're telling the event listener to use the default input listener for our backend. `default_input_listener` will setup the default input listener for termion/crossterm or the backend you chose. Then we also define the `poll_timeout`, which describes the interval between each poll to the listener thread.

> ❗ Here we could also define other Ports thanks to the method `port()` or setup the `Tick` producer with `tick_interval()`

Then we can mount the two components into the view:

```rust
assert!(app
    .mount(
        Id::LetterCounter,
        Box::new(LetterCounter::new(0)),
        Vec::default()
    )
    .is_ok());
assert!(app
    .mount(
        Id::DigitCounter,
        Box::new(DigitCounter::new(5)),
        Vec::default()
    )
.is_ok());
```

> ❗ The two empty vectors are the subscriptions related to the component. (In this case none)

Then we initilize focus:

```rust
assert!(app.active(&Id::LetterCounter).is_ok());
```

We can now setup the terminal configuration:

```rust
let _ = model.terminal.enter_alternate_screen();
let _ = model.terminal.enable_raw_mode();
```

and we can finally implement the **main loop**:

```rust
while !model.quit {
    // Tick
    match app.tick(&mut model, PollStrategy::Once) {
        Err(err) => {
            // Handle error...
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
        model.view(&mut app);
        model.redraw = false;
    }
}
```

On each cycle we call `tick()` on our application, with strategy `Once` and we ask the model to redraw the view only if at least one message has been processed (otherwise there shouldn't be any change to display).

Once `quit` becomes true, the application terminates, but don't forget to finalize the terminal:

```rust
let _ = model.terminal.leave_alternate_screen();
let _ = model.terminal.disable_raw_mode();
let _ = model.terminal.clear_screen();
```

---

## What's next

Now you know pretty much how tui-realm works and its essential concepts, but there's still a lot of features to explore, if you want to discover them, you now might be interested in these reads:

- [Advanced concepts](advanced.md)
- [Migrating tui-realm 0.x to 1.x](migrating-legacy.md)
