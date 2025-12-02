// src/graph.rs
use iced::mouse;
use iced::widget::canvas;
use iced::widget::canvas::{Stroke, Path, LineCap, LineJoin};
use iced::{ Rectangle, Renderer, Point, Length, Theme, Color};

// use iced::{
//     widget::canvas::{self, Canvas, Frame, Geometry, Path, Stroke},
//     Point, Rectangle, Length, Element,
// };

/// Simple graph program for iced::canvas
pub struct Graph {
    pub data: Vec<f32>,
}

impl<Message> canvas::Program<Message> for Graph {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        // Create a frame sized to the available bounds
        let mut frame = canvas::Frame::new(renderer, bounds.size());

        let w = bounds.width.max(1.0);
        let h = bounds.height.max(1.0);

        let axis_stroke = || Stroke::default()
           .with_width(2.0)
           .with_color(Color::from_rgb(0.8, 0.8, 0.8));

        let y_axis = Path::line(
            Point::new(40.0, 0.0),
            Point::new(40.0, h),
        );
        frame.stroke(&y_axis, axis_stroke());

        let x_axis_y = h - 40.0;
        let x_axis = Path::line(
            Point::new(0.0, x_axis_y),
            Point::new(w, x_axis_y),
        );
        frame.stroke(&x_axis, axis_stroke());

        // early return if no data
        if self.data.is_empty() {
            return vec![frame.into_geometry()];
        }

        // Build a path from the data points
        let path = canvas::Path::new(|builder|{
            let usable_width = w - 40.0;
            let usable_height = x_axis_y;

            let step = usable_width
                / ((self.data.len() - 1) as f32).max(1.0);

            // find max to scale vertical values
            let max_v = self
                .data
                .iter()
                .cloned()
                .fold(f32::MIN, f32::max)
                .max(1.0);

            for (i, &v) in self.data.iter().enumerate() {
                let x = 40.0 + (i as f32 * step);
                let y = usable_height - (v / max_v) * usable_height;
                if i == 0 {
                    builder.move_to(Point::new(x, y));
                } else {
                    builder.line_to(Point::new(x, y));
                }
            }
          }
        );
        let stroke = Stroke {
            width: 2.0,
            line_cap: LineCap::Round,
            line_join: LineJoin::Round,
            ..Stroke::default()
        };
        // draw the path onto the frame
        frame.stroke(&path, stroke);

        vec![frame.into_geometry()]
    }
}

/// Return a Canvas element that can be placed in your view()
pub fn view<Message>(data: Vec<f32>) -> canvas::Canvas<Graph,Message> {
    canvas(Graph { data })
        .width(Length::Fill)
        .height(Length::Fill)
}


// pub fn view<Message>(data: Vec<f32>) -> canvas::Canvas<Graph> {
//     let canvas = canvas::Canvas::new(Graph { data })
//         .width(Length::Fill)
//         .height(Length::Fill);
//     canvas
// }