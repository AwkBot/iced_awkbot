use iced::widget::{column, row, text};
use iced::Alignment::Center;
use iced::{Element, Length, Padding, Theme};
use iced_nova::ColorPattern;

fn main() -> iced::Result {
    iced::application("Example", Example::update, Example::view)
        .theme(Example::theme)
        .run()
}

#[derive(Clone, Debug)]
enum Message {
    ThemeChanged(Theme),
}

#[derive(Default)]
struct Example {
    theme: Theme,
    color_schema: iced_nova::ColorPattern,
}

impl Example {
    pub fn theme(&self) -> Theme {
        self.theme.clone()
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::ThemeChanged(theme) => {
                self.theme = theme;
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let sidebar = iced_nova::SideBarItem::new("123")
            .width(Length::FillPortion(1))
            .height(Length::Shrink)
            .padding(Padding {
                top: 10.0,
                bottom: 10.0,
                right: 50.0,
                left: 10.0,
            });

        let btn = iced::widget::Button::new("test");

        row![
            sidebar,
            btn,
            column![text("Test").width(Length::FillPortion(2))]
        ]
        .width(Length::Fill)
        .align_y(Center)
        .into()
    }
}
