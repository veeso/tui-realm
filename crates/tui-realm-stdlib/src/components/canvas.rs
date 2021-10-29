//! ## Canvas
//!
//! A canvas where you can draw more complex figures

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
use tuirealm::command::{Cmd, CmdResult};
use tuirealm::props::{
    Alignment, AttrValue, Attribute, Borders, Color, PropPayload, PropValue, Props, Shape, Style,
};
use tuirealm::tui::{
    layout::Rect,
    widgets::canvas::{Canvas as TuiCanvas, Context, Points},
};
use tuirealm::{Frame, MockComponent, State};

// -- Props
use super::props::{CANVAS_X_BOUNDS, CANVAS_Y_BOUNDS};

// -- Component

/// ## Canvas
///
/// The Canvas widget may be used to draw more detailed figures using braille patterns (each cell can have a braille character in 8 different positions).
pub struct Canvas {
    props: Props,
}

impl Default for Canvas {
    fn default() -> Self {
        Self {
            props: Props::default(),
        }
    }
}

impl Canvas {
    pub fn foreground(mut self, fg: Color) -> Self {
        self.attr(Attribute::Foreground, AttrValue::Color(fg));
        self
    }

    pub fn background(mut self, bg: Color) -> Self {
        self.attr(Attribute::Background, AttrValue::Color(bg));
        self
    }

    pub fn borders(mut self, b: Borders) -> Self {
        self.attr(Attribute::Borders, AttrValue::Borders(b));
        self
    }

    pub fn title<S: AsRef<str>>(mut self, t: S, a: Alignment) -> Self {
        self.attr(
            Attribute::Title,
            AttrValue::Title((t.as_ref().to_string(), a)),
        );
        self
    }

    pub fn data(mut self, data: &[Shape]) -> Self {
        self.attr(
            Attribute::Shape,
            AttrValue::Payload(PropPayload::Vec(
                data.into_iter().map(|x| PropValue::Shape(*x)).collect(),
            )),
        );
        self
    }

    pub fn x_bounds(mut self, bounds: (f64, f64)) -> Self {
        self.attr(
            Attribute::Custom(CANVAS_X_BOUNDS),
            AttrValue::Payload(PropPayload::Tup2((
                PropValue::F64(bounds.0),
                PropValue::F64(bounds.1),
            ))),
        );
        self
    }

    pub fn y_bounds(mut self, bounds: (f64, f64)) -> Self {
        self.attr(
            Attribute::Custom(CANVAS_Y_BOUNDS),
            AttrValue::Payload(PropPayload::Tup2((
                PropValue::F64(bounds.0),
                PropValue::F64(bounds.1),
            ))),
        );
        self
    }

    /// ### draw_shape
    ///
    /// Draw a shape into the canvas `Context`
    fn draw_shape(ctx: &mut Context, shape: &Shape) {
        match shape {
            Shape::Label((x, y, s, c)) => {} /* ctx.print(*x, *y, &s, *c) FIXME: UNSUPPORTED */
            Shape::Layer => ctx.layer(),
            Shape::Line(line) => ctx.draw(line),
            Shape::Map(map) => ctx.draw(map),
            Shape::Points((coords, color)) => ctx.draw(&Points {
                coords,
                color: *color,
            }),
            Shape::Rectangle(rectangle) => ctx.draw(rectangle),
        }
    }
}

impl MockComponent for Canvas {
    fn view(&mut self, render: &mut Frame, area: Rect) {
        if self.props.get_or(Attribute::Display, AttrValue::Flag(true)) == AttrValue::Flag(true) {
            let foreground = self
                .props
                .get_or(Attribute::Foreground, AttrValue::Color(Color::Reset))
                .unwrap_color();
            let background = self
                .props
                .get_or(Attribute::Background, AttrValue::Color(Color::Reset))
                .unwrap_color();
            let borders = self
                .props
                .get_or(Attribute::Borders, AttrValue::Borders(Borders::default()))
                .unwrap_borders();
            let title = self.props.get(Attribute::Title).map(|x| x.unwrap_title());
            let focus = self
                .props
                .get_or(Attribute::Focus, AttrValue::Flag(false))
                .unwrap_flag();
            let mut block = crate::utils::get_block(borders, title, focus, None);
            block = block.style(Style::default().bg(background));
            // Get properties
            let x_bounds: [f64; 2] = self
                .props
                .get(Attribute::Custom(CANVAS_X_BOUNDS))
                .map(|x| x.unwrap_payload().unwrap_tup2())
                .map(|(a, b)| [a.unwrap_f64(), b.unwrap_f64()])
                .unwrap_or([0.0, 0.0]);
            let y_bounds: [f64; 2] = self
                .props
                .get(Attribute::Custom(CANVAS_X_BOUNDS))
                .map(|x| x.unwrap_payload().unwrap_tup2())
                .map(|(a, b)| [a.unwrap_f64(), b.unwrap_f64()])
                .unwrap_or([0.0, 0.0]);
            // Get shapes
            let shapes: Vec<Shape> = self
                .props
                .get(Attribute::Shape)
                .map(|x| {
                    x.unwrap_payload()
                        .unwrap_vec()
                        .iter()
                        .map(|x| x.unwrap_shape())
                        .collect()
                })
                .unwrap_or_default();
            // Make canvas
            let canvas = TuiCanvas::default()
                .background_color(background)
                .block(block)
                .x_bounds(x_bounds)
                .y_bounds(y_bounds)
                .paint(|ctx| shapes.iter().for_each(|x| Self::draw_shape(ctx, x)));
            // Render
            render.render_widget(canvas, area);
        }
    }

    fn query(&self, attr: Attribute) -> Option<AttrValue> {
        self.props.get(attr)
    }

    fn attr(&mut self, attr: Attribute, value: AttrValue) {
        self.props.set(attr, value)
    }

    fn state(&self) -> State {
        State::None
    }

    fn perform(&mut self, _cmd: Cmd) -> CmdResult {
        CmdResult::None
    }
}

#[cfg(test)]
mod test {

    use super::*;

    use pretty_assertions::assert_eq;
    use tuirealm::tui::widgets::canvas::{Line, Map, MapResolution, Rectangle};

    #[test]
    fn test_component_canvas_with_shapes() {
        let component: Canvas = Canvas::default()
            .background(Color::Black)
            .title("playing risiko", Alignment::Center)
            .borders(Borders::default())
            .x_bounds((-180.0, 180.0))
            .y_bounds((-90.0, 90.0))
            .data(&[
                Shape::Map(Map {
                    resolution: MapResolution::High,
                    color: Color::Rgb(240, 240, 240),
                }),
                Shape::Layer,
                Shape::Line(Line {
                    x1: 0.0,
                    y1: 10.0,
                    x2: 10.0,
                    y2: 10.0,
                    color: Color::Red,
                }),
                Shape::Rectangle(Rectangle {
                    x: 60.0,
                    y: 20.0,
                    width: 70.0,
                    height: 22.0,
                    color: Color::Cyan,
                }),
                Shape::Points((
                    vec![
                        (21.0, 13.0),
                        (66.0, 77.0),
                        (34.0, 69.0),
                        (45.0, 76.0),
                        (120.0, 55.0),
                        (-32.0, -50.0),
                        (-4.0, 2.0),
                        (-32.0, -48.0),
                    ],
                    Color::Green,
                )),
            ]);
        assert_eq!(component.state(), State::None);
    }
}
