use iced::{Element, Task};

use crate::db::queries::{fetch_filesystem_data};
use crate::components::table::TableState;
use crate::db::load_async;

const FILESYSTEM_HEADERS: &[&str] = &[
    "Filesystem",
    "Size",
    "Used",
    "Available",
    "Use (%)",
    "Mounted",
];

#[derive(Debug, Clone)]
pub enum Message {
    Load,
    Loaded(Result<Vec<Vec<String>>, String>),
}

#[derive(Default, Debug, Clone)]
pub struct FilesystemTable {
    pub state: TableState,
}

impl FilesystemTable {
    pub fn update(&mut self,message: Message) -> Task<Message> {
        match message {
            Message::Load => {
                self.state.begin_load();
                Task::perform(load_async(fetch_filesystem_data), Message::Loaded)
            }
            Message::Loaded(result) => {
                self.state.apply_loaded(result);
                Task::none()
            }
        }
    }

    pub fn view(self: &'_ Self) -> Element<'_, Message> { 
        self.state.view::<Message>(FILESYSTEM_HEADERS)
    }
}
