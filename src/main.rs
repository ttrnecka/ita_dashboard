use iced::{
    executor, Application, Command, Element,
    Settings, Theme, widget::{column, row, button, text, container},
    Alignment, Length,
};

mod graph;

#[derive(Debug, Clone)]
pub enum Message {
    MenuSelected(MenuItem),
}

#[derive(Debug, Clone)]
pub enum MenuItem {
    Dashboard,
    Reports,
    Graph1,
}

struct MainApp {
    selected: MenuItem,
}

impl Application for MainApp {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Self::Message>) {
        (
            Self {
                selected: MenuItem::Dashboard,
            },
            Command::none(),
        )
    }

    fn theme(&self) -> Theme {
        Theme::TokyoNightStorm
    }

    fn title(&self) -> String {
        "Iced Dashboard".into()
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::MenuSelected(item) => {
                self.selected = item;
            }
        }
        Command::none()
    }

    fn view(&'_ self) -> Element<'_, Self::Message> {
        let menu = column![
            button("Dashboard")
                .width(Length::Fill)
                .on_press(Message::MenuSelected(MenuItem::Dashboard)),
            button("Reports")
                .width(Length::Fill)
                .on_press(Message::MenuSelected(MenuItem::Reports)),
            button("Temp")
                .width(Length::Fill)
                .on_press(Message::MenuSelected(MenuItem::Graph1)),
        ]
        .spacing(2)
        .padding(5)
        .width(Length::Fixed(150.0));

        // RIGHT SIDE READING PANE CONTENT
        let content: Element<_> = match self.selected {
            MenuItem::Dashboard => {
                text("Dashboard content here").into()
            }
            MenuItem::Reports => {
                text("Reports view").into()
            }
            MenuItem::Graph1 => {
                // text("Grapth view").into()
                graph::view(vec![1.0, 3.0, 2.0, 5.0, 4.0]).into()
            }
        };

        let reading_pane = container(content)
            .padding(20)
            .width(Length::Fill);

        // LAYOUT: menu left, content right
        row![menu, reading_pane]
            .align_items(Alignment::Start)
            .into()
    }
}

fn main() -> iced::Result {
    MainApp::run(Settings::default())
}
