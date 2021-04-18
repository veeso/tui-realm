# tui-realm

[![License: MIT](https://img.shields.io/badge/License-MIT-teal.svg)](https://opensource.org/licenses/MIT) [![Stars](https://img.shields.io/github/stars/veeso/tui-realm.svg)](https://github.com/veeso/tui-realm) [![Downloads](https://img.shields.io/crates/d/tui-realm.svg)](https://crates.io/crates/tui-realm) [![Crates.io](https://img.shields.io/badge/crates.io-v0.1.0-orange.svg)](https://crates.io/crates/tui-realm) [![Docs](https://docs.rs/tui-realm/badge.svg)](https://docs.rs/tui-realm)  

[![Build](https://github.com/veeso/tui-realm/workflows/Linux/badge.svg)](https://github.com/veeso/tui-realm/actions) [![Build](https://github.com/veeso/tui-realm/workflows/MacOS/badge.svg)](https://github.com/veeso/tui-realm/actions) [![Build](https://github.com/veeso/tui-realm/workflows/Windows/badge.svg)](https://github.com/veeso/tui-realm/actions) [![codecov](https://codecov.io/gh/veeso/tui-realm/branch/main/graph/badge.svg?token=au67l7nQah)](https://codecov.io/gh/veeso/tui-realm)

Developed by Christian Visintin  
Current version: 0.1.0 (18/04/2021)

---

- [tui-realm](#tui-realm)
  - [About tui-realm 👑](#about-tui-realm-)
    - [Why tui-realm 🤔](#why-tui-realm-)
  - [Get started 🏁](#get-started-)
    - [Add tui-realm to your Cargo.toml 🦀](#add-tui-realm-to-your-cargotoml-)
    - [Implement an input handler](#implement-an-input-handler)
    - [Setup terminal](#setup-terminal)
    - [Let's create the application](#lets-create-the-application)
    - [Run examples](#run-examples)
  - [Standard component library 🎨](#standard-component-library-)
  - [Documentation 📚](#documentation-)
  - [About other backends](#about-other-backends)
  - [Contributing and issues 🤝🏻](#contributing-and-issues-)
  - [Changelog ⏳](#changelog-)
  - [Buy me a coffee ☕](#buy-me-a-coffee-)
  - [License 📃](#license-)

---

## About tui-realm 👑

tui-realm is a **framework** for [tui](https://github.com/fdehau/tui-rs) which provides a layer to simplify the implementation of terminal user interfaces adding the possibility to work with re-usable component with properties and state, as you'd do in React; but that's not all: the input events are handled through a system based on **Messages**, providing you with the possibility to implement `update` functions as happens in Elm.

And that's also explains the reason of the name: Realm stands for React and Elm.

Tui-realm also comes with a built-in standard library of components you may find very useful. Don't worry, they are optional if you don't want them 😉, just follow the guide in [get started](#get-started-).

### Why tui-realm 🤔

Personally I didn't start this project from scratch. I've just decided to make a library out of the already existing code in [termscp](https://github.com/veeso/termscp), which I had just finished at the time I started this project. I thought this library could have come handy for somebody.

You might be wondering now how much is this project influenced by the development of termscp. Well, a lot actually, I won't deny this, so don't expect this library to always try to fit the community needs, I'm just providing you with a tool I've made for myself, but that I wanted to share with the community.

---

## Get started 🏁

⚠ Warning: tui-realm works only with **crossterm** as backend ⚠

### Add tui-realm to your Cargo.toml 🦀

```toml
tuirealm = "0.1.0"
```

or if you want to include the [standard component library](#standard-component-library-)...

```toml
tuirealm = { "version" = "0.1.0", features = [ "with-components" ] }
```

Since this library requires `crossterm` too, you'll also need to add it to your Cargo.toml

```toml
crossterm = "0.19.0"
```

### Implement an input handler

If you're using tui-realm, I assume you need to catch input events (if you don't, use tui then...).
To do this I will make your life easier showing you how to implement an input handler very quickly:

>> input.rs

```rust
extern crate crossterm;

use crossterm::event::{poll, read, Event};
use std::time::Duration;

pub struct InputHandler;

impl InputHandler {
    /// ### InputHandler
    ///
    ///
    pub fn new() -> InputHandler {
        InputHandler {}
    }

    pub fn read_event(&self) -> Result<Option<Event>, ()> {
        if let Ok(available) = poll(Duration::from_millis(10)) {
            match available {
                true => {
                    // Read event
                    if let Ok(ev) = read() {
                        Ok(Some(ev))
                    } else {
                        Err(())
                    }
                }
                false => Ok(None),
            }
        } else {
            Err(())
        }
    }
}
```

We can use `read_event()` to read input events now.

### Setup terminal

We obviously also need to setup the terminal for crossterm. This is quite simple, but I'll use an helper here to make it even easier.

>> terminal.rs

```rust
extern crate crossterm;
extern crate tui;

use crossterm::event::DisableMouseCapture;
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use std::io::{stdout, Stdout};
use tui::backend::CrosstermBackend;
use tui::Terminal as TuiTerminal;

pub struct Terminal {
    pub terminal: Terminal<CrosstermBackend<Stdout>>,
}

impl Terminal {

    pub fn new() -> Self {
        let _ = enable_raw_mode();
        // Create terminal
        let mut stdout = stdout();
        assert!(execute!(stdout, EnterAlternateScreen).is_ok());
        Self {
            terminal: Terminal::new(CrosstermBackend::new(stdout)).unwrap(),
        }
    }

    pub fn leave_alternate_screen(&mut self) {
        let _ = execute!(
            self.terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        );
    }

    pub fn clear_screen(&mut self) {
        let _ = self.terminal.clear();
    }

}

impl Drop for Context {
    fn drop(&mut self) {
        // Re-enable terminal stuff
        self.leave_alternate_screen();
        let _ = disable_raw_mode();
    }
}

```

### Let's create the application

I will implement everything in a single file in this case and I will reproduce step-by-step the demo example:

>> main.rs

1. Let's import all we need

    ```rust
    extern crate tuirealm;
    extern crate tui;

    use std::thread::sleep;
    use std::time::{Duration, Instant};

    use tuirealm::components::{input, label};
    use tuirealm::props::borders::{BorderType, Borders};
    use tuirealm::{InputType, Msg, Payload, PropsBuilder, View};
    // tui
    use tui::layout::{Constraint, Direction, Layout};
    use tui::style::Color;
    ```

2. Let's declare as const the component names:

    ```rust
    const COMPONENT_INPUT: &str = "INPUT";
    const COMPONENT_LABEL: &str = "LABEL";
    ```

3. Let's declare our `Model`. The model as in Elm contains the states of the application. This name is completely up to you. Let's say Model is a convention.

    ```rust
    struct Model {
        quit: bool,           // Becomes true when the user presses <ESC>
        redraw: bool,         // Tells whether to refresh the UI; performance optimization
        last_redraw: Instant, // Last time the ui has been redrawed
    }
    
    impl Model {
        fn new() -> Self {
            Model {
                quit: false,
                redraw: true,
                last_redraw: Instant::now(),
            }
        }
    
        fn quit(&mut self) {
            self.quit = true;
        }
    
        fn redraw(&mut self) {
            self.redraw = true;
        }
    
        fn reset(&mut self) {
            self.redraw = false;
            self.last_redraw = Instant::now();
        }
    }
    ```

4. Setup terminal

    ```rust
    fn main() {
        let mut terminal: Terminal = Terminal::new();
        let input: InputHandler = InputHandler::new();
        // Enter alternate screen
        terminal.enter_alternate_screen();
        // Clear screen
        terminal.clear_screen();
    ```

5. Setup `View`

    Remember you can have the amount of views you want, but in this example we're going to use only one.

    ```rust
    let mut myview: View = View::init();
    // Let's mount the component we need
    myview.mount(
        COMPONENT_INPUT,
        Box::new(input::Input::new(
            input::InputPropsBuilder::default()
                .with_borders(Borders::ALL, BorderType::Rounded, Color::LightYellow)
                .with_foreground(Color::LightYellow)
                .with_input(InputType::Text)
                .with_label(String::from("Type in something nice"))
                .build(),
        )),
    );
    myview.mount(
        COMPONENT_LABEL,
        Box::new(label::Label::new(
            label::LabelPropsBuilder::default()
                .with_foreground(Color::Cyan)
                .with_text(String::from("Your input will appear in after a submit"))
                .build(),
        )),
    );
    // We need to initialize the focus
    myview.active(COMPONENT_INPUT);
    ```

6. Setup `Model`

    ```rust
    let mut model: Model = Model::new();
    ```

7. Implement the `view()` function

    The view function will render our `View` into the Canvas

    ```rust
    fn view(t: &mut Terminal, view: &View) {
        let _ = t.terminal.draw(|f| {
            // Prepare chunks
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([Constraint::Length(3), Constraint::Length(3)]. as_ref())
                .split(f.size());
            view.render(COMPONENT_INPUT, f, chunks[0]);
            view.render(COMPONENT_LABEL, f, chunks[1]);
        });
    }
    ```

8. Implement the `update()` function

    The update function will update the `Model` based on the `Msg` reported by the `View`. The update function has usually the signature we're going to use and I strongly suggest you to use it for two reasons:

    1. It looks like the one used by Elm
    2. It allows recursive update

    ```rust
    fn update(model: &mut Model, view: &mut View, msg: Option<(String, Msg)>) -> Option<(String, Msg)> {
        let ref_msg: Option<(&str, &Msg)> = msg.as_ref().map(|(s, msg)| (s.as_str   (), msg));
        match ref_msg {
            None => None, // Exit after None
            Some(msg) => match msg {
                (COMPONENT_INPUT, Msg::OnSubmit(Payload::Text(input))) => {
                    // Update span
                    let props =
                        label::LabelPropsBuilder::from(view.get_props   (COMPONENT_LABEL).unwrap())
                            .with_text(format!("You typed: '{}'", input))
                            .build();
                    // Update label; then call update recursively
                    let msg = view.update(COMPONENT_LABEL, props)
                    update(model, view, msg)
                }
                (_, &MSG_KEY_ESC) => {
                    // Quit on esc
                    model.quit();
                    None
                }
                _ => None,
            },
        }
    }
    ```

9. Run the GUI

    Let's return in our `main()` function to implement the main loop for our application:

    ```rust
    while !model.quit {
        // Listen for input events
        if let Ok(Some(ev)) = ctx.input_hnd.read_event() {
            // Pass event to view
            let msg = myview.on(ev);
            model.redraw();
            // Call the elm friend update
            update(&mut model, &mut myview, msg);
        }
        // If redraw, draw interface
        if model.redraw || model.last_redraw.elapsed() > Duration::from_millis(50) {
            // Call the elm friend vie1 function
            view(&mut ctx, &myview);
            model.reset();
        }
        sleep(Duration::from_millis(10));
    }
    ```

10. Finalize the terminal

    This is very important, otherwise the terminal won't work as expected once the application is terminated:

    ```rust
        drop(terminal);
    }
    ```

### Run examples

Still confused about how tui-realm works? Don't worry, try with the examples:

- [demo](examples/demo.rs): a simple application which shows how tui-realm works

    ```sh
    cargo run --features="with-components" --example demo
    ```

- [termscp](https://github.com/veeso/termscp): real production implemenetation of tui-realm; just browse the `src/ui/` folder.

---

## Standard component library 🎨

Tui-realm comes with an optional standard library of components I thought may be useful for most of the applications.
If you want to use it, just enabled the `with-components` feature in your `Cargo.toml`.

TODO: complete

To have an overview of the components just run the gallery example 🦄

```sh
cargo run --features="with-components" --example gallery
```

---

## Documentation 📚

The developer documentation can be found on Rust Docs at <https://docs.rs/tui-realm>

---

## About other backends

TODO: fill

---

## Contributing and issues 🤝🏻

Contributions, bug reports, new features and questions are welcome! 😉
If you have any question or concern, or you want to suggest a new feature, or you want just want to improve tui-realm, feel free to open an issue or a PR.

Please follow [our contributing guidelines](CONTRIBUTING.md) TODO: write contributing

---

## Changelog ⏳

View tui-realm's changelog [HERE](CHANGELOG.md)

---

## Buy me a coffee ☕

If you like tui-realm and you're grateful for the work I've done, please consider a little donation 🥳

[![Buy-me-a-coffee](https://img.buymeacoffee.com/button-api/?text=Buy%20me%20a%20coffee&emoji=&slug=veeso&button_colour=404040&font_colour=ffffff&font_family=Comic&outline_colour=ffffff&coffee_colour=FFDD00)](https://www.buymeacoffee.com/veeso)

---

## License 📃

tui-realm is licensed under the MIT license.

You can read the entire license [HERE](LICENSE)
