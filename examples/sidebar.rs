use iced::widget::{column, pick_list, row, text};
use iced::Alignment::Center;
use iced::{Element, Length, Padding, Theme};
use iced_nova::widgets::sidebar::SideBar;

fn main() -> iced::Result {
    iced::application("Example", Example::update, Example::view)
        .theme(Example::theme)
        .run()
}

#[derive(Clone, Debug)]
enum Message {
    ThemeChanged(Theme),
    ClickSideBarItem,
}

#[derive(Default)]
struct Example {
    theme: Theme,
}

impl Example {
    pub fn theme(&self) -> Theme {
        self.theme.clone()
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::ThemeChanged(theme) => self.theme = theme,
            Message::ClickSideBarItem => {
                println!("clique recebido")
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let tmp = SideBar::new();
        let sidebar = iced_nova::SideBarItem::new("123")
            .width(Length::FillPortion(1))
            .on_press(Message::ClickSideBarItem)
            .padding(Padding {
                top: 10.0,
                bottom: 10.0,
                right: 50.0,
                left: 10.0,
            });

        let thema = pick_list(Theme::ALL, Some(&self.theme), Message::ThemeChanged);

        row![
            sidebar,
            thema,
            column![text("Test").width(Length::FillPortion(2))]
        ]
        .width(Length::Fill)
        .align_y(Center)
        .into()
    }
}
