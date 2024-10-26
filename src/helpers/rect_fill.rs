use iced::advanced::layout::{self, Layout};
use iced::advanced::renderer;
use iced::advanced::widget::{self, Widget};
use iced::border;
use iced::mouse;
use iced::{Color, Element, Length, Rectangle, Size};

#[derive(Debug)]
pub struct RectFill {
    width: Length,
    border_radios: f32,
}

impl<Message, Theme, Renderer> Widget<Message, Theme, Renderer> for RectFill
where
    Renderer: renderer::Renderer,
{
    fn draw(
        &self,
        _state: &widget::Tree,
        renderer: &mut Renderer,
        _theme: &Theme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        _cursor: mouse::Cursor,
        _viewport: &Rectangle,
    ) {
        renderer.fill_quad(
            renderer::Quad {
                bounds: layout.bounds(),
                border: border::rounded(self.border_radios),
                ..renderer::Quad::default()
            },
            // TODO implement theme management
            Color::BLACK,
        );
    }

    fn layout(
        &self,
        _tree: &mut widget::Tree,
        _renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let width = match self.width {
            Length::Fill => limits.max().width,
            Length::FillPortion(portion) => portion as f32, // TODO missing Length::FillPortion implementation
            Length::Shrink => 0.0,
            Length::Fixed(fixed) => fixed,
        };
        layout::Node::new(Size::new(width, limits.max().height))
    }

    fn size(&self) -> Size<Length> {
        Size {
            width: Length::Shrink,
            height: Length::Fill,
        }
    }
}

impl<'a, Message, Theme, Renderer> From<RectFill> for Element<'a, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer,
{
    fn from(obj: RectFill) -> Self {
        Self::new(obj)
    }
}

impl Default for RectFill {
    fn default() -> Self {
        RectFill {
            width: Length::Shrink,
            border_radios: 0.,
        }
    }
}

impl RectFill {
    pub fn border_radius(mut self, radius: usize) -> Self {
        self.border_radios = radius as f32;
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

pub fn rect_fill() -> RectFill {
    RectFill::new()
}
