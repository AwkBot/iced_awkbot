use iced::widget::{button, column, pick_list, row};
use iced::Alignment::Center;
use iced::{Length, Theme};

use iced_awkbot::helpers::rect_fill::RectFill;

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
}

impl Example {
    pub fn theme(&self) -> Theme {
        self.theme.clone()
    }

    fn update(&mut self, message: self::Message) {
        match message {
            Message::ThemeChanged(theme) => {
                self.theme = theme;
            }
        }
    }

    fn view(&self) -> iced::Element<self::Message> {
        let sidebar = RectFill::new()
            .border_radius(10)
            .width(Length::FillPortion(1))
            .height(Length::Fill);

        let tmp = button("oi");

        row![
            sidebar,
            column![
                tmp,
                pick_list(Theme::ALL, Some(&self.theme), Message::ThemeChanged)
                    .width(Length::FillPortion(2)),
            ]
        ]
        .width(Length::Fill)
        .align_y(Center)
        .into()
    }
}
