use iced::widget::{text};
use iced::{Element, Length, Task, widget::{row, column, Container, scrollable},Font};

use crate::db::queries::{fetch_tablespace_data};

#[derive(Debug, Clone)]
pub enum Message {
    Load,
    Loaded(Result<Vec<Vec<String>>, String>),
}
#[derive(Default, Debug, Clone)]
pub struct TablespaceTable {
    pub loading: bool,
    pub data: Option<Vec<Vec<String>>>,
    pub error: Option<String>,
}

impl TablespaceTable {
    pub fn update(&mut self,message: Message) -> Task<Message> {
        match message {
            Message::Load => {
                self.loading = true;
                self.data = None;
                self.error = None;
                Task::perform(load_async(), Message::Loaded)
            }
            Message::Loaded(result) => {
                self.loading = false;

                match result {
                    Ok(vals) => {
                        self.data = Some(vals)
                    }
                    Err(err) => {
                        eprintln!("Oracle error: {}", err);
                        self.error = Some(err)
                    },
                }
                Task::none()
            }
        }
    }

    pub fn view(self: &'_ Self) -> Element<'_, Message> { 
        if self.loading {
            text("Loading data...").into()
        } else if let Some(_data) = &self.data {
            let bold_font: Font = Font {
                weight: iced::font::Weight::Bold,
                ..Default::default()
            };
            let header_row = row(
                ["Name","Used MB","Free MB","Total MB","Max Total MB","Used %"]
                    .iter()
                    .map(|h| {
                        text(*h)
                            // .bold()
                            .size(16)
                            .width(Length::FillPortion(1))
                            .font(bold_font)
                            .into()
                    })
            )
            .spacing(10);
            let body_rows = self.data.as_ref().unwrap().iter().map(|row_data| {
                row(
                    row_data.iter().map(|cell| {
                        text(cell)
                            .size(12)
                            .width(Length::FillPortion(1))
                            .into()
                    })
                )
                .spacing(10)
                .into()
            });

            let content = column([
                header_row.into(),
                scrollable(
                    column(body_rows)
                        .spacing(5)
                )
                .height(Length::Fill)
                .into(),
            ])
            .spacing(15)
            .padding(20);

            Container::new(content)
                .width(Length::Fill)
                .height(Length::Fill)
                .into()
        } else if let Some(err) = &self.error {
            iced::widget::text(err).into()
        } else {
            iced::widget::text("No data").into()
        }
    }
}

pub async fn load_async() -> Result<Vec<Vec<String>>, String> {
    tokio::task::spawn_blocking(|| fetch_tablespace_data())
        .await
        .map_err(|e| e.to_string())?
        .map_err(|e| e.to_string())
}
