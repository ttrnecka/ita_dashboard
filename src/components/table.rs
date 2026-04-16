use std::default::Default;
use crate::db::{queries::TableResult, DbError};
use iced::widget::{row, column, scrollable, Container, text, mouse_area};
use iced::{Element, Length, Font};

/// Generic table state used by multiple table views (filesystem, tablespace, ...)
#[derive(Default, Debug, Clone)]
pub struct TableState {
    pub loading: bool,
    pub data: Option<Vec<Vec<String>>>,
    pub error: Option<DbError>,
}

impl TableState {
    /// Mark the table as loading and clear previous data/error
    pub fn begin_load(&mut self) {
        self.loading = true;
        self.data = None;
        self.error = None;
    }

    /// Apply a loaded result (success or error)
    pub fn apply_loaded(&mut self, result: TableResult) {
        self.loading = false;
        match result {
            Ok(vals) => self.data = Some(vals),
            Err(err) => self.error = Some(err),
        }
    }

    /// Render the table UI. This function is generic over the Message type because the
    /// table itself does not emit any messages; the parent wrapper controls loading
    /// and maps async Task results back to the wrapper's Message type.
    pub fn view<'a, Message, F>(&'a self, headers: &'a [&'a str], on_click: Option<F>) -> Element<'a, Message>
    where
        Message: 'a + std::clone::Clone,
        F: Fn(usize, String) -> Message + 'a,
    {
        if self.loading {
            return text("Loading data...").into();
        }

        if let Some(_data) = &self.data {
            let bold_font: Font = Font {
                weight: iced::font::Weight::Bold,
                ..Default::default()
            };

            let header_row = row(
                headers
                    .iter()
                    .map(|h| {
                        text(*h)
                            .size(16)
                            .width(Length::FillPortion(1))
                            .font(bold_font)
                            .into()
                    }),
            )
            .spacing(10);

            let body_vec: Vec<Element<Message>> = self
                .data
                .as_ref()
                .unwrap()
                .iter()
                .map(|row_data| {
                    let cells = row_data
                        .iter()
                        .enumerate()
                        .map(|(col_idx, cell)| {
                            let content = text(cell).size(12).width(Length::FillPortion(1));
                            match &on_click {
                                Some(f) => {
                                    let msg = (f)(col_idx, cell.clone());
                                    mouse_area(content)
                                        .on_press(msg)   
                                        .interaction(iced::mouse::Interaction::Pointer) 
                                        .into()
                                }
                                None => mouse_area(content).into(),
                            }
                        });
                    row(cells).spacing(10).into()
                })
                .collect();

            let content = column([
                header_row.into(),
                scrollable(column(body_vec).spacing(5)).height(Length::Fill).into(),
            ])
            .spacing(15)
            .padding(20);

            return Container::new(content).width(Length::Fill).height(Length::Fill).into();
        }

        if let Some(err) = &self.error {
            return text(err.to_string()).into();
        }

        text("No data").into()
    }
}
