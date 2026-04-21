#![windows_subsystem = "windows"]
use std::io::Write;
use iced::{
    Alignment, Element, Length, Task, Theme, widget::{button, column, container, row}, window::{Settings, icon}
};

mod temp;
use temp::TempGraph;
mod tablespace;
use tablespace::TablespaceTable;
mod filesystem;
use filesystem::FilesystemTable;
mod session_history;
use session_history::SessionHistoryTable;
mod session_temp;
use session_temp::SessionTempTable;
mod constants;
mod components;
mod db;

#[derive(Debug, Clone)]
pub enum Message {
    Tablespace(tablespace::Message),
    Filesystem(filesystem::Message),
    SessionHistory(session_history::Message),
    SessionTemp(session_temp::Message),
    Temp(temp::Message)
    
}

#[derive(Debug, Clone)]
pub enum MenuItem {
    Tablespace,
    Filesystem,
    SessionHistory,
    SessionTemp,
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
    tablespace: TablespaceTable,
    filesystem: FilesystemTable,
    session_history: SessionHistoryTable,
    session_temp: SessionTempTable,
}

impl MainApp {
    fn theme(&self) -> Theme {
        Theme::TokyoNightStorm
    }

    fn title(&self) -> String {
        "ITA Dashboard".into()
    }

    fn boot() -> (MainApp, Task<Message>) {
        (MainApp::default(), Task::done(Message::Temp(temp::Message::Load)))
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Tablespace(message) => {
                self.selected = MenuItem::Tablespace;
                self.tablespace.update(message).map(Message::Tablespace)
            }
            Message::Filesystem(message) => {
                self.selected = MenuItem::Filesystem;
                self.filesystem.update(message).map(Message::Filesystem)
            }
            Message::SessionHistory(message) => {
                self.selected = MenuItem::SessionHistory;
                self.session_history.update(message).map(Message::SessionHistory)
            }
            Message::SessionTemp(message) => {
                self.selected = MenuItem::SessionTemp;
                self.session_temp.update(message).map(Message::SessionTemp)
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
            button("Session Temp")
                .width(Length::Fill)
                .on_press(Message::SessionTemp(session_temp::Message::Load)),
            button("Tablespace")
                .width(Length::Fill)
                .on_press(Message::Tablespace(tablespace::Message::Load)),
            button("Filesystem")
                .width(Length::Fill)
                .on_press(Message::Filesystem(filesystem::Message::Load)),
            button("Session History")
                .width(Length::Fill)
                .on_press(Message::SessionHistory(session_history::Message::PresetLast1Hour)),
        ]
        .spacing(2)
        .padding(5)
        .width(Length::Fixed(150.0));

        // RIGHT SIDE READING PANE CONTENT
        let content: Element<_> = match &self.selected {
            MenuItem::Tablespace => {
                self.tablespace.view().map(Message::Tablespace)
            }
            MenuItem::Filesystem => {
                self.filesystem.view().map(Message::Filesystem)
            }
            MenuItem::SessionHistory => {
                self.session_history.view().map(Message::SessionHistory)
            }
            MenuItem::SessionTemp => {
                self.session_temp.view().map(Message::SessionTemp)
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

    let icon = match icon::from_file("icon.png") {
        Ok(icon) => Some(icon),
        Err(e) => {
            eprintln!("Failed to load window icon: {}", e);
            None
        }
    };

    iced::application(MainApp::boot,MainApp::update, MainApp::view)
    .theme(MainApp::theme)
    .title(MainApp::title)
    .centered()
    .window(Settings { maximized: true, icon, ..Default::default() })
    .run()
}