use iced::{Element, Task };

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
}

#[derive(Default, Debug, Clone)]
pub struct SessionHistoryTable {
    pub state: TableState,
}

impl SessionHistoryTable {
    pub fn update(&mut self,message: Message) -> Task<Message> {
        match message {
            Message::Load => {
                self.state.begin_load();
                let start_date = NaiveDateTime::parse_from_str("2026-04-14 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
                let end_date = NaiveDateTime::parse_from_str("2026-04-14 23:59:59", "%Y-%m-%d %H:%M:%S").unwrap();
                Task::perform(load_async(move || fetch_session_history_data(start_date, end_date)), Message::Loaded)
            }
            Message::Loaded(result) => {
                self.state.apply_loaded(result);
                Task::none()
            }
        }
    }

    pub fn view(self: &'_ Self) -> Element<'_, Message> { 
        self.state.view::<Message>(SESSION_HISTORY_HEADERS)
    }
}
