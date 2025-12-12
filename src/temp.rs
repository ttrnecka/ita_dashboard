use chrono::{NaiveDateTime, Duration};
use iced::mouse;
use iced::widget::{ canvas, text};
use iced::widget::canvas::{Stroke, Path, Text, Frame};
use iced::{ Rectangle, Renderer, Point, Length, Theme, Color, Task, Element};

use crate::db::queries::{fetch_temp_data,DataPoint};

#[derive(Default, Debug, Clone)]
pub struct Graph {
    pub loading: bool,
    pub data: Option<Vec<DataPoint>>,
    pub error: Option<String>,
    pub t0: NaiveDateTime,
    pub x: Vec<f32>,
    pub y1: Vec<f32>,
    pub y2: Vec<f32>,
    pub xmin: f32,
    pub xmax: f32,
    pub ymin: f32,
    pub ymax: f32,
}

#[derive(Debug, Clone)]
pub enum Message {
    Load,
    Loaded(Result<Vec<DataPoint>, String>),
}


impl Graph {
    pub fn update(&mut self,message: Message) -> Task<Message> {
        match message {
            Message::Load => {
                self.loading = true;
                self.data = None;
                self.error = None;
                Task::perform(load_temp_async(), Message::Loaded)
            }
            Message::Loaded(result) => {
                self.loading = false;

                match result {
                    Ok(vals) => {
                        self.t0 = vals[0].ts;
                        self.data = Some(vals);
                        (self.x, self.y1, self.y2) = self.split_series(&self.data.as_ref().unwrap());
                        self.xmin = self.x
                            .iter()
                            .cloned()
                            .fold(f32::INFINITY, f32::min);

                        self.xmax = self.x
                            .iter()
                            .cloned()
                            .fold(f32::NEG_INFINITY, f32::max);

                        // self.ymin = self.y1
                        //     .iter()
                        //     .chain(self.y2.iter())
                        //     .cloned()
                        //     .fold(f32::INFINITY, f32::min);

                        self.ymin = 0 as f32;

                        self.ymax = self.y1
                            .iter()
                            .chain(self.y2.iter())
                            .cloned()
                            .fold(f32::NEG_INFINITY, f32::max);
                    }
                    Err(err) => {
                        eprintln!("Oracle error: {}", err);
                        self.error = Some(err)
                    },
                }
                Task::none()
            }
        }
    }

    pub fn view(self: &'_ Self) -> Element<'_, Message> { 
        if self.loading {
            text("Loading data...").into()
        } else if let Some(_data) = &self.data {
            canvas(self.clone())
                .width(Length::Fill)
                .height(Length::Fill).into()
        } else if let Some(err) = &self.error {
            iced::widget::text(err).into()
        } else {
            iced::widget::text("No data").into()
        }
    }

    fn split_series(self: &'_ Self, data: &[DataPoint]) -> (Vec<f32>, Vec<f32>, Vec<f32>) {
        let mut x = Vec::new();
        let mut y1 = Vec::new();
        let mut y2 = Vec::new();

        for p in data {
            let dt = (p.ts - self.t0).num_seconds() as f32;
            x.push(dt);
            y1.push(p.a);
            y2.push(p.b);
        }

        (x, y1, y2)
    }

    fn scale_series(
        &self,
        width: f32,
        height: f32,
        pad: f32,
    ) -> Vec<Vec<Point>> {
        let xr = (self.xmax - self.xmin).max(1.0);
        let yr = (self.ymax - self.ymin).max(1.0);

        let iw = width - 2.0 * pad;
        let ih = height - 2.0 * pad;

        [&self.y1, &self.y2].iter()
            .map(|series| {
                series.iter().zip(self.x.iter()).map(|(&y, &xx)| {
                    let sx = pad + (xx - self.xmin) / xr * iw;
                    let sy = height - pad - (y - self.ymin) / yr * ih;
                    Point::new(sx, sy)
                }).collect()
            })
            .collect()
    }
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

        let pad = 50.0;

        let scaled = self.scale_series(
            w,
            h,
            pad,
        );

        draw_axes(
            &mut frame,
            w,
            h,
            pad,
            self.ymin, self.ymax,
            self.xmin, self.xmax,
            self.t0
        );

        draw_series(
            &mut frame,
            &scaled[0],
            Color::from_rgb(0.2, 0.6, 1.0),
        );

        // Series B (orange)
        draw_series(
            &mut frame,
            &scaled[1],
            Color::from_rgb(1.0, 0.5, 0.1),
        );
        vec![frame.into_geometry()]
    }
}


pub async fn load_temp_async() -> Result<Vec<DataPoint>, String> {
    tokio::task::spawn_blocking(|| fetch_temp_data())
        .await
        .map_err(|e| e.to_string())?
        .map_err(|e| e.to_string())
}

fn draw_axes(
    frame: &mut Frame,
    w: f32,
    h: f32,
    pad: f32,
    ymin: f32,
    ymax: f32,
    xmin: f32,
    xmax: f32,
    t0: chrono::NaiveDateTime,
) {
    let axis = Stroke::default().with_width(2.0);

    // X axis
    frame.stroke(
        &Path::line(
            Point::new(pad, h - pad),
            Point::new(w - pad, h - pad),
        ),
        axis.clone(),
    );

    // Y axis
    frame.stroke(
        &Path::line(
            Point::new(pad, pad),
            Point::new(pad, h - pad),
        ),
        axis,
    );

    // --- Y ticks + labels ---
    for i in 0..5 {
        let t = i as f32 / 4.0;
        let y = pad + t * (h - 2.0 * pad);
        let v = ymax - t * (ymax - ymin);

        frame.fill_text(Text {
            content: format!("{:.1}", v),
            position: Point::new(5.0, y - 7.0),
            color: Color::WHITE,
            size: 14.into(),
            ..Text::default()
        });
    }

    // -------- X labels as DATE/TIME --------
    let ticks = ((w / 120.0) as usize).max(2);
    for i in 0..=ticks {
        let t = i as f32 / 4.0;
        let x_pos = pad + t * (w - 2.0 * pad);
        let secs = xmin + t * (xmax - xmin);

        let dt = seconds_to_datetime(t0, secs);

        frame.fill_text(Text {
            content: dt.format("%Y-%m-%d\n%H:%M").to_string(), // two lines
            position: Point::new(x_pos - 30.0, h - pad + 6.0),
            color: Color::WHITE,
            size: 12.0.into(),
            ..Text::default()
        });
    }
}

fn smooth(points: &[Point]) -> iced::widget::canvas::Path {

    canvas::Path::new(|builder|{
    
        builder.move_to(points[0]);

        for i in 1..points.len() - 1 {
            let p0 = points[i - 1];
            let p1 = points[i];
            let p2 = points[i + 1];

            let cx = (p0.x + p1.x) / 2.0;
            let cy = (p0.y + p1.y) / 2.0;

            builder.quadratic_curve_to(Point::new(cx, cy), p1);
            builder.line_to(p2);
        }
    })
}

fn draw_series(frame: &mut Frame, pts: &[Point], color: Color) {
    let stroke = Stroke::default()
        .with_width(1.0)
        .with_color(color);

    let path = smooth(pts);
    frame.stroke(&path, stroke);
}

fn seconds_to_datetime(t0: NaiveDateTime, secs: f32) -> NaiveDateTime {
    t0 + Duration::seconds(secs as i64)
}