//! Distribute content vertically.
use std::hash::Hash;

use crate::layout;
use crate::renderer::{self, Renderer};
use crate::{Element, Hasher, Layout, Length, Point, Rectangle, Size, Widget};

/// An amount of empty space.
///
/// It can be useful if you want to fill some space with nothing.
#[derive(Debug)]
pub struct Space {
    width: Length,
    height: Length,
}

impl Space {
    /// Creates an amount of empty [`Space`] with the given width and height.
    pub fn new(width: Length, height: Length) -> Self {
        Space { width, height }
    }

    /// Creates an amount of horizontal [`Space`].
    pub fn with_width(width: Length) -> Self {
        Space {
            width,
            height: Length::Shrink,
        }
    }

    /// Creates an amount of vertical [`Space`].
    pub fn with_height(height: Length) -> Self {
        Space {
            width: Length::Shrink,
            height,
        }
    }
}

impl<Message> Widget<Message> for Space {
    fn width(&self) -> Length {
        self.width
    }

    fn height(&self) -> Length {
        self.height
    }

    fn layout(
        &self,
        _renderer: &dyn Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let limits = limits.width(self.width).height(self.height);

        layout::Node::new(limits.resolve(Size::ZERO))
    }

    fn draw(
        &self,
        renderer: &mut dyn Renderer,
        _defaults: &renderer::Defaults,
        layout: Layout<'_>,
        _cursor_position: Point,
        _viewport: &Rectangle,
    ) {
        renderer.draw(layout.bounds())
    }

    fn hash_layout(&self, state: &mut Hasher) {
        std::any::TypeId::of::<Space>().hash(state);

        self.width.hash(state);
        self.height.hash(state);
    }
}

impl<'a, Message> From<Space> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(space: Space) -> Element<'a, Message> {
        Element::new(space)
    }
}
