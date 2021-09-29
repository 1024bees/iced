//! Provide progress feedback to your users.
use crate::layout;
use crate::renderer::{self, Renderer};
use crate::{Element, Hasher, Layout, Length, Point, Rectangle, Size, Widget};

use std::{hash::Hash, ops::RangeInclusive};

pub use iced_style::progress_bar::{Style, StyleSheet};

/// A bar that displays progress.
///
/// # Example
/// ```
/// # use iced_native::renderer::Null;
/// #
/// # pub type ProgressBar = iced_native::ProgressBar<Null>;
/// let value = 50.0;
///
/// ProgressBar::new(0.0..=100.0, value);
/// ```
///
/// ![Progress bar drawn with `iced_wgpu`](https://user-images.githubusercontent.com/18618951/71662391-a316c200-2d51-11ea-9cef-52758cab85e3.png)
#[allow(missing_debug_implementations)]
pub struct ProgressBar<'a> {
    range: RangeInclusive<f32>,
    value: f32,
    width: Length,
    height: Option<Length>,
    style: &'a dyn StyleSheet,
}

impl<'a> ProgressBar<'a> {
    /// Creates a new [`ProgressBar`].
    ///
    /// It expects:
    ///   * an inclusive range of possible values
    ///   * the current value of the [`ProgressBar`]
    pub fn new(range: RangeInclusive<f32>, value: f32) -> Self {
        ProgressBar {
            value: value.max(*range.start()).min(*range.end()),
            range,
            width: Length::Fill,
            height: None,
            style: Renderer::Style::default(),
        }
    }

    /// Sets the width of the [`ProgressBar`].
    pub fn width(mut self, width: Length) -> Self {
        self.width = width;
        self
    }

    /// Sets the height of the [`ProgressBar`].
    pub fn height(mut self, height: Length) -> Self {
        self.height = Some(height);
        self
    }

    /// Sets the style of the [`ProgressBar`].
    pub fn style<'b>(mut self, style: impl Into<&'b dyn StyleSheet>) -> Self
    where
        'b: 'a,
    {
        self.style = style.into();
        self
    }
}

impl<'a, Message> Widget<Message> for ProgressBar<'a> {
    fn width(&self) -> Length {
        self.width
    }

    fn height(&self) -> Length {
        self.height
            .unwrap_or(Length::Units(Renderer::DEFAULT_HEIGHT))
    }

    fn layout(
        &self,
        _renderer: &dyn Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let limits = limits.width(self.width).height(
            self.height
                .unwrap_or(Length::Units(Renderer::DEFAULT_HEIGHT)),
        );

        let size = limits.resolve(Size::ZERO);

        layout::Node::new(size)
    }

    fn draw(
        &self,
        renderer: &mut dyn Renderer,
        _defaults: &renderer::Defaults,
        layout: Layout<'_>,
        _cursor_position: Point,
        _viewport: &Rectangle,
    ) {
        renderer.draw(
            layout.bounds(),
            self.range.clone(),
            self.value,
            &self.style,
        )
    }

    fn hash_layout(&self, state: &mut Hasher) {
        struct Marker;
        std::any::TypeId::of::<Marker>().hash(state);

        self.width.hash(state);
        self.height.hash(state);
    }
}

impl<'a, Message> From<ProgressBar<'a>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(progress_bar: ProgressBar<'a>) -> Element<'a, Message> {
        Element::new(progress_bar)
    }
}
