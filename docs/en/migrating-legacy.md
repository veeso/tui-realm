# Migrating from tui-realm 0.x

- [Migrating from tui-realm 0.x](#migrating-from-tui-realm-0x)
  - [Introduction](#introduction)
  - [Why tui-realm 1.x 🤔](#why-tui-realm-1x-)
    - [What didn't work with the old API](#what-didnt-work-with-the-old-api)
  - [What changed](#what-changed)
  - [How to migrate to tui-realm 1.x](#how-to-migrate-to-tui-realm-1x)

---

## Introduction

This guide will explain you how you should migrate a tui-realm 0.x application to a 1.x application, step by step.

What you will learn:

- Why you should migrate to tui-realm 1.x
- What is new and what has been deprecated
- How to actually migrate the application, step-by-step.

## Why tui-realm 1.x 🤔

Let's say that tui-realm 1.x has always been in my plans, since tui-realm 0.x came out.
I've always dreamt of an event-driven framework for my project "termscp", but the problem were basically two:

- I really didn't have a clear idea how to implement something like this
- I didn't really want to work on a framework that may have failed
- tui-realm wasn't a thing at the beginning. Its first version was embedded into termscp, so I didn't even want it to become a public crate.

But many things have changed since there:

- I've become much more confident with Rust and I pretty much know how to use advanced concepts
- My new job has improved me so much, that now I'm much better at programming and designing libraries.
- I saw that tui-realm actually worked and not only for me

So everything seemed to say "you should implement the new api for tui-realm", but was it really necessary?

### What didn't work with the old API

I hope I'm not the only one thinking that, but the old API for tui-realm was just bad, in particular:

- No abstraction over events: components directly handled Keyboard events and that sucked. Especially for the stdlib. If a user wanted to use "WASD" over "arrow keys" to move he had to re-implement an entire component just to have this behaviour. This just sucks. Could I have implement a `keymap` for components. Yes, but just a patch over another.s
- The props system was a good idea, but it was terrible in the implementation. It was just too limited in what you could store, and I changed it in each version, because it just was **TOO LIMITED**. At the very beginning there were some static attributes for text, alignment, styles, etc. Then there was an HashMap, and finally there was the `own` attribute which was a map of string and `PropPayload`. And you know what? It sucked too. I've never understood why I've never thought of something simple as the new props system of tui-realm 1.x. And I almost forgot the worst part: props builders. Really boring to implement and ever worse to use.
- Crossterm 0.20: when crossterm 0.20 was out I got mad really. They removed `Eq` from `KeyEvent` and that caused everybody to implement **Horrible** match cases in the update routine. This is probably one of the strongest points that made me implement tui-realm 1.x
- `Msg` was not a `Msg`: Messages were a good idea (of course it was, I copied it from Elm lang!!!), but they were static, no-user-defined and overall useless. You just endeed up with huge update functions to match them.
- No support for other backends: do I even need to explain this point? Only crossterm was supported. And I still think crossterm is good, since it runs on every platform. BUUUUUT, if you don't need windows support, termion is much better. And people have tastes. And if a user doesn't like crossterm, that user won't be using tui-realm.
- The setup was just too long: setting up a realm application took too long.

---

## What changed

If you've read the previous guides, you'll have already seen what's the entities of the new api, but don't panic, even if it doesn't seem, many things have still the same purpose, but are just fancier:

- Props is still a thing, but instead of holding random properties, it now holds a map of `Attribute` and `AttrValue`. This is really like CSS, which we all hate, but we all know that is works fine.
- PropsBuilder has been replaced by the constructors for mock components
- Msg is no more defined in tui-realm, you define your OWN messages for YOUR application. No more useless messages travelling around the application. Just the messages that you actually need.
- Event and KeyEvent now finally suppport Eq, since I wrapped crossterm structures in tui-realm.
- Crossterm is no more mandatory, you can finally use termion and whatever you like (actually only termion is implemented, but you can implement the other backends supported by tui. But really, is there anyone using rustbox out there?)
- View has been partially replaced by application. I mean, there is still a view, you you hold an application in your program to work with the view.
- The **update trait** is now mandatory (:feelsgood) in order to call the `tick()` method on the application.
- Component has been replaced by MockComponent (and method names have been changed).
- You need to implement a Component for all the elements in your UI.

---

## How to migrate to tui-realm 1.x

Now it's time to see how to do it. I will be very quick in explaining this, but don't expect this to be quick for you.

I will be honest with you: migrating an application **WON'T** be **FAST**, but it'll be easy though, just boring.
Take your time to migrate your application, work on a brand new branch and really: take your time. It'll may take several hours to complete the migration, but trust me: when you finish, you'll really feel satisfied, seeing how much better the application will look like.

Now let's see step-by-step how to perform the migration:

1. Update your dependencies in Cargo.toml

    Do you still want to use crossterm or you want to go with termion?

    If you want to stay with crossterm:

    ```toml
    tuirealm = "^1.0.0"
    ```

    If you want to opt for termion:

    ```toml
    tuirealm = { "version" = "^1.0.0", default-features = false, features = [ "derive", "with-termion" ] }
    ```

    Don't worry about migrating crossterm to termion. It won't be necessary. We'll see later why.

    Oh, don't forget to migrate the stdlib if you use it (I know you use it 😉)

    ```toml
    tui-realm-stdlib = "^1.0.0"
    ```

    or use your favourite backend (must match with tuirealm!)

    ```toml
    tui-realm-stdlib = { "version" = "^1.0.0", default-features = false, features = [ "with-termion" ] }
    ```

2. Remove all the terminal constructors! Let's go for the TerminalBridge

    In tui-realm 1.x I've implemented the `TerminalBridge` to have an abstraction layer with the terminal, in order to have the same API across all backends (I know, this shouldn't be implemented in realm, but in tui-rs. But...).

    So first of all remove all the methods to enter/leave alternate screen and toggling raw mode and replace the terminal in your context with:

    ```rust
    use tuirealm::terminal::TerminalBridge;
    
    Context {
      // ...
      terminal: TerminalBridge::new().expect("Could not initialize terminal"),
    }
    ```

    The terminal bridge is all you need to work with the terminal and provides the same methods on both termion and crossterm or whater you use.

3. Define an enum with all the ids of the components you use

    Probably in tui-realm 0.x you used some constants as id for your components. In tui-realm 1.x we need to use
    a type as identifier for our components, which must be then provided to the application to work.
    You can still use string, but strings suck and enums are much better:

    ```rust
    #[derive(Debug, Eq, PartialEq, Clone, Hash)]
    pub enum Id {
        AddressInput,
        PasswordInput,
        ProtocolRadio,
        GlobalListener,
    }
    ```

4. Define the Message your application will handle

    Take your time to think about what kind of message your application will handle.
    Remember: messages are events that your **Model** or **View** need to be concerned of and **NOT** what components need to receive.

    ```rust
    #[derive(Debug, PartialEq)]
    pub enum Msg {
        AppClose,
        FormSubmit,
        ProtocolChanged(FileTransferProtocol),
        None,
    }
    ```

5. Split the model from the view

    In your current implementation you may have a structure which both holds the model data and the view. This is no longer valid (won't build). You need to have a structure which holds the model structure and the application:

    Then:

    ```rust
    struct Activity {
      context: Context,
      protocol: FileTransferProtocol,
      address: String,
      view: View, // Replaced by application at user-level
    }
    ```

    Now:

    ```rust
    struct Activity {
      model: Model,
      application: Application<Id, Msg, NoUserEvent>,
    }

    struct Model {
      context: Context,
      protocol: FileTransferProtocol,
      address: String,
    }

    impl Update for Model {
      // ... (will be using the view passed by application, that's why model cannot hold view)
    }
    ```

6. Implement a Component trait for each component you're going to use

    Take your time to do this, it'll take a long time. Basically you need to implement a `Component` for all the components in your application.
    The component will always have `component: impl MockComponent` as attribute, which will use a Mock component implemented by you or by the stdlib. If you're using a stdlib component, remember to use the command api to match event and results. Remember that you don't have to implement `MockComponent` for your component (unless you need to specify alternative behaviours), there is a magic `#[derive(MockComponent)]` procedural macro.
    In the constructor of your component, you'll specify everything you used to set in the props builder before:

    Then:

    ```rust
    InputPropsBuilder::default()
      .with_foreground(fg)
      .with_borders(Borders::ALL, BorderType::Rounded, fg)
      .with_label(label, Alignment::Left)
      .with_input(typ);
    ```

    Now:

    ```rust
    use tui_realm_stdlib::Input;

    #[derive(MockComponent)]
    pub struct AddressInput {
        component: Input,
    }

    impl Default for AddressInput {
        fn default() -> Self {
            Self {
                component: Input::default()
                    .foreground(Color::LightBlue)
                    .borders(
                        Borders::default()
                            .color(Color::LightBlue)
                            .modifiers(BorderType::Rounded),
                    )
                    .input_type(InputType::Text)
                    .placeholder(
                        "192.168.1.10",
                        Style::default().fg(Color::Rgb(120, 120, 120)),
                    )
                    .title("Remote address", Alignment::Left),
            }
        }
    }

    impl Component<Msg, NoUserEvent> for AddressInput {
        fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
            let result = match ev {
                Event::Keyboard(KeyEvent {
                    code: Key::Enter,
                    modifiers: KeyModifiers::NONE,
                }) => return Some(Msg::FormSubmit),
                Event::Keyboard(KeyEvent {
                    code: Key::Char(ch),
                    modifiers: KeyModifiers::NONE,
                }) => self.perform(Cmd::Type(ch)),
                Event::Keyboard(KeyEvent {
                    code: Key::Left,
                    modifiers: KeyModifiers::NONE,
                }) => self.perform(Cmd::Move(Direction::Left)),
                Event::Keyboard(KeyEvent {
                    code: Key::Right,
                    modifiers: KeyModifiers::NONE,
                }) => self.perform(Cmd::Move(Direction::Right)),
                Event::Keyboard(KeyEvent {
                    code: Key::Home,
                    modifiers: KeyModifiers::NONE,
                }) => self.perform(Cmd::GoTo(Position::Begin)),
                Event::Keyboard(KeyEvent {
                    code: Key::End,
                    modifiers: KeyModifiers::NONE,
                }) => self.perform(Cmd::GoTo(Position::End)),
                Event::Keyboard(KeyEvent {
                    code: Key::Delete,
                    modifiers: KeyModifiers::NONE,
                }) => self.perform(Cmd::Cancel),
                Event::Keyboard(KeyEvent {
                    code: Key::Backspace,
                    modifiers: KeyModifiers::NONE,
                }) => self.perform(Cmd::Delete),
                Event::Keyboard(KeyEvent {
                    code: Key::Tab,
                    modifiers: KeyModifiers::NONE,
                }) => return Some(Msg::AddressInputBlur),
                _ => return None,
            };
            Some(Msg::None)
        }
    }
    ```

7. Implement the update routine for your model

    Implement the `Update` trait for `Model` matching all your `Msg` and performing what you need to do.

    ```rust
    impl Update<Id, Msg, NoUserEvent> for Model {
        fn update(&mut self, view: &mut View<Id, Msg, NoUserEvent>, msg: Option<Msg>) -> Option<Msg> {
            match msg.unwrap_or(Msg::None) {
                Msg::AppClose => {
                    self.quit = true;
                    None
                }
                // ... 
                Msg::None => None,
            }
        }
    }
    ```

8. Update your previous `on()` call:

    ```rust
    if let Ok(sz) = app.tick(&mut model, PollStrategy::Once) {
        if sz > 0 {
            // NOTE: redraw if at least one msg has been processed
            model.redraw = true;
        }
    }
    // Redraw
    if model.redraw {
        // View must be implemented by yourself!
        model.view(&mut app);
        model.redraw = false;
    }
    ```
