use iced::widget::{column, pick_list, row, text};
use iced::Alignment::Center;
use iced::{Length, Theme};

use iced_awkbot::helpers::rect_fill::{RectFill, RectFillColors};

fn main() -> iced::Result {
    iced::application("Example", Example::update, Example::view)
        .theme(Example::theme)
        .run()
}

#[derive(Clone, Debug)]
enum Message {
    ThemeChanged(Theme),
    ColorChanged(RectFillColors),
}

#[derive(Default)]
struct Example {
    theme: Theme,
    color_schema: RectFillColors,
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
            Message::ColorChanged(color) => {
                self.color_schema = color;
            }
        }
    }

    fn view(&self) -> iced::Element<self::Message> {
        let sidebar = RectFill::new()
            .border_radius(10)
            .width(Length::FillPortion(1))
            .height(Length::Fill)
            .color_schema(self.color_schema.clone());

        row![
            sidebar,
            column![
                row![
                    text("Theme: "),
                    pick_list(Theme::ALL, Some(&self.theme), Message::ThemeChanged)
                ]
                .width(Length::FillPortion(2)),
                row![
                    text("Color Scheme:"),
                    pick_list(
                        RectFillColors::ALL,
                        Some(&self.color_schema),
                        Message::ColorChanged
                    )
                ]
                .width(Length::FillPortion(2)),
            ]
        ]
        .width(Length::Fill)
        .align_y(Center)
        .into()
    }
}
