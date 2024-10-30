use iced::widget::{column, row, text};
use iced::Alignment::Center;
use iced::{Element, Length, Theme};

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
        let sidebar = iced_nova::Rectangle::new()
            .border_radius(10)
            .width(Length::FillPortion(1))
            .height(Length::Fill)
            .color_schema(self.color_schema.clone());

        row![sidebar, column![text("Test").width(Length::FillPortion(2))]]
            .width(Length::Fill)
            .align_y(Center)
            .into()
    }
}
