# Get Started 🏁

- [Get Started 🏁](#get-started-)
  - [Implement an input handler](#implement-an-input-handler)
  - [Setup terminal](#setup-terminal)
  - [Let's create the application](#lets-create-the-application)
  - [What's next](#whats-next)

---

## Implement an input handler

If you're using tui-realm, I assume you need to catch input events (if you don't, use tui then...).
To do this I will make your life easier showing you how to implement an input handler very quickly:

>> input.rs

```rust
extern crate crossterm;

use crossterm::event::{poll, read, Event};
use std::time::Duration;

pub struct InputHandler;

impl InputHandler {

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

## Setup terminal

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

## Let's create the application

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
    use tuirealm::{InputType, Msg, Payload, PropsBuilder, Update, Value, View};
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
        view: View,
    }
    
    impl Model {
        fn new(view: View) -> Self {
            Model {
                quit: false,
                redraw: true,
                last_redraw: Instant::now(),
                view,
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
    let mut model: Model = Model::new(myview);
    ```

7. Implement the `view()` function

    The view function will render our `View` into the Frame

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

8. Implement the `update()` trait

    The update function will update the `Model` based on the `Msg` reported by the `View`. The update function has usually the signature we're going to use and I strongly suggest you to use it for two reasons:

    1. It looks like the one used by Elm
    2. It allows recursive update

    ```rust
    impl Update for Model {
        fn update(&mut self, msg: Option<(String, Msg)>) -> Option<(String, Msg)> {
            let ref_msg: Option<(&str, &Msg)> = msg.as_ref().map(|(s, msg)| (s.as_str(), msg));
            match ref_msg {
                None => None, // Exit after None
                Some(msg) => match msg {
                    (COMPONENT_INPUT, Msg::OnChange(Payload::One(Value::Str(input)))) => {
                        // Update span
                        let props = label::LabelPropsBuilder::from(
                            self.view.get_props(COMPONENT_LABEL).unwrap(),
                        )
                        .with_text(format!("You typed: '{}'", input))
                        .build();
                        // Report submit
                        let msg = self.view.update(COMPONENT_LABEL, props);
                        self.update(msg)
                    }
                    (_, &MSG_KEY_ESC) => {
                        // Quit on esc
                        self.quit();
                        None
                    }
                    _ => None,
                },
            }
        }
    }
    ```

    You don't have to use the `Update` trait. If you want you can implement an update function by yourself.

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

## What's next

You now might be interested in read more about the tui-realm elements and how it works. If so, go read the [Tui-realm application lifecycle](lifecycle.md).
