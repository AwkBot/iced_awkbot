use iced::{
    mouse,
    widget::canvas::{Frame, Geometry, Program},
    Point, Rectangle, Renderer, Theme,
};

#[derive(Debug, Default)]
pub struct RectFill {}

impl<Message> Program<Message> for RectFill {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<Geometry> {
        let palette = theme.extended_palette();
        let color = palette.secondary.base.color;

        let mut frame = Frame::new(renderer, bounds.size());
        frame.fill_rectangle(Point::ORIGIN, bounds.size(), color);

        vec![frame.into_geometry()]
    }
}

impl RectFill {
    pub fn new() -> Self {
        RectFill::default()
    }
}
