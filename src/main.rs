use std::io::Write;
use iced::{
    Alignment, Element, Length, Task, Theme, widget::{button, column, container, row, text}, window::{Settings}
};

mod temp;
use temp::Graph;

mod db;

#[derive(Debug, Clone)]
pub enum Message {
    Dashboard,
    Temp(temp::Message)
}

#[derive(Debug, Clone)]
pub enum MenuItem {
    Dashboard,
    Temp,
}

impl Default for MenuItem {
    fn default() -> Self {
        Self::Dashboard    }
}

#[derive(Default,Debug)]
struct MainApp {
    selected: MenuItem,
    temp: Graph,
}

impl MainApp {
    fn theme(&self) -> Theme {
        Theme::TokyoNightStorm
    }

    fn title(&self) -> String {
        "ITA Dashboard".into()
    }
    // fn title(&self) -> Theme {
    //     "ITA Dashboard"
    // }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Dashboard => {
                self.selected = MenuItem::Dashboard;
                Task::none()
            }
            Message::Temp(temp_message) => {
                self.selected = MenuItem::Temp;

                self.temp.update(temp_message).map(Message::Temp)
            }
        }
    }

    fn view(&'_ self) -> Element<'_, Message> {
        let menu = column![
            button("Dashboard")
                .width(Length::Fill)
                .on_press(Message::Dashboard),
            button("Temp")
                .width(Length::Fill)
                .on_press(Message::Temp(temp::Message::Load)),
        ]
        .spacing(2)
        .padding(5)
        .width(Length::Fixed(150.0));

        // RIGHT SIDE READING PANE CONTENT
        let content: Element<_> = match &self.selected {
            MenuItem::Dashboard => {
                text("Dashboard content here").into()
            }
            MenuItem::Temp => {
                self.temp.view().map(Message::Temp)
            }
        };

        let reading_pane = container(content)
            .padding(20)
            .width(Length::Fill);

        // LAYOUT: menu left, content right
        row![menu, reading_pane]
            .align_y(Alignment::Start)
            .into()
    }
}


fn main() -> iced::Result {
    let _ = std::io::stderr().flush();

    iced::application(MainApp::default,MainApp::update, MainApp::view)
    .theme(MainApp::theme)
    .title(MainApp::title)
    .centered()
    .window(Settings { maximized: true, ..Default::default() })
    .run()
}