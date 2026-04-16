use iced::{Element, Task };

use crate::db::queries::{fetch_tablespace_data, TableResult};
use crate::components::table::TableState;
use crate::db::load_async;

const TABLESPACE_HEADERS: &[&str] = &[
    "Name",
    "Used MB",
    "Free MB",
    "Total MB",
    "Max Total MB",
    "Used %",
];
#[derive(Debug, Clone)]
pub enum Message {
    Load,
    Loaded(TableResult),
}

#[derive(Default, Debug, Clone)]
pub struct TablespaceTable {
    pub state: TableState,
}

impl TablespaceTable {
    pub fn update(&mut self,message: Message) -> Task<Message> {
        match message {
            Message::Load => {
                self.state.begin_load();
                Task::perform(load_async(fetch_tablespace_data), Message::Loaded)
            }
            Message::Loaded(result) => {
                self.state.apply_loaded(result);
                Task::none()
            }
        }
    }

    pub fn view(self: &'_ Self) -> Element<'_, Message> { 
        let on_click: Option<fn(usize, String) -> Message> = None;
        self.state.view(TABLESPACE_HEADERS, on_click).into()
    }
}
