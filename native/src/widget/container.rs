//! Decorate content and apply alignment.
use std::hash::Hash;

use crate::alignment::{self, Alignment};
use crate::event::{self, Event};
use crate::layout;
use crate::overlay;
use crate::renderer::{self, Renderer};
use crate::{
    Clipboard, Element, Hasher, Layout, Length, Padding, Point, Rectangle,
    Widget,
};

use std::u32;

pub use iced_style::container::{Style, StyleSheet};

/// An element decorating some content.
///
/// It is normally used for alignment purposes.
#[allow(missing_debug_implementations)]
pub struct Container<'a, Message> {
    padding: Padding,
    width: Length,
    height: Length,
    max_width: u32,
    max_height: u32,
    horizontal_alignment: alignment::Horizontal,
    vertical_alignment: alignment::Vertical,
    style: &'a dyn StyleSheet,
    content: Element<'a, Message>,
}

impl<'a, Message> Container<'a, Message> {
    /// Creates an empty [`Container`].
    pub fn new<T>(content: T) -> Self
    where
        T: Into<Element<'a, Message>>,
    {
        Container {
            padding: Padding::ZERO,
            width: Length::Shrink,
            height: Length::Shrink,
            max_width: u32::MAX,
            max_height: u32::MAX,
            horizontal_alignment: alignment::Horizontal::Left,
            vertical_alignment: alignment::Vertical::Top,
            style: Renderer::Style::default(),
            content: content.into(),
        }
    }

    /// Sets the [`Padding`] of the [`Container`].
    pub fn padding<P: Into<Padding>>(mut self, padding: P) -> Self {
        self.padding = padding.into();
        self
    }

    /// Sets the width of the [`Container`].
    pub fn width(mut self, width: Length) -> Self {
        self.width = width;
        self
    }

    /// Sets the height of the [`Container`].
    pub fn height(mut self, height: Length) -> Self {
        self.height = height;
        self
    }

    /// Sets the maximum width of the [`Container`].
    pub fn max_width(mut self, max_width: u32) -> Self {
        self.max_width = max_width;
        self
    }

    /// Sets the maximum height of the [`Container`] in pixels.
    pub fn max_height(mut self, max_height: u32) -> Self {
        self.max_height = max_height;
        self
    }

    /// Sets the content alignment for the horizontal axis of the [`Container`].
    pub fn align_x(mut self, alignment: alignment::Horizontal) -> Self {
        self.horizontal_alignment = alignment;
        self
    }

    /// Sets the content alignment for the vertical axis of the [`Container`].
    pub fn align_y(mut self, alignment: alignment::Vertical) -> Self {
        self.vertical_alignment = alignment;
        self
    }

    /// Centers the contents in the horizontal axis of the [`Container`].
    pub fn center_x(mut self) -> Self {
        self.horizontal_alignment = alignment::Horizontal::Center;
        self
    }

    /// Centers the contents in the vertical axis of the [`Container`].
    pub fn center_y(mut self) -> Self {
        self.vertical_alignment = alignment::Vertical::Center;
        self
    }

    /// Sets the style of the [`Container`].
    pub fn style<'b>(mut self, style: impl Into<&'b dyn StyleSheet>) -> Self
    where
        'b: 'a,
    {
        self.style = style.into();
        self
    }
}

impl<'a, Message> Widget<Message> for Container<'a, Message> {
    fn width(&self) -> Length {
        self.width
    }

    fn height(&self) -> Length {
        self.height
    }

    fn layout(
        &self,
        renderer: &dyn Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let limits = limits
            .loose()
            .max_width(self.max_width)
            .max_height(self.max_height)
            .width(self.width)
            .height(self.height)
            .pad(self.padding);

        let mut content = self.content.layout(renderer, &limits.loose());
        let size = limits.resolve(content.size());

        content.move_to(Point::new(
            self.padding.left.into(),
            self.padding.top.into(),
        ));
        content.align(
            Alignment::from(self.horizontal_alignment),
            Alignment::from(self.vertical_alignment),
            size,
        );

        layout::Node::with_children(size.pad(self.padding), vec![content])
    }

    fn on_event(
        &mut self,
        event: Event,
        layout: Layout<'_>,
        cursor_position: Point,
        renderer: &dyn Renderer,
        clipboard: &mut dyn Clipboard,
        messages: &mut Vec<Message>,
    ) -> event::Status {
        self.content.widget.on_event(
            event,
            layout.children().next().unwrap(),
            cursor_position,
            renderer,
            clipboard,
            messages,
        )
    }

    fn draw(
        &self,
        renderer: &mut dyn Renderer,
        defaults: &renderer::Defaults,
        layout: Layout<'_>,
        cursor_position: Point,
        viewport: &Rectangle,
    ) {
        renderer.draw(
            defaults,
            layout.bounds(),
            cursor_position,
            viewport,
            &self.style,
            &self.content,
            layout.children().next().unwrap(),
        );
    }

    fn hash_layout(&self, state: &mut Hasher) {
        struct Marker;
        std::any::TypeId::of::<Marker>().hash(state);

        self.padding.hash(state);
        self.width.hash(state);
        self.height.hash(state);
        self.max_width.hash(state);
        self.max_height.hash(state);

        self.content.hash_layout(state);
    }

    fn overlay(
        &mut self,
        layout: Layout<'_>,
    ) -> Option<overlay::Element<'_, Message>> {
        self.content.overlay(layout.children().next().unwrap())
    }
}

impl<'a, Message> From<Container<'a, Message>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(column: Container<'a, Message>) -> Element<'a, Message> {
        Element::new(column)
    }
}
