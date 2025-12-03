use oracle::{Connection, Error};
use iced::mouse;
use iced::widget::{ canvas, text};
use iced::widget::canvas::{Stroke, Path, LineCap, LineJoin};
use iced::{ Rectangle, Renderer, Point, Length, Theme, Color, Task, Element};

#[derive(Default, Debug, Clone)]
pub struct Graph {
    pub loading: bool,
    pub data: Option<Vec<f32>>,
    pub error: Option<String>,
}

#[derive(Debug, Clone)]
pub enum Message {
    Load,
    Loaded(Result<Vec<f32>, String>),
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
                    Ok(vals) => self.data = Some(vals),
                    Err(err) => {
                        eprintln!("Oracle error: {}", err);
                        self.error = Some(err)
                    },
                }
                Task::none()
            }
        }
    }

    pub fn view(self: &'_ Self) -> Element<'_, Message> { // canvas::Canvas<Graph,Message> {
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
        // if let None = self.data {
        //     return vec![frame.into_geometry()];
        // }

        let data = self.data.clone().unwrap();
        // Build a path from the data points
        let path = canvas::Path::new(|builder|{
            let usable_width = w - 40.0;
            let usable_height = x_axis_y;

            let step = usable_width
                / ((data.len() - 1) as f32).max(1.0);

            // find max to scale vertical values
            let max_v = data
                .iter()
                .cloned()
                .fold(f32::MIN, f32::max)
                .max(1.0);

            for (i, &v) in data.iter().enumerate() {
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
            width: 1.0,
            line_cap: LineCap::Round,
            line_join: LineJoin::Round,
            style: canvas::Style::Solid(Color::WHITE),
            ..Stroke::default()
        };
        // draw the path onto the frame
        frame.stroke(&path, stroke);

        vec![frame.into_geometry()]
    }
}


pub async fn load_temp_async() -> Result<Vec<f32>, String> {
    tokio::task::spawn_blocking(|| fetch_temp_data())
        .await
        .map_err(|e| e.to_string())?
        .map_err(|e| e.to_string())
}

fn fetch_temp_data() -> Result<Vec<f32>, Error> {
    let conn = Connection::connect("xxx", "xxx", "1.1.1.1:1521/db")?;

    let sql = r#"
        SELECT 100 * used_gb / nullif(total_gb,0) 
        FROM dxc_v_temp_log
        WHERE log_date > SYSDATE - 7
        ORDER BY log_date
    "#;

    let mut values = Vec::new();

    for row_result in conn.query(sql, &[])? {
        let row = row_result?;
        let val: f32 = row.get(0)?;
        values.push(val);
    }

    Ok(values)
}
