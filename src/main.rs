use std::io::Write;
use iced::{
    Alignment, Element, Length, Task, Theme, widget::{button, column, container, row, text}, window::{Settings}
};

mod temp;
use temp::TempGraph;
mod tablespace;
use tablespace::TablespaceTable;

mod db;

#[derive(Debug, Clone)]
pub enum Message {
    Tablespace(tablespace::Message),
    Temp(temp::Message)
}

#[derive(Debug, Clone)]
pub enum MenuItem {
    Tablespace,
    Temp,
}

impl Default for MenuItem {
    fn default() -> Self {
        Self::Temp    }
}

#[derive(Default,Debug)]
struct MainApp {
    selected: MenuItem,
    temp: TempGraph,
    tablespace: TablespaceTable
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
            Message::Tablespace(message) => {
                self.selected = MenuItem::Tablespace;
                self.tablespace.update(message).map(Message::Tablespace)
            }
            Message::Temp(temp_message) => {
                self.selected = MenuItem::Temp;

                self.temp.update(temp_message).map(Message::Temp)
            }
        }
    }

    fn view(&'_ self) -> Element<'_, Message> {
        let menu = column![
            button("Temp")
                .width(Length::Fill)
                .on_press(Message::Temp(temp::Message::Load)),
            button("Tablespace")
                .width(Length::Fill)
                .on_press(Message::Tablespace(tablespace::Message::Load)),
        ]
        .spacing(2)
        .padding(5)
        .width(Length::Fixed(150.0));

        // RIGHT SIDE READING PANE CONTENT
        let content: Element<_> = match &self.selected {
            MenuItem::Tablespace => {
                self.tablespace.view().map(Message::Tablespace)
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