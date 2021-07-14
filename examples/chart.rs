//! ## Demo
//!
//! `Demo` shows how to use tui-realm in a real case

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
extern crate rand;

mod utils;

use utils::context::Context;
use utils::keymap::*;

use rand::{rngs::ThreadRng, thread_rng, Rng};
use std::thread::sleep;
use std::time::{Duration, Instant};

use tuirealm::components::{
    chart::{Chart, ChartPropsBuilder},
    label,
};
use tuirealm::props::{
    borders::{BorderType, Borders},
    Dataset,
};
use tuirealm::{Msg, PropsBuilder, Update, View};
// tui
use tui::layout::{Constraint, Direction, Layout};
use tui::style::{Color, Style};
use tui::symbols::Marker;
use tui::widgets::GraphType;

const COMPONENT_CHART1: &str = "chart1";
const COMPONENT_EVENT: &str = "LABEL";

struct Model {
    quit: bool,               // Becomes true when the user presses <ESC>
    redraw: bool,             // Tells whether to refresh the UI; performance optimization
    last_redraw: Instant,     // Last time the ui has been redrawed
    last_data_fetch: Instant, // Last time data was added
    start: Instant,
    view: View,
    boundaries: (f64, f64),
}

impl Model {
    fn new(view: View) -> Self {
        Model {
            quit: false,
            redraw: true,
            last_redraw: Instant::now(),
            last_data_fetch: Instant::now(),
            start: Instant::now(),
            view,
            boundaries: (0.0, 50.0),
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

    fn should_fetch_data(&self) -> bool {
        self.last_data_fetch.elapsed() >= Duration::from_millis(200)
    }

    fn data_fetched(&mut self) {
        self.last_data_fetch = Instant::now();
        self.boundaries = ((self.boundaries.0 + 1.0), (self.boundaries.1 + 1.0));
    }

    fn elapsed(&self) -> u64 {
        ((self.start.elapsed().as_millis() + 1024) * 5) as u64
    }
}

fn main() {
    // let's create a context: the context contains the backend of crossterm and the input handler
    let mut ctx: Context = Context::new();
    // Enter alternate screen
    ctx.enter_alternate_screen();
    // Clear screen
    ctx.clear_screen();
    // let's create a `View`, which will contain the components
    let mut myview: View = View::init();
    // Init data
    let mut rand: ThreadRng = thread_rng();
    let max_val: u64 = 450;
    let data: Vec<(f64, f64)> = (0..1024)
        .map(|i| (i as f64, get_rand(&mut rand, max_val)))
        .collect();
    let data_avg: Vec<(f64, f64)> = (0..1024)
        .map(|i| (i as f64, get_rand(&mut rand, max_val)))
        .collect();
    myview.mount(
        COMPONENT_CHART1,
        Box::new(Chart::new(
            ChartPropsBuilder::default()
                .visible()
                .disabled(true)
                .with_background(Color::Reset)
                .with_foreground(Color::Reset)
                .with_title(String::from("Temperatures in room"))
                .with_borders(Borders::ALL, BorderType::Double, Color::Yellow)
                .with_x_style(Style::default().fg(Color::LightBlue))
                .with_x_title("Time")
                .with_x_bounds((0.0, 50.0))
                .with_x_labels(&["1Y", "10M", "8M", "6M", "4M", "2M", "now"])
                .with_y_bounds((0.0, 50.0))
                .with_y_labels(&[
                    "0", "5", "10", "15", "20", "25", "30", "35", "40", "45", "50",
                ])
                .with_y_style(Style::default().fg(Color::Yellow))
                .with_y_title("Temperature (°C)")
                .with_data(&[
                    Dataset::default()
                        .name("Temperatures")
                        .graph_type(GraphType::Line)
                        .marker(Marker::Braille)
                        .style(Style::default().fg(Color::Cyan))
                        .data(data),
                    Dataset::default()
                        .name("Avg temperatures")
                        .graph_type(GraphType::Line)
                        .marker(Marker::Braille)
                        .style(Style::default().fg(Color::Red))
                        .data(data_avg),
                ])
                .build(),
        )),
    );
    myview.mount(
        COMPONENT_EVENT,
        Box::new(label::Label::new(
            label::LabelPropsBuilder::default()
                .with_foreground(Color::Cyan)
                .with_text(String::from("Event will appear here"))
                .build(),
        )),
    );
    // We need to give focus to input then
    myview.active(COMPONENT_CHART1);
    // Now we use the Model struct to keep track of some states
    let mut model: Model = Model::new(myview);
    // let's loop until quit is true
    while !model.quit {
        // Listen for input events
        if let Ok(Some(ev)) = ctx.input_hnd.read_event() {
            // Pass event to view
            let msg = model.view.on(ev);
            model.redraw();
            // Call the elm friend update
            model.update(msg);
        }
        // Fetch data
        if model.should_fetch_data() {
            model.data_fetched();
            model.view.update(
                COMPONENT_CHART1,
                ChartPropsBuilder::from(model.view.get_props(COMPONENT_CHART1).unwrap())
                    .pop_record_front(0)
                    .push_record(0, (model.elapsed() as f64, get_rand(&mut rand, max_val)))
                    .pop_record_front(1)
                    .push_record(1, (model.elapsed() as f64, get_rand(&mut rand, max_val)))
                    .with_x_bounds(model.boundaries)
                    .build(),
            );
            model.redraw();
        }
        // If redraw, draw interface
        if model.redraw || model.last_redraw.elapsed() > Duration::from_millis(50) {
            // Call the elm friend vie1 function
            view(&mut ctx, &model.view);
            model.reset();
        }
        sleep(Duration::from_millis(10));
    }
    // Let's drop the context finally
    drop(ctx);
}

fn view(ctx: &mut Context, view: &View) {
    let _ = ctx.terminal.draw(|f| {
        // Prepare chunks
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([Constraint::Length(15), Constraint::Length(3)].as_ref())
            .split(f.size());
        view.render(COMPONENT_CHART1, f, chunks[0]);
        view.render(COMPONENT_EVENT, f, chunks[1]);
    });
}

impl Update for Model {
    fn update(&mut self, msg: Option<(String, Msg)>) -> Option<(String, Msg)> {
        let ref_msg: Option<(&str, &Msg)> = msg.as_ref().map(|(s, msg)| (s.as_str(), msg));
        match ref_msg {
            None => None, // Exit after None
            Some(msg) => match msg {
                (_, &MSG_KEY_ESC) => {
                    // Quit on esc
                    self.quit();
                    None
                }
                (component, event) => {
                    // Update span
                    let props = label::LabelPropsBuilder::from(
                        self.view.get_props(COMPONENT_EVENT).unwrap(),
                    )
                    .with_text(format!("{} => '{:?}'", component, event))
                    .build();
                    // Report submit
                    let _ = self.view.update(COMPONENT_EVENT, props);
                    None
                }
            },
        }
    }
}

fn get_rand(rng: &mut ThreadRng, max: u64) -> f64 {
    rng.gen_range(0..max) as f64 / 10.0
}
