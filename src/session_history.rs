use iced::{Element, Task, widget::{column, row, button,stack} };

use crate::db::queries::{fetch_session_history_data,fetch_sqlid_data,TableResult};
use crate::components::table::TableState;
use crate::components::popup::PopUp;
use crate::db::{load_async, DbError};
use crate::db::load::{sqlid_as_text};
use chrono::{NaiveDateTime, Local, Duration};

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
    Loaded(TableResult),
    SQLID(TableResult),
    CellClicked(usize,String), 
    ClosePopup,
    StartChanged(String),
    EndChanged(String),
    PresetLast1Hour,
    PresetLast24Hours,
    PresetToday,
    PresetYesterday,
    PresetLast7Days,
}

#[derive(Debug, Clone)]
pub struct SessionHistoryTable {
    pub state: TableState,
    pub start_str: String,
    pub end_str: String,
    show_popup: bool,
    sqlid_result: Option<TableResult>,
    pub popup: PopUp
}

impl Default for SessionHistoryTable {
    fn default() -> Self {
        let now = Local::now().naive_local();
        let start = now - Duration::hours(1);
        Self {
            state: TableState::default(),
            start_str: start.format("%Y-%m-%d %H:%M:%S").to_string(),
            end_str: now.format("%Y-%m-%d %H:%M:%S").to_string(),
            show_popup: false,
            sqlid_result: None,
            popup: PopUp::default()
        }
    }
}

impl SessionHistoryTable {
    pub fn update(&mut self,message: Message) -> Task<Message> {
        match message {
            Message::Load => {
                self.state.begin_load();
                // Parse dates provided by user (stored in start_str/end_str). If parsing fails,
                // set an error into the table state and do not start the async task.
                let start = if !self.start_str.is_empty() {
                    match NaiveDateTime::parse_from_str(&self.start_str, "%Y-%m-%d %H:%M:%S") {
                        Ok(dt) => dt,
                        Err(e) => {
                            self.state.apply_loaded(Err(DbError::Config(format!("Invalid start date: {}", e))));
                            return Task::none();
                        }
                    }
                } else {
                    self.state.apply_loaded(Err(DbError::Config("Start date is empty".into())));
                    return Task::none();
                };

                let end = if !self.end_str.is_empty() {
                    match NaiveDateTime::parse_from_str(&self.end_str, "%Y-%m-%d %H:%M:%S") {
                        Ok(dt) => dt,
                        Err(e) => {
                            self.state.apply_loaded(Err(DbError::Config(format!("Invalid end date: {}", e))));
                            return Task::none();
                        }
                    }
                } else {
                    self.state.apply_loaded(Err(DbError::Config("End date is empty".into())));
                    return Task::none();
                };

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
            Message::PresetLast1Hour => {
                let now = Local::now().naive_local();
                let start = now - Duration::hours(1);
                self.start_str = start.format("%Y-%m-%d %H:%M:%S").to_string();
                self.end_str = now.format("%Y-%m-%d %H:%M:%S").to_string();
                Task::none()
            }
            Message::PresetLast24Hours => {
                let now = Local::now().naive_local();
                let start = now - Duration::hours(24);
                self.start_str = start.format("%Y-%m-%d %H:%M:%S").to_string();
                self.end_str = now.format("%Y-%m-%d %H:%M:%S").to_string();
                Task::none()
            }
            Message::PresetToday => {
                let now = Local::now().naive_local();
                let start = now.date().and_hms_opt(0,0,0).unwrap();
                let end = now.date().and_hms_opt(23,59,59).unwrap();
                self.start_str = start.format("%Y-%m-%d %H:%M:%S").to_string();
                self.end_str = end.format("%Y-%m-%d %H:%M:%S").to_string();
                Task::none()
            }
            Message::PresetYesterday => {
                let now = Local::now().naive_local();
                let yesterday_dt = now - Duration::days(1);
                let yesterday = yesterday_dt.date();
                let start = yesterday.and_hms_opt(0,0,0).unwrap();
                let end = yesterday.and_hms_opt(23,59,59).unwrap();
                self.start_str = start.format("%Y-%m-%d %H:%M:%S").to_string();
                self.end_str = end.format("%Y-%m-%d %H:%M:%S").to_string();
                Task::none()
            }
            Message::PresetLast7Days => {
                let now = Local::now().naive_local();
                let start = now - Duration::days(7);
                self.start_str = start.format("%Y-%m-%d %H:%M:%S").to_string();
                self.end_str = now.format("%Y-%m-%d %H:%M:%S").to_string();
                Task::none()
            }
            Message::Loaded(result) => {
                self.state.apply_loaded(result);
                Task::none()
            }
            Message::SQLID(result) => {
                self.sqlid_result = Some(result);
                Task::none()
            }
            Message::CellClicked(col, txt) => {
                let sql_col = SESSION_HISTORY_HEADERS.iter().position(|&h| h == "SQL ID").unwrap();
                if col == sql_col { 
                    self.sqlid_result = None;
                    self.show_popup = true;
                    Task::perform(load_async( move || fetch_sqlid_data(&txt)), Message::SQLID)
                } else {
                    Task::none()
                }
            }
            Message::ClosePopup => {
                self.show_popup = false;
                Task::none()
            }
        }
    }

    pub fn view(self: &'_ Self) -> Element<'_, Message> { 
        // preset buttons + inputs for start and end dates plus a load button
        let presets = row![
            button("Last 1h").on_press(Message::PresetLast1Hour),
            button("Last 24h").on_press(Message::PresetLast24Hours),
            button("Today").on_press(Message::PresetToday),
            button("Yesterday").on_press(Message::PresetYesterday),
            button("Last 7d").on_press(Message::PresetLast7Days),
        ]
        .spacing(8)
        .padding(6);

        let start_input = iced::widget::TextInput::new("YYYY-MM-DD HH:MM:SS", &self.start_str)
            .on_input(Message::StartChanged)
            .padding(6)
            .size(14);
        let end_input = iced::widget::TextInput::new("YYYY-MM-DD HH:MM:SS", &self.end_str)
            .on_input(Message::EndChanged)
            .padding(6)
            .size(14);
        let load_btn = button("Load").on_press(Message::Load);

        // let on_click: Option<fn(usize, String) -> Message> = None;
        let on_click: Option<fn(usize, String) -> Message> = Some(|i,s| Message::CellClicked(i, s));
        
        let main_content = column![
            presets,
            row![start_input, end_input, load_btn].spacing(10).padding(10),
            self.state.view(SESSION_HISTORY_HEADERS, on_click)
        ]
        .spacing(10);
        
        if self.show_popup {
            stack![
                main_content,
                self.popup.pop_up(Message::ClosePopup, sqlid_as_text(&self.sqlid_result).clone())
            ]
            .into()
        } else {
            main_content.into()
        }
    }
}
