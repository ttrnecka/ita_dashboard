use iced::{Element, Task};

use crate::db::queries::{fetch_session_temp_data};
use crate::components::table::TableState;
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
    Loaded(Result<Vec<Vec<String>>, String>),
}

#[derive(Debug, Clone, Default)]
pub struct SessionTempTable {
    pub state: TableState,
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
        }
    }

    pub fn view(self: &'_ Self) -> Element<'_, Message> { 
        self.state.view::<Message>(SESSION_TEMP_HEADERS)
    }
}
