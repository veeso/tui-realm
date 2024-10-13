//! ## Canvas
//!
//! A canvas where you can draw more complex figures

use tuirealm::command::{Cmd, CmdResult};
use tuirealm::props::{
    Alignment, AttrValue, Attribute, Borders, Color, PropPayload, PropValue, Props, Shape, Style,
};
use tuirealm::ratatui::symbols::Marker;
use tuirealm::ratatui::text::Line as Spans;
use tuirealm::ratatui::{
    layout::Rect,
    text::Span,
    widgets::canvas::{Canvas as TuiCanvas, Context, Points},
};
use tuirealm::{Frame, MockComponent, State};

// -- Props
use super::props::{
    CANVAS_MARKER, CANVAS_MARKER_BLOCK, CANVAS_MARKER_BRAILLE, CANVAS_MARKER_DOT, CANVAS_X_BOUNDS,
    CANVAS_Y_BOUNDS,
};

// -- Component

/// ## Canvas
///
/// The Canvas widget may be used to draw more detailed figures using braille patterns (each cell can have a braille character in 8 different positions).
#[derive(Default)]
pub struct Canvas {
    props: Props,
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

    pub fn title<S: Into<String>>(mut self, t: S, a: Alignment) -> Self {
        self.attr(Attribute::Title, AttrValue::Title((t.into(), a)));
        self
    }

    pub fn data(mut self, data: &[Shape]) -> Self {
        self.attr(
            Attribute::Shape,
            AttrValue::Payload(PropPayload::Vec(
                data.iter().map(|x| PropValue::Shape(x.clone())).collect(),
            )),
        );
        self
    }

    /// From <https://github.com/fdehau/tui-rs/issues/286>:
    ///
    /// > Those are used to define the viewport of the canvas.
    /// > Only the points whose coordinates are within the viewport are displayed.
    /// > When you render the canvas using Frame::render_widget, you give an area to draw the widget to (a Rect) and
    /// > the crate translates the floating point coordinates to those used by our internal terminal representation.
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

    /// From <https://github.com/fdehau/tui-rs/issues/286>:
    ///
    /// > Those are used to define the viewport of the canvas.
    /// > Only the points whose coordinates are within the viewport are displayed.
    /// > When you render the canvas using Frame::render_widget, you give an area to draw the widget to (a Rect) and
    /// > the crate translates the floating point coordinates to those used by our internal terminal representation.
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

    /// Set marker to use to draw on canvas
    pub fn marker(mut self, marker: Marker) -> Self {
        self.attr(
            Attribute::Custom(CANVAS_MARKER),
            Self::marker_to_prop(marker),
        );
        self
    }

    fn marker_to_prop(marker: Marker) -> AttrValue {
        AttrValue::Number(match marker {
            Marker::HalfBlock => crate::props::CANVAS_MARKER_HALF_BLOCK,
            Marker::Bar => crate::props::CANVAS_MARKER_BAR,
            Marker::Block => CANVAS_MARKER_BLOCK,
            Marker::Braille => CANVAS_MARKER_BRAILLE,
            Marker::Dot => CANVAS_MARKER_DOT,
        })
    }

    fn prop_to_marker(&self) -> Marker {
        match self
            .props
            .get_or(
                Attribute::Custom(CANVAS_MARKER),
                AttrValue::Number(CANVAS_MARKER_BRAILLE),
            )
            .unwrap_number()
        {
            CANVAS_MARKER_BLOCK => Marker::Block,
            CANVAS_MARKER_DOT => Marker::Dot,
            _ => Marker::Braille,
        }
    }

    /// Draw a shape into the canvas `Context`
    fn draw_shape(ctx: &mut Context, shape: &Shape) {
        match shape {
            Shape::Label((x, y, label, color)) => {
                let span = Span::styled(label.to_string(), Style::default().fg(*color));
                ctx.print(*x, *y, Spans::from(vec![span]));
            }
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
            block = block.style(Style::default().bg(background).fg(foreground));
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
                        .cloned()
                        .map(|x| x.unwrap_shape())
                        .collect()
                })
                .unwrap_or_default();
            // Make canvas
            let canvas = TuiCanvas::default()
                .background_color(background)
                .block(block)
                .marker(self.prop_to_marker())
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
    use tuirealm::ratatui::widgets::canvas::{Line, Map, MapResolution, Rectangle};

    #[test]
    fn test_component_canvas_with_shapes() {
        let component: Canvas = Canvas::default()
            .background(Color::Black)
            .title("playing risiko", Alignment::Center)
            .borders(Borders::default())
            .marker(Marker::Dot)
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
