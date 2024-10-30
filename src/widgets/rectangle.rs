use iced::advanced::layout::{self, Layout};
use iced::advanced::renderer;
use iced::advanced::widget::{self, Widget};
use iced::mouse;
use iced::{border, Theme};
use iced::{Color, Element, Length, Size};

use crate::ColorPattern;

#[derive(Debug)]
pub struct Rectangle<'a, Theme>
where
    Theme: Catalog,
{
    width: Length,
    height: Length,
    border_radios: f32,
    color_schema: ColorPattern,
    class: Theme::Class<'a>,
}

impl<'a, Theme> Rectangle<'a, Theme>
where
    Theme: Catalog,
{
    pub fn color_schema(mut self, color_schema: ColorPattern) -> Self {
        self.color_schema = color_schema;
        self
    }

    pub fn border_radius(mut self, radius: usize) -> Self {
        self.border_radios = radius as f32;
        self
    }

    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }

    pub fn new() -> Self {
        Rectangle::default()
    }

    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }
}

impl<'a, Theme> Default for Rectangle<'a, Theme>
where
    Theme: Catalog,
{
    fn default() -> Self {
        Rectangle {
            width: Length::Shrink,
            height: Length::Fill,
            border_radios: 0.,
            color_schema: ColorPattern::default(),
            class: Theme::default(),
        }
    }
}

impl<'a, Message, Theme, Renderer> From<Rectangle<'a, Theme>>
    for Element<'a, Message, Theme, Renderer>
where
    Theme: Catalog + 'a,
    Renderer: renderer::Renderer,
{
    fn from(obj: Rectangle<'a, Theme>) -> Self {
        Self::new(obj)
    }
}

impl<'a, Message, Theme, Renderer> Widget<Message, Theme, Renderer> for Rectangle<'a, Theme>
where
    Theme: Catalog,
    Renderer: renderer::Renderer,
{
    fn size(&self) -> Size<Length> {
        Size {
            width: self.width,
            height: self.height,
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
        let style = theme.style(&self.class, self.color_schema.clone());

        renderer.fill_quad(
            renderer::Quad {
                bounds: layout.bounds(),
                border: border::rounded(self.border_radios),
                ..renderer::Quad::default()
            },
            style.color,
        );
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Style {
    pub color: Color,
}

pub trait Catalog {
    type Class<'a>;
    fn default<'a>() -> Self::Class<'a>;
    fn style(&self, item: &Self::Class<'_>, cs: ColorPattern) -> Style;
}

pub type StyleFn<'a, Theme> = Box<dyn Fn(&Theme, ColorPattern) -> Style + 'a>;

impl Catalog for Theme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(load_color)
    }

    fn style(&self, class: &Self::Class<'_>, cs: ColorPattern) -> Style {
        class(self, cs)
    }
}

pub fn load_color(theme: &Theme, cs: ColorPattern) -> Style {
    let palette = theme.extended_palette();
    let color = match cs {
        ColorPattern::BackgroundBase => palette.background.base,
        ColorPattern::BackgroundStrong => palette.background.strong,
        ColorPattern::BackgroundWeak => palette.background.weak,
        ColorPattern::DangerBase => palette.danger.base,
        ColorPattern::DangerStrong => palette.danger.strong,
        ColorPattern::DangerWeak => palette.danger.weak,
        ColorPattern::PrimaryBase => palette.primary.base,
        ColorPattern::PrimaryStrong => palette.primary.strong,
        ColorPattern::PrimaryWeak => palette.primary.weak,
        ColorPattern::SecondaryBase => palette.secondary.base,
        ColorPattern::SecondaryStrong => palette.secondary.strong,
        ColorPattern::SecondaryWeak => palette.secondary.weak,
        ColorPattern::SuccessBase => palette.success.base,
        ColorPattern::SuccessStrong => palette.success.strong,
        ColorPattern::SuccessWeak => palette.success.weak,
    };

    Style { color: color.color }
}
