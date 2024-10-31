use iced::advanced::layout::{self, Layout};
use iced::advanced::renderer;
use iced::advanced::widget::{self, Widget};
use iced::mouse;
use iced::{border, Theme};
use iced::{Color, Element, Length, Size};

#[derive(Debug)]
pub struct SideBar<'a, Theme>
where
    Theme: Catalog,
{
    width: Length,
    height: Length,
    border_radios: f32,
    // color_schema: ColorPattern,
    class: Theme::Class<'a>,
}

impl<'a, Theme> SideBar<'a, Theme>
where
    Theme: Catalog,
{
    pub fn border_radius(mut self, radius: usize) -> Self {
        self.border_radios = radius as f32;
        self
    }

    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }

    pub fn new() -> Self {
        SideBar::default()
    }

    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }
}

impl<'a, Theme> Default for SideBar<'a, Theme>
where
    Theme: Catalog,
{
    fn default() -> Self {
        SideBar {
            width: Length::Shrink,
            height: Length::Fill,
            border_radios: 0.,
            // color_schema: ColorPattern::default(),
            class: Theme::default(),
        }
    }
}

impl<'a, Message, Theme, Renderer> From<SideBar<'a, Theme>>
    for Element<'a, Message, Theme, Renderer>
where
    Theme: Catalog + 'a,
    Renderer: renderer::Renderer,
{
    fn from(obj: SideBar<'a, Theme>) -> Self {
        Self::new(obj)
    }
}

impl<'a, Message, Theme, Renderer> Widget<Message, Theme, Renderer> for SideBar<'a, Theme>
where
    Theme: Catalog,
    Renderer: renderer::Renderer,
{
    fn size(&self) -> Size<Length> {
        Size {
            width: self.width,
            height: Length::Shrink,
        }
    }

    fn layout(
        &self,
        _tree: &mut widget::Tree,
        _renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        layout::atomic(limits, self.width, Length::Fill)
    }

    fn draw(
        &self,
        _state: &widget::Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        _cursor: mouse::Cursor,
        _viewport: &iced::Rectangle,
    ) {
        let status = Status::Selected(1);
        let style = theme.style(&self.class, status);

        renderer.fill_quad(
            renderer::Quad {
                bounds: layout.bounds(),
                border: border::rounded(self.border_radios),
                ..renderer::Quad::default()
            },
            Color::BLACK,
        );
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Status {
    Selected(usize),
    None,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Style {
    pub selected: usize,
}

pub trait Catalog {
    type Class<'a>;
    fn default<'a>() -> Self::Class<'a>;
    fn style(&self, item: &Self::Class<'_>, status: Status) -> Style;
}

pub type StyleFn<'a, Theme> = Box<dyn Fn(&Theme, Status) -> Style + 'a>;

impl Catalog for Theme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(load_color)
    }

    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style {
        class(self, status)
    }
}

pub fn load_color(_theme: &Theme, status: Status) -> Style {
    let selected = match status {
        Status::Selected(v) => v,
        Status::None => 0,
    };
    Style { selected }
}
