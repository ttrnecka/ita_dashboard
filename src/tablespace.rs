use iced::{Element, Task };

use crate::db::queries::{fetch_tablespace_data};
use crate::components::table::TableState;

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
    Loaded(Result<Vec<Vec<String>>, String>),
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
                Task::perform(load_async(), Message::Loaded)
            }
            Message::Loaded(result) => {
                self.state.apply_loaded(result);
                Task::none()
            }
        }
    }

    pub fn view(self: &'_ Self) -> Element<'_, Message> { 
        self.state.view::<Message>(TABLESPACE_HEADERS)
    }
}

pub async fn load_async() -> Result<Vec<Vec<String>>, String> {
    tokio::task::spawn_blocking(|| fetch_tablespace_data())
        .await
        .map_err(|e| e.to_string())?
        .map_err(|e| e.to_string())
}
