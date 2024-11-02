use iced::advanced::layout::{self};
use iced::advanced::widget::{self, Tree};
use iced::advanced::{renderer, Widget};
use iced::{Element, Length, Size, Theme};

use super::sidebar_item::SideBarItem;

#[allow(missing_debug_implementations)]
pub struct SideBar<'a, Message, Theme = iced::Theme, Renderer = iced::Renderer>
where
    Renderer: renderer::Renderer,
    Theme: Catalog + super::sidebar_item::Catalog,
{
    width: Length,
    height: Length,
    panels: Vec<SideBarItem<'a, Message, Theme, Renderer>>,
}

impl<'a, Message, Theme, Renderer> SideBar<'a, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer,
    Theme: Catalog + super::sidebar_item::Catalog,
{
    pub fn new<T>(label: T) -> Self
    where
        T: Into<Element<'a, Message, Theme, Renderer>>,
    {
        let painel = SideBarItem::new(label);
        let panels = vec![painel];

        Self {
            width: Length::Shrink,
            height: Length::Shrink,
            panels,
        }
    }

    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }

    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }
}

impl<'a, Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for SideBar<'a, Message, Theme, Renderer>
where
    Message: 'a + Clone,
    Renderer: 'a + renderer::Renderer,
    Theme: Catalog + super::sidebar_item::Catalog,
{
    fn children(&self) -> Vec<Tree> {
        self.panels
            .iter()
            .map(|panel| Tree {
                tag: panel.tag(),
                state: panel.state(),
                children: panel.children(),
            })
            .collect()
    }

    fn layout(
        &self,
        _tree: &mut widget::Tree,
        _renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        layout::atomic(limits, Length::Fill, Length::Fill)
    }

    fn size(&self) -> Size<Length> {
        Size {
            width: self.width,
            height: self.height,
        }
    }

    fn draw(
        &self,
        _tree: &widget::Tree,
        _renderer: &mut Renderer,
        _theme: &Theme,
        _style: &renderer::Style,
        _layout: layout::Layout<'_>,
        _cursor: iced::advanced::mouse::Cursor,
        _viewport: &iced::Rectangle,
    ) {
        let _x = 1;
    }
}

impl<'a, Message, Theme, Renderer> From<SideBar<'a, Message, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Renderer: 'a + renderer::Renderer,
    Theme: 'a + Catalog + super::sidebar_item::Catalog,
    Message: 'a + Clone,
{
    fn from(obj: SideBar<'a, Message, Theme, Renderer>) -> Self {
        Self::new(obj)
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Style {}

pub trait Catalog {
    type Class<'a>;
    fn default<'a>() -> Self::Class<'a>;
    fn style(&self, item: &Self::Class<'_>) -> Style;
}

type StyleFn<'a, Theme> = Box<dyn Fn(&Theme) -> Style + 'a>;

impl Catalog for Theme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(|_theme: &Self| Style::default())
    }

    fn style(&self, class: &Self::Class<'_>) -> Style {
        class(self)
    }
}
