use iced::{Element, Task, widget::{stack}};

use crate::db::queries::{fetch_session_temp_data,fetch_sqlid_data, TableResult, default_table_result};
use crate::db::load::{sqlid_as_text};
use crate::components::table::TableState;
use crate::components::popup::PopUp;
use crate::db::load_async;

const SESSION_TEMP_HEADERS: &[&str] = &[
    "SID",
    "Serial#",
    "Username",
    "Program", 
    "Tablespace",  
    "Segtype",
    "Temp MB Used",
    "SQL ID",
    "SQL Text"
];
#[derive(Debug, Clone)]
pub enum Message {
    Load,
    Loaded(TableResult),
    SQLID(TableResult),
    CellClicked(usize,String), 
    ClosePopup,
}

#[derive(Debug, Clone)]
pub struct SessionTempTable {
    pub state: TableState,
    show_popup: bool,
    sqlid_result: TableResult,
    pub popup: PopUp
}

impl Default for SessionTempTable {
    fn default() -> Self {
        Self {
            state: TableState::default(),
            show_popup: false,
            sqlid_result: default_table_result(),
            popup: PopUp::default()
        }
    }
}

impl SessionTempTable {
    pub fn update(&mut self,message: Message) -> Task<Message> {
        match message {
            Message::Load => {
                self.state.begin_load();
                Task::perform(load_async(fetch_session_temp_data), Message::Loaded)
            }
            Message::Loaded(result) => {
                self.state.apply_loaded(result);
                Task::none()
            }
            Message::SQLID(result) => {
                // self.state.apply_loaded(result);
                self.sqlid_result = result;
                self.show_popup = true;
                Task::none()
            }
            Message::CellClicked(col, txt) => {
                let sql_col = SESSION_TEMP_HEADERS.iter().position(|&h| h == "SQL ID").unwrap();
                if col == sql_col { 
                    // self.state.begin_load();
                    self.sqlid_result = default_table_result();
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
        let on_click: Option<fn(usize, String) -> Message> = Some(|i,s| Message::CellClicked(i, s));
        if self.show_popup {
            stack![
                self.state.view(SESSION_TEMP_HEADERS, on_click),
                self.popup.pop_up(Message::ClosePopup, sqlid_as_text(&self.sqlid_result).clone())
                    
            ]
            .into()
        } else {
            self.state.view(SESSION_TEMP_HEADERS, on_click).into()
        }
    }
}
