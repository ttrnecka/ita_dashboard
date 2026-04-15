use iced::{Element, Task, Length,  Theme, widget::{container,column, button, text, stack}};

use crate::db::queries::{fetch_session_temp_data,fetch_sqlid_data};
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
    SQLID(Result<Vec<Vec<String>>, String>),
    CellClicked(usize,String), 
    ClosePopup,
}

#[derive(Debug, Clone, Default)]
pub struct SessionTempTable {
    pub state: TableState,
    show_popup: bool,
    sqlid_data: Option<Vec<Vec<String>>>,
    sqlid_error: Option<String>,
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
                match result {
                    Ok(data) => self.sqlid_data = Some(data),
                    Err(err) => self.sqlid_error = Some(err),
                }
                self.show_popup = true;
                Task::none()
            }
            Message::CellClicked(col, txt) => {
                let sql_col = SESSION_TEMP_HEADERS.iter().position(|&h| h == "SQL ID").unwrap();
                if col == sql_col { 
                    // self.state.begin_load();
                    self.sqlid_data = None;
                    self.sqlid_error = None;
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

    fn sqlid_as_text(&self) -> String {
        if let Some(err) = &self.sqlid_error {
            return format!("Error: {}", err);
        }
        if let Some(data) = &self.sqlid_data {
            return format!("{}", data.get(0).and_then(|v| v.get(1)).map(|s| s.as_str()).unwrap_or("Unknown SQL ID"));
        }
        "No data".to_string()
    }

    pub fn view(self: &'_ Self) -> Element<'_, Message> { 
        let on_click: Option<fn(usize, String) -> Message> = Some(|i,s| Message::CellClicked(i, s));
        if self.show_popup {
            let popup = container(
                column![
                    text(self.sqlid_as_text()),
                    button("Close").on_press(Message::ClosePopup),
                ]
                .spacing(10)
            )
            .width(Length::Shrink)
            .padding(20);

            stack![
                self.state.view(SESSION_TEMP_HEADERS, on_click),
                container(popup)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    // .center(Length::Fill)
                    .style(|theme: &Theme| {
                        let palette = theme.palette();
                        container::Style {
                            background: Some(palette.background.into()),
                            text_color: Some(palette.text),
                            ..Default::default()
                        }
                    })
                    
            ]
            .into()
        } else {
            self.state.view(SESSION_TEMP_HEADERS, on_click).into()
        }
    }
}
