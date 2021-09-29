//! Show toggle controls using checkboxes.
use std::hash::Hash;

use crate::alignment::{self, Alignment};
use crate::event::{self, Event};
use crate::layout;
use crate::mouse;
use crate::renderer::{self, Renderer};
use crate::touch;
use crate::{
    Clipboard, Color, Element, Font, Hasher, Layout, Length, Point, Rectangle,
    Row, Text, Widget,
};

pub use iced_style::checkbox::{Style, StyleSheet};

/// A box that can be checked.
///
/// # Example
///
/// ```
/// # type Checkbox<Message> = iced_native::Checkbox<Message, iced_native::renderer::Null>;
/// #
/// pub enum Message {
///     CheckboxToggled(bool),
/// }
///
/// let is_checked = true;
///
/// Checkbox::new(is_checked, "Toggle me!", Message::CheckboxToggled);
/// ```
///
/// ![Checkbox drawn by `iced_wgpu`](https://github.com/hecrj/iced/blob/7760618fb112074bc40b148944521f312152012a/docs/images/checkbox.png?raw=true)
#[allow(missing_debug_implementations)]
pub struct Checkbox<'a, Message> {
    is_checked: bool,
    on_toggle: Box<dyn Fn(bool) -> Message>,
    label: String,
    width: Length,
    size: u16,
    spacing: u16,
    text_size: Option<u16>,
    font: Font,
    text_color: Option<Color>,
    style: &'a dyn StyleSheet,
}

impl<'a, Message> Checkbox<'a, Message> {
    /// Creates a new [`Checkbox`].
    ///
    /// It expects:
    ///   * a boolean describing whether the [`Checkbox`] is checked or not
    ///   * the label of the [`Checkbox`]
    ///   * a function that will be called when the [`Checkbox`] is toggled. It
    ///     will receive the new state of the [`Checkbox`] and must produce a
    ///     `Message`.
    pub fn new<F>(is_checked: bool, label: impl Into<String>, f: F) -> Self
    where
        F: 'static + Fn(bool) -> Message,
    {
        Checkbox {
            is_checked,
            on_toggle: Box::new(f),
            label: label.into(),
            width: Length::Shrink,
            size: 20,
            spacing: 15,
            text_size: None,
            font: Font::default(),
            text_color: None,
            style: Renderer::Style::default(),
        }
    }

    /// Sets the size of the [`Checkbox`].
    pub fn size(mut self, size: u16) -> Self {
        self.size = size;
        self
    }

    /// Sets the width of the [`Checkbox`].
    pub fn width(mut self, width: Length) -> Self {
        self.width = width;
        self
    }

    /// Sets the spacing between the [`Checkbox`] and the text.
    pub fn spacing(mut self, spacing: u16) -> Self {
        self.spacing = spacing;
        self
    }

    /// Sets the text size of the [`Checkbox`].
    pub fn text_size(mut self, text_size: u16) -> Self {
        self.text_size = Some(text_size);
        self
    }

    /// Sets the [`Font`] of the text of the [`Checkbox`].
    ///
    /// [`Font`]: crate::widget::text::Renderer::Font
    pub fn font(mut self, font: Font) -> Self {
        self.font = font;
        self
    }

    /// Sets the text color of the [`Checkbox`] button.
    pub fn text_color(mut self, color: Color) -> Self {
        self.text_color = Some(color);
        self
    }

    /// Sets the style of the [`Checkbox`].
    pub fn style<'b>(mut self, style: &'b dyn StyleSheet) -> Self
    where
        'b: 'a,
    {
        self.style = style.into();
        self
    }
}

impl<'a, Message> Widget<Message> for Checkbox<'a, Message> {
    fn width(&self) -> Length {
        self.width
    }

    fn height(&self) -> Length {
        Length::Shrink
    }

    fn layout(
        &self,
        renderer: &dyn Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        Row::<()>::new()
            .width(self.width)
            .spacing(self.spacing)
            .align_items(Alignment::Center)
            .push(
                Row::new()
                    .width(Length::Units(self.size))
                    .height(Length::Units(self.size)),
            )
            .push(
                Text::new(&self.label)
                    .font(self.font)
                    .width(self.width)
                    .size(self.text_size.unwrap_or(renderer.default_size())),
            )
            .layout(renderer, limits)
    }

    fn on_event(
        &mut self,
        event: Event,
        layout: Layout<'_>,
        cursor_position: Point,
        _renderer: &dyn Renderer,
        _clipboard: &mut dyn Clipboard,
        messages: &mut Vec<Message>,
    ) -> event::Status {
        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerPressed { .. }) => {
                let mouse_over = layout.bounds().contains(cursor_position);

                if mouse_over {
                    messages.push((self.on_toggle)(!self.is_checked));

                    return event::Status::Captured;
                }
            }
            _ => {}
        }

        event::Status::Ignored
    }

    fn draw(
        &self,
        renderer: &mut dyn Renderer,
        defaults: &renderer::Defaults,
        layout: Layout<'_>,
        cursor_position: Point,
        _viewport: &Rectangle,
    ) {
        let bounds = layout.bounds();
        let mut children = layout.children();

        let checkbox_layout = children.next().unwrap();
        let label_layout = children.next().unwrap();
        let checkbox_bounds = checkbox_layout.bounds();

        let label = renderer.fill_text(
            renderer,
            defaults,
            label_layout.bounds(),
            &self.label,
            self.text_size.unwrap_or(renderer.default_size()),
            self.font,
            self.text_color,
            alignment::Horizontal::Left,
            alignment::Vertical::Center,
        );

        let is_mouse_over = bounds.contains(cursor_position);

        self::Renderer::draw(
            renderer,
            checkbox_bounds,
            self.is_checked,
            is_mouse_over,
            label,
            &self.style,
        )
    }

    fn hash_layout(&self, state: &mut Hasher) {
        struct Marker;
        std::any::TypeId::of::<Marker>().hash(state);

        self.label.hash(state);
    }
}

impl<'a, Message> From<Checkbox<'a, Message>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(checkbox: Checkbox<'a, Message>) -> Element<'a, Message> {
        Element::new(checkbox)
    }
}
