use iced::{Element, Task, widget::{column, row, button } };

use crate::db::queries::{fetch_session_history_data};
use crate::components::table::TableState;
use crate::db::load_async;
use chrono::NaiveDateTime;

const SESSION_HISTORY_HEADERS: &[&str] = &[
    "SID",
    "Serial#",
    "SQL ID",
    "OP NAME",
    "Start Time",
    "End Time",
    "Duration secs",
    "Duration mins",
    "Max Temp MB",
];
#[derive(Debug, Clone)]
pub enum Message {
    Load,
    Loaded(Result<Vec<Vec<String>>, String>),
    StartChanged(String),
    EndChanged(String),
}

#[derive(Default, Debug, Clone)]
pub struct SessionHistoryTable {
    pub state: TableState,
    pub start_str: String,
    pub end_str: String,
}

impl SessionHistoryTable {
    pub fn update(&mut self,message: Message) -> Task<Message> {
        match message {
            Message::Load => {
                // Parse dates provided by user (stored in start_str/end_str). If parsing fails,
                // set an error into the table state and do not start the async task.
                let start = if !self.start_str.is_empty() {
                    match NaiveDateTime::parse_from_str(&self.start_str, "%Y-%m-%d %H:%M:%S") {
                        Ok(dt) => dt,
                        Err(e) => {
                            self.state.apply_loaded(Err(format!("Invalid start date: {}", e)));
                            return Task::none();
                        }
                    }
                } else {
                    self.state.apply_loaded(Err("Start date is empty".into()));
                    return Task::none();
                };

                let end = if !self.end_str.is_empty() {
                    match NaiveDateTime::parse_from_str(&self.end_str, "%Y-%m-%d %H:%M:%S") {
                        Ok(dt) => dt,
                        Err(e) => {
                            self.state.apply_loaded(Err(format!("Invalid end date: {}", e)));
                            return Task::none();
                        }
                    }
                } else {
                    self.state.apply_loaded(Err("End date is empty".into()));
                    return Task::none();
                };

                self.state.begin_load();
                Task::perform(load_async(move || fetch_session_history_data(start, end)), Message::Loaded)
            }
            Message::StartChanged(s) => {
                self.start_str = s;
                Task::none()
            }
            Message::EndChanged(s) => {
                self.end_str = s;
                Task::none()
            }
            Message::Loaded(result) => {
                self.state.apply_loaded(result);
                Task::none()
            }
        }
    }

    pub fn view(self: &'_ Self) -> Element<'_, Message> { 
        // inputs for start and end dates plus a load button
        let start_input = iced::widget::TextInput::new("YYYY-MM-DD HH:MM:SS", &self.start_str)
            .on_input(Message::StartChanged)
            .padding(6)
            .size(14);
        let end_input = iced::widget::TextInput::new("YYYY-MM-DD HH:MM:SS", &self.end_str)
            .on_input(Message::EndChanged)
            .padding(6)
            .size(14);
        let load_btn = button("Load").on_press(Message::Load);

        column![
            row![start_input, end_input, load_btn].spacing(10).padding(10),
            self.state.view::<Message>(SESSION_HISTORY_HEADERS)
        ]
        .spacing(10)
        .into()
    }
}
