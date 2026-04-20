use std::default::Default;
use iced::widget::{Container, button, column, container, scrollable, text};
use iced::{Length, Theme, Border, Color};

#[derive(Default, Debug, Clone)]
pub struct PopUp {
}

impl PopUp {
    pub fn pop_up<'a, Message>(&self, message: Message, content: String) -> Container<'a,Message> 
    where
        Message: 'a + std::clone::Clone,
    {
        let popup = container(
                scrollable(column![
                    button("Close").on_press(message),
                    text(content),
                ]
                .spacing(10))
            )
            .padding(20)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(|theme: &Theme| {
                let palette = theme.palette();
                container::Style {
                    background: Some(palette.background.into()),
                    text_color: Some(palette.text),
                    border: Border::default().color(Color::from_rgb(0.8, 0.8, 0.8)).width(5),
                    ..Default::default()
                }
            });
        return popup;
    } 
}