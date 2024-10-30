use std::fmt;

use iced::advanced::layout::{self, Layout};
use iced::advanced::renderer;
use iced::advanced::widget::{self, Widget};
use iced::mouse;
use iced::{border, Theme};
use iced::{Color, Element, Length, Rectangle, Size};

#[derive(Debug)]
pub struct RectFill<'a, Theme>
where
    Theme: Catalog,
{
    width: Length,
    height: Length,
    border_radios: f32,
    color_schema: RectFillColors,
    class: Theme::Class<'a>,
}

impl<'a, Theme> RectFill<'a, Theme>
where
    Theme: Catalog,
{
    pub fn color_schema(mut self, color_schema: RectFillColors) -> Self {
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
        RectFill::default()
    }

    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }
}

impl<'a, Theme> Default for RectFill<'a, Theme>
where
    Theme: Catalog,
{
    fn default() -> Self {
        RectFill {
            width: Length::Shrink,
            height: Length::Fill,
            border_radios: 0.,
            color_schema: RectFillColors::default(),
            class: Theme::default(),
        }
    }
}

impl<'a, Message, Theme, Renderer> From<RectFill<'a, Theme>>
    for Element<'a, Message, Theme, Renderer>
where
    Theme: Catalog + 'a,
    Renderer: renderer::Renderer,
{
    fn from(obj: RectFill<'a, Theme>) -> Self {
        Self::new(obj)
    }
}

impl<'a, Message, Theme, Renderer> Widget<Message, Theme, Renderer> for RectFill<'a, Theme>
where
    Theme: Catalog,
    Renderer: renderer::Renderer,
{
    fn draw(
        &self,
        _state: &widget::Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        _cursor: mouse::Cursor,
        _viewport: &Rectangle,
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

    fn layout(
        &self,
        _tree: &mut widget::Tree,
        _renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        layout::atomic(limits, self.width, Length::Fill)
    }

    fn size(&self) -> Size<Length> {
        Size {
            width: self.width,
            height: self.height,
        }
    }
}

//-------------------------------

#[derive(Debug, Default, Clone, PartialEq)]
pub enum RectFillColors {
    #[default]
    BackgroundBase,
    BackgroundStrong,
    BackgroundWeak,
    DangerBase,
    DangerStrong,
    DangerWeak,
    PrimaryBase,
    PrimaryStrong,
    PrimaryWeak,
    SecondaryBase,
    SecondaryStrong,
    SecondaryWeak,
    SuccessBase,
    SuccessStrong,
    SuccessWeak,
}

impl RectFillColors {
    pub const ALL: &'static [Self] = &[
        Self::BackgroundBase,
        Self::BackgroundStrong,
        Self::BackgroundWeak,
        Self::DangerBase,
        Self::DangerStrong,
        Self::DangerWeak,
        Self::PrimaryBase,
        Self::PrimaryStrong,
        Self::PrimaryWeak,
        Self::SecondaryBase,
        Self::SecondaryStrong,
        Self::SecondaryWeak,
        Self::SuccessBase,
        Self::SuccessStrong,
        Self::SuccessWeak,
    ];
}

impl fmt::Display for RectFillColors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BackgroundBase => write!(f, "BackgroundBase"),
            Self::BackgroundStrong => write!(f, "BackgroundStrong"),
            Self::BackgroundWeak => write!(f, "BackgroundWeak"),
            Self::DangerBase => write!(f, "DangerBase"),
            Self::DangerStrong => write!(f, "DangerStrong"),
            Self::DangerWeak => write!(f, "DangerWeak"),
            Self::PrimaryBase => write!(f, "PrimaryBase"),
            Self::PrimaryStrong => write!(f, "PrimaryStrong"),
            Self::PrimaryWeak => write!(f, "PrimaryWeak"),
            Self::SecondaryBase => write!(f, "SecondaryBase"),
            Self::SecondaryStrong => write!(f, "SecondaryStrong"),
            Self::SecondaryWeak => write!(f, "SecondaryWeak"),
            Self::SuccessBase => write!(f, "SuccessBase"),
            Self::SuccessStrong => write!(f, "SuccessStrong"),
            Self::SuccessWeak => write!(f, "SuccessWeak"),
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Style {
    pub color: Color,
}

pub trait Catalog {
    type Class<'a>;
    fn default<'a>() -> Self::Class<'a>;
    fn style(&self, item: &Self::Class<'_>, cs: RectFillColors) -> Style;
}

pub type StyleFn<'a, Theme> = Box<dyn Fn(&Theme, RectFillColors) -> Style + 'a>;

impl Catalog for Theme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(load_color)
    }

    fn style(&self, class: &Self::Class<'_>, cs: RectFillColors) -> Style {
        class(self, cs)
    }
}

pub fn load_color(theme: &Theme, cs: RectFillColors) -> Style {
    let pallete = theme.extended_palette();
    let color = match cs {
        RectFillColors::BackgroundBase => pallete.background.base,
        RectFillColors::BackgroundStrong => pallete.background.strong,
        RectFillColors::BackgroundWeak => pallete.background.weak,
        RectFillColors::DangerBase => pallete.danger.base,
        RectFillColors::DangerStrong => pallete.danger.strong,
        RectFillColors::DangerWeak => pallete.danger.weak,
        RectFillColors::PrimaryBase => pallete.primary.base,
        RectFillColors::PrimaryStrong => pallete.primary.strong,
        RectFillColors::PrimaryWeak => pallete.primary.weak,
        RectFillColors::SecondaryBase => pallete.secondary.base,
        RectFillColors::SecondaryStrong => pallete.secondary.strong,
        RectFillColors::SecondaryWeak => pallete.secondary.weak,
        RectFillColors::SuccessBase => pallete.success.base,
        RectFillColors::SuccessStrong => pallete.success.strong,
        RectFillColors::SuccessWeak => pallete.success.weak,
    };

    Style { color: color.color }
}
