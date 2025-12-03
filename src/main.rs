use std::io::Write;

use iced::{
    Alignment, Element, Length, Task, Theme, widget::{button, column, container, row, text}
};


mod temp;
use temp::Graph;

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

fn theme(_state: &MainApp) -> Theme {
    Theme::TokyoNightStorm
}

fn update(state: &mut MainApp, message: Message) -> Task<Message> {
    match message {
        Message::Dashboard => {
            state.selected = MenuItem::Dashboard;
            Task::none()
        }
        Message::Temp(temp_message) => {
            state.selected = MenuItem::Temp;

            state.temp.update(temp_message).map(Message::Temp)
        }
    }
}

fn view(state: &'_ MainApp) -> Element<'_, Message> {
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
    let content: Element<_> = match &state.selected {
        MenuItem::Dashboard => {
            text("Dashboard content here").into()
        }
        MenuItem::Temp => {
            state.temp.view().map(Message::Temp)
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
fn main() -> iced::Result {
    let _ = std::io::stderr().flush();

    iced::application("ITA Dashboard",update, view)
    .theme(theme).run()
}