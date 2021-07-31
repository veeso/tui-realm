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
use crate::props::{BordersProps, PropPayload, PropValue, Props, PropsBuilder, Shape};
use crate::tui::{
    layout::Rect,
    style::{Color, Style},
    widgets::canvas::{Canvas as TuiCanvas, Context, Line, Map, MapResolution, Points, Rectangle},
    widgets::{Block, BorderType, Borders},
};
use crate::{Component, Event, Frame, Msg, Payload};

// -- Props

const PROP_X_BOUNDS: &str = "x-bounds";
const PROP_Y_BOUNDS: &str = "y-bounds";
const PROP_SHAPES: &str = "shapes";
const PROP_TITLE: &str = "title";

pub struct CanvasPropsBuilder {
    props: Option<Props>,
}

impl Default for CanvasPropsBuilder {
    fn default() -> Self {
        Self {
            props: Some(Props::default()),
        }
    }
}

impl PropsBuilder for CanvasPropsBuilder {
    fn build(&mut self) -> Props {
        self.props.take().unwrap()
    }

    fn hidden(&mut self) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            props.visible = false;
        }
        self
    }

    fn visible(&mut self) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            props.visible = true;
        }
        self
    }
}

impl From<Props> for CanvasPropsBuilder {
    fn from(props: Props) -> Self {
        Self { props: Some(props) }
    }
}

impl CanvasPropsBuilder {
    /// ### with_background
    ///
    /// Set background color for component
    pub fn with_background(&mut self, color: Color) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            props.background = color;
        }
        self
    }

    /// ### with_borders
    ///
    /// Set component borders style
    pub fn with_borders(
        &mut self,
        borders: Borders,
        variant: BorderType,
        color: Color,
    ) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            props.borders = BordersProps {
                borders,
                variant,
                color,
            }
        }
        self
    }

    /// ### with_title
    ///
    /// Set title
    pub fn with_title<S: AsRef<str>>(&mut self, title: S) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            props.own.insert(
                PROP_TITLE,
                PropPayload::One(PropValue::Str(title.as_ref().to_string())),
            );
        }
        self
    }

    /// ### with_x_bounds
    ///
    /// Define x axis bounds
    pub fn with_x_bounds(&mut self, (floor, ceil): (f64, f64)) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            props.own.insert(
                PROP_X_BOUNDS,
                PropPayload::Tup2((PropValue::F64(floor), PropValue::F64(ceil))),
            );
        }
        self
    }

    /// ### with_y_bounds
    ///
    /// Define y axis bounds
    pub fn with_y_bounds(&mut self, (floor, ceil): (f64, f64)) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            props.own.insert(
                PROP_Y_BOUNDS,
                PropPayload::Tup2((PropValue::F64(floor), PropValue::F64(ceil))),
            );
        }
        self
    }

    /// ### with_shapes
    ///
    /// Sets (and eventually replace) shapes
    pub fn with_shapes(&mut self, paint: Vec<Shape>) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            props.own.insert(
                PROP_SHAPES,
                PropPayload::Vec(paint.into_iter().map(PropValue::Shape).collect()),
            );
        }
        self
    }

    /// ### with_new_canvas
    ///
    /// Initialize a new canvas data
    pub fn with_new_drawing(&mut self) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            props.own.insert(PROP_SHAPES, PropPayload::Vec(Vec::new()));
        }
        self
    }

    /*
    /// ### with_label
    ///
    /// Push a new label to canvas
    pub fn with_label<S: AsRef<str>>(&mut self, s: S, x: f64, y: f64, color: Color) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            Self::push_paint_data(props, Shape::Label((x, y, s.as_ref().to_string(), color)));
        }
        self
    }
    */

    /// ### with_layer
    ///
    /// Add a new layer to canvas
    pub fn with_layer(&mut self) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            Self::push_paint_data(props, Shape::Layer);
        }
        self
    }

    /// ### with_line
    ///
    /// Add a new line to canvas
    pub fn with_line(&mut self, x1: f64, y1: f64, x2: f64, y2: f64, color: Color) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            Self::push_paint_data(
                props,
                Shape::Line(Line {
                    x1,
                    y1,
                    x2,
                    y2,
                    color,
                }),
            );
        }
        self
    }

    /// ### with_map
    ///
    /// Add a map to canvas
    pub fn with_map(&mut self, resolution: MapResolution, color: Color) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            Self::push_paint_data(props, Shape::Map(Map { resolution, color }));
        }
        self
    }

    /// ### with_points
    ///
    /// Add points to canvas
    pub fn with_points(&mut self, coordinates: Vec<(f64, f64)>, color: Color) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            Self::push_paint_data(props, Shape::Points((coordinates, color)));
        }
        self
    }

    /// ### with_rectangle
    ///
    /// Add rectangle to canvas
    pub fn with_rectangle(
        &mut self,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        color: Color,
    ) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            Self::push_paint_data(
                props,
                Shape::Rectangle(Rectangle {
                    x,
                    y,
                    width,
                    height,
                    color,
                }),
            );
        }
        self
    }

    /// ### push_paint_data
    ///
    /// Push paint to data; if data doesn't exist initialize it
    fn push_paint_data(props: &mut Props, p: Shape) {
        match props.own.get_mut(PROP_SHAPES) {
            Some(PropPayload::Vec(data)) => {
                data.push(PropValue::Shape(p));
            }
            _ => {
                props
                    .own
                    .insert(PROP_SHAPES, PropPayload::Vec(vec![PropValue::Shape(p)]));
            }
        }
    }
}

// -- States

/// ## OwnStates
///
/// Canvas states
struct OwnStates {
    focus: bool,
}

impl Default for OwnStates {
    fn default() -> Self {
        Self { focus: false }
    }
}

// -- Component

/// ## Canvas
///
/// The Canvas widget may be used to draw more detailed figures using braille patterns (each cell can have a braille character in 8 different positions).
pub struct Canvas {
    props: Props,
    states: OwnStates,
}

impl Canvas {
    /// ### new
    ///
    /// Instantiates a new `Canvas`
    pub fn new(props: Props) -> Self {
        Self {
            props,
            states: OwnStates::default(),
        }
    }

    /// ### draw_shape
    ///
    /// Draw a shape into the canvas `Context`
    fn draw_shape(ctx: &mut Context, shape: &Shape) {
        match shape {
            //Shape::Label((x, y, s, c)) => ctx.print(*x, *y, &s, *c),
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

impl Component for Canvas {
    /// ### render
    ///
    /// Based on the current properties and states, renders a widget using the provided render engine in the provided Area
    /// If focused, cursor is also set (if supported by widget)
    fn render(&self, render: &mut Frame, area: Rect) {
        if self.props.visible {
            let title: Option<&str> = match self.props.own.get(PROP_TITLE).as_ref() {
                Some(PropPayload::One(PropValue::Str(t))) => Some(t),
                _ => None,
            };
            let mut block: Block =
                super::utils::get_block(&self.props.borders, title, self.states.focus);
            block = block.style(Style::default().bg(self.props.background));
            // Get properties
            let x_bounds: [f64; 2] = self
                .props
                .own
                .get(PROP_X_BOUNDS)
                .map(|x| match x {
                    PropPayload::Tup2((PropValue::F64(a), PropValue::F64(b))) => [*a, *b],
                    _ => [0.0, 0.0],
                })
                .unwrap_or([0.0, 0.0]);
            let y_bounds: [f64; 2] = self
                .props
                .own
                .get(PROP_Y_BOUNDS)
                .map(|x| match x {
                    PropPayload::Tup2((PropValue::F64(a), PropValue::F64(b))) => [*a, *b],
                    _ => [0.0, 0.0],
                })
                .unwrap_or([0.0, 0.0]);
            // Get shapes
            let shapes: Vec<&Shape> = match self.props.own.get(PROP_SHAPES) {
                Some(PropPayload::Vec(shapes)) => shapes
                    .iter()
                    .map(|x| match x {
                        PropValue::Shape(p) => p,
                        _ => panic!("Shapes item is not a of type `Shape`"),
                    })
                    .collect(),
                _ => Vec::new(),
            };
            // Make canvas
            let canvas = TuiCanvas::default()
                .background_color(self.props.background)
                .block(block)
                .x_bounds(x_bounds)
                .y_bounds(y_bounds)
                .paint(|ctx| shapes.iter().for_each(|x| Self::draw_shape(ctx, x)));
            // Render
            render.render_widget(canvas, area);
        }
    }

    /// ### update
    ///
    /// Update component properties
    /// Properties should first be retrieved through `get_props` which creates a builder from
    /// existing properties and then edited before calling update.
    /// Returns a Msg to the view
    fn update(&mut self, props: Props) -> Msg {
        self.props = props;
        Msg::None
    }

    /// ### get_props
    ///
    /// Returns a props builder starting from component properties.
    /// This returns a prop builder in order to make easier to create
    /// new properties for the element.
    fn get_props(&self) -> Props {
        self.props.clone()
    }

    /// ### on
    ///
    /// Handle input event and update internal states.
    /// Returns a Msg to the view
    fn on(&mut self, ev: Event) -> Msg {
        if let Event::Key(key) = ev {
            Msg::OnKey(key)
        } else {
            Msg::None
        }
    }

    /// ### get_state
    ///
    /// Get current state from component
    /// This component always returns `None`
    fn get_state(&self) -> Payload {
        Payload::None
    }

    // -- events

    /// ### blur
    ///
    /// Blur component; basically remove focus
    fn blur(&mut self) {
        self.states.focus = false;
    }

    /// ### active
    ///
    /// Active component; basically give focus
    fn active(&mut self) {
        self.states.focus = true;
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::event::{KeyCode, KeyEvent};

    use pretty_assertions::assert_eq;

    #[test]
    fn test_component_canvas() {
        let mut component: Canvas = Canvas::new(
            CanvasPropsBuilder::default()
                .hidden()
                .visible()
                .with_background(Color::Black)
                .with_title(String::from("playing risiko"))
                .with_borders(Borders::ALL, BorderType::Rounded, Color::LightYellow)
                .with_x_bounds((-180.0, 180.0))
                .with_y_bounds((-90.0, 90.0))
                .with_new_drawing()
                .with_map(MapResolution::High, Color::Rgb(240, 240, 240))
                .with_layer()
                .with_line(0.0, 10.0, 10.0, 10.0, Color::Red)
                .with_rectangle(60.0, 20.0, 70.0, 22.0, Color::Cyan)
                .with_points(
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
                )
                .build(),
        );
        assert_eq!(component.props.background, Color::Black);
        assert_eq!(component.props.visible, true);
        assert_eq!(component.props.borders.borders, Borders::ALL);
        assert_eq!(component.props.borders.variant, BorderType::Rounded);
        assert_eq!(component.props.borders.color, Color::LightYellow);
        assert!(component.props.own.get(PROP_SHAPES).is_some());
        assert!(component.props.own.get(PROP_X_BOUNDS).is_some());
        assert!(component.props.own.get(PROP_Y_BOUNDS).is_some());
        // focus
        component.active();
        assert_eq!(component.states.focus, true);
        component.blur();
        assert_eq!(component.states.focus, false);
        // Keys
        assert_eq!(
            component.on(Event::Key(KeyEvent::from(KeyCode::Char('a')))),
            Msg::OnKey(KeyEvent::from(KeyCode::Char('a'))),
        );
    }

    #[test]
    fn test_component_canvas_with_shapes() {
        let component: Canvas = Canvas::new(
            CanvasPropsBuilder::default()
                .hidden()
                .visible()
                .with_background(Color::Black)
                .with_title(String::from("playing risiko"))
                .with_borders(Borders::ALL, BorderType::Rounded, Color::LightYellow)
                .with_x_bounds((-180.0, 180.0))
                .with_y_bounds((-90.0, 90.0))
                .with_shapes(vec![
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
                ])
                .build(),
        );
        assert_eq!(component.props.background, Color::Black);
        assert_eq!(component.props.visible, true);
        assert_eq!(component.props.borders.borders, Borders::ALL);
        assert_eq!(component.props.borders.variant, BorderType::Rounded);
        assert_eq!(component.props.borders.color, Color::LightYellow);
        assert!(component.props.own.get(PROP_SHAPES).is_some());
        assert!(component.props.own.get(PROP_X_BOUNDS).is_some());
        assert!(component.props.own.get(PROP_Y_BOUNDS).is_some());
    }
}
