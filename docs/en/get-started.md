# Get Started 🏁

- [Get Started 🏁](#get-started-)
  - [An introduction to realm](#an-introduction-to-realm)
  - [Key Concepts](#key-concepts)
  - [MockComponent Vs. Component](#mockcomponent-vs-component)
    - [The Mock Component](#the-mock-component)
    - [The Component](#the-component)
    - [Event and Commands](#event-and-commands)
  - [Application, Model and View](#application-model-and-view)
    - [Mounting / Umounting](#mounting--umounting)
    - [Focus](#focus)
  - [Event -> (Cmd -> CmdResult) -> Msg](#event---cmd---cmdresult---msg)
  - [Lifecycle (or "tick")](#lifecycle-or-tick)
  - [Our first application](#our-first-application)
  - [What's next](#whats-next)

---

## An introduction to realm

TODO: youtube link

tui-realm is a tui-rs **framework** which provides an easy way to implement stateful application.
First of all, let's give a look to the main features of tui-realm and why you should opt for this framework when building
terminal user interfaces:

- ⌨️ **Event-driven**

    tui-realm uses the `Event -> Msg` approach, taken from Elm. **Events** are produced by some entities called `Port`, which work as event listener (such as a stdin reader or an HTTP client), which produce Events. These are then forwarded to **Components**, which will produce a **Message**. The message will cause then a certain behaviour on your application model, based on its variant.
    Kinda simple and everything in your application will work around this logic, so it's really easy to implement whatever you want.

- ⚛️ Based on **React** and **Elm**

    tui-realm is based on [React] and [Elm](https://elm-lang.org/). These two are kinda different as approach, but I decided to take the best from each of them to combine them in **Realm**. From React I took the **Component** concept. In realm each component represents a single graphic instance, which could potentially include some children; each component then has a **State** and some **Properties**.
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

- **MockComponent**: A Mock component represents a re-usable UI component, which can have some properties for rendering or to handle commands. It can also have its own states, if necessary. In practice it is a trait which exposes some methods to render and to handle properties, states and events. We'll see it in details in the next chapter.
- **Component**: A component is a wrapper around a mock component which represents a single component in your application. It directly takes events and generates messages for the application consumer. Underneath it relies on its Mock component for properties/states management and rendering.
- **State**: The state represents the current state for a component (e.g. the current text in a text input). The state depends on how the user (or other sources) interacts with the component (e.g. the user press 'a', and the char is pushed to the text input).
- **Attribute**: An attribute describes a single property in a component. The attribute shouldn't depend on the component state, but should only be configured by the user when the component is initialized. Usually a mock component exposes many attributes to be configured, and the component using the mock, sets them based on what the user requires.
- **Event**: an event is a **raw** entity describing an event caused mainly but the user (such as a keystroke), but could also be generated by an external source (we're going to talk about these last in the "advanced concepts").
- **Message** (or usually called `Msg`): A message is a Logic event that is generated from the Components, after an **Event**.

    While the Event is *raw* (such as a keystroke), the message is application-oriented. The message is later consumed by the **Update routine**. I think an example would explain it better: let's say we have a popup component, that when `ESC` is pressed, it must report to the application to hide it. Then the event will be `Key::Esc`, it will consume it, and will return a `PopupClose` message. The mesage are totally user-defined through template types, but we'll see that later in this guide.

- **Command** (or usually called `Cmd`): Is an entity generated by the **Component** when it receives an **Event**. It is used by the component to operate on its **MockComponent**. We'll see why of these two entities later.
- **View**: The view is where all the components are stored. The view has basically three tasks:
  - **Managing components mounting/umounting**: components are mounted into the view when they're created. The view prevents to mount duplicated components and will warn you when you try to operate on unexisting component.
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

- *view*: a method which renders the component in the provided area. You must use `tui-rs` widgets to render your component based on its properties and states.
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
    component: Input, // Wehere input implements `MockComponent`
}

impl Component for UsernameInput { ... }
```

Another thing you'll have may noticed and that may frighten some of you are the two generic types that Component takes.

### Event and Commands

---

## Application, Model and View

### Mounting / Umounting

### Focus

---

## Event -> (Cmd -> CmdResult) -> Msg

---

## Lifecycle (or "tick")

---

## Our first application

---

## What's next

You now might be interested in read more about the tui-realm, so you may be interested in these reads:

- [Advanced concepts](advanced.md)
- [Migrating tui-realm 0.x to 1.x](migrating-legacy.md)
- [Implementing new components](new-components.md)
