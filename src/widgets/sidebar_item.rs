use iced::advanced::layout::{self, Layout};
use iced::advanced::renderer;
use iced::advanced::widget::{self, Tree, Widget};
use iced::{border, padding, Color, Element, Length, Size, Theme};
use iced::{mouse, Padding};

use crate::common::defaults::DEFAULT_PADDING;

#[allow(missing_debug_implementations)]
pub struct SideBarItem<'a, Message, Theme = iced::Theme, Renderer = iced::Renderer>
where
    Renderer: iced::advanced::Renderer,
    Theme: Catalog,
{
    width: Length,
    height: Length,
    border_radios: f32,
    // color_schema: ColorPattern,
    class: Theme::Class<'a>,
    text: Element<'a, Message, Theme, Renderer>,
    padding: Padding,
}

impl<'a, Message, Theme, Renderer> SideBarItem<'a, Message, Theme, Renderer>
where
    Renderer: iced::advanced::Renderer,
    Theme: Catalog,
{
    pub fn border_radius(mut self, radius: usize) -> Self {
        self.border_radios = radius as f32;
        self
    }

    pub fn class(mut self, class: impl Into<Theme::Class<'a>>) -> Self {
        self.class = class.into();
        self
    }

    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }

    pub fn new(label: impl Into<Element<'a, Message, Theme, Renderer>>) -> Self {
        Self {
            width: Length::Shrink,
            height: Length::Fill,
            border_radios: 0.,
            // color_schema: ColorPattern::default(),
            class: Theme::default(),
            text: label.into(),
            padding: DEFAULT_PADDING,
        }
    }

    pub fn padding(mut self, padding: Padding) -> Self {
        self.padding = padding;
        self
    }

    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }
}

impl<'a, Message: 'a, Theme, Renderer> From<SideBarItem<'a, Message, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Theme: Catalog + 'a,
    Renderer: iced::advanced::Renderer + 'a,
{
    fn from(obj: SideBarItem<'a, Message, Theme, Renderer>) -> Self {
        Self::new(obj)
    }
}

impl<'a, Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for SideBarItem<'a, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer,
    Theme: Catalog,
{
    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(&self.text)]
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(std::slice::from_ref(&self.text));
    }

    fn size(&self) -> Size<Length> {
        Size {
            width: self.width,
            height: Length::Shrink,
        }
    }

    fn layout(
        &self,
        tree: &mut widget::Tree,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        // layout::atomic(limits, self.width, Length::Fill)
        layout::padded(limits, self.width, self.height, self.padding, |limits| {
            self.text
                .as_widget()
                .layout(&mut tree.children[0], renderer, limits)
        })
    }

    fn draw(
        &self,
        tree: &widget::Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &iced::Rectangle,
    ) {
        // let status = Status::Selected(1);
        // let style = theme.style(&self.class, status);

        renderer.fill_quad(
            renderer::Quad {
                bounds: layout.bounds(),
                border: border::rounded(self.border_radios),
                ..renderer::Quad::default()
            },
            Color::BLACK,
        );

        let content_layout = layout.children().next().unwrap();

        self.text.as_widget().draw(
            &tree.children[0],
            renderer,
            theme,
            &renderer::Style {
                text_color: style.text_color,
            },
            content_layout,
            cursor,
            viewport,
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
