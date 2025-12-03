use std::io::Write;

use iced::{
    executor, Application, Command, Element,
    Settings, Theme, widget::{column, row, button, text, container},
    Alignment, Length,
};

mod graph;
mod oracle;

#[derive(Debug, Clone)]
pub enum Message {
    MenuSelected(MenuItem),
    DataLoaded(Result<Vec<f32>, String>),
}

#[derive(Debug, Clone)]
pub enum MenuItem {
    Dashboard,
    Reports,
    Graph1,
}

struct MainApp {
    selected: MenuItem,
    data: Vec<f32>
}

impl Application for MainApp {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Self::Message>) {
        (
            Self {
                selected: MenuItem::Dashboard,
                data: Vec::new(),
            },
            Command::perform(load_data_async(), |res| Message::DataLoaded(res))
        )
    }

    fn theme(&self) -> Theme {
        Theme::TokyoNightStorm
    }

    fn title(&self) -> String {
        "Iced Dashboard".into()
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::MenuSelected(item) => {
                self.selected = item;
            }
            Message::DataLoaded(Ok(vals)) => {
                self.data = vals;
            }
            Message::DataLoaded(Err(err)) => {
                eprintln!("Oracle error: {}", err);
            }
        }
        Command::none()
    }

    fn view(&'_ self) -> Element<'_, Self::Message> {
        let menu = column![
            button("Dashboard")
                .width(Length::Fill)
                .on_press(Message::MenuSelected(MenuItem::Dashboard)),
            button("Reports")
                .width(Length::Fill)
                .on_press(Message::MenuSelected(MenuItem::Reports)),
            button("Temp")
                .width(Length::Fill)
                .on_press(Message::MenuSelected(MenuItem::Graph1)),
        ]
        .spacing(2)
        .padding(5)
        .width(Length::Fixed(150.0));

        // RIGHT SIDE READING PANE CONTENT
        let content: Element<_> = match self.selected {
            MenuItem::Dashboard => {
                text("Dashboard content here").into()
            }
            MenuItem::Reports => {
                text("Reports view").into()
            }
            MenuItem::Graph1 => {
                graph::view(self.data.clone()).into()
            }
        };

        let reading_pane = container(content)
            .padding(20)
            .width(Length::Fill);

        // LAYOUT: menu left, content right
        row![menu, reading_pane]
            .align_items(Alignment::Start)
            .into()
    }
}

fn main() -> iced::Result {
    let _ = std::io::stderr().flush();

    MainApp::run(Settings::default())
}

async fn load_data_async() -> Result<Vec<f32>, String> {
    tokio::task::spawn_blocking(|| oracle::fetch_oracle_data())
        .await
        .map_err(|e| e.to_string())?
        .map_err(|e| e.to_string())
}