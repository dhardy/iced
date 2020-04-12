//! Show toggle controls using checkboxes.
use crate::{css, Bus, Css, Element, Length, Widget};

pub use iced_style::checkbox::{Style, StyleSheet};

use dodrio::bumpalo;
use std::rc::Rc;

/// A box that can be checked.
///
/// # Example
///
/// ```
/// # use iced_web::Checkbox;
///
/// pub enum Message {
///     CheckboxToggled(bool),
/// }
///
/// let is_checked = true;
///
/// Checkbox::new(is_checked, "Toggle me!", Message::CheckboxToggled);
/// ```
///
/// ![Checkbox drawn by Coffee's renderer](https://github.com/hecrj/coffee/blob/bda9818f823dfcb8a7ad0ff4940b4d4b387b5208/images/ui/checkbox.png?raw=true)
#[allow(missing_debug_implementations)]
pub struct Checkbox<Message> {
    is_checked: bool,
    on_toggle: Rc<dyn Fn(bool) -> Message>,
    label: String,
    id: String,
    width: Length,
    style: Box<dyn StyleSheet>,
}

impl<Message> Checkbox<Message> {
    /// Creates a new [`Checkbox`].
    ///
    /// It expects:
    ///   * a boolean describing whether the [`Checkbox`] is checked or not
    ///   * the label of the [`Checkbox`]
    ///   * a function that will be called when the [`Checkbox`] is toggled. It
    ///     will receive the new state of the [`Checkbox`] and must produce a
    ///     `Message`.
    ///
    /// [`Checkbox`]: struct.Checkbox.html
    pub fn new<F>(is_checked: bool, label: impl Into<String>, f: F) -> Self
    where
        F: 'static + Fn(bool) -> Message,
    {
        Checkbox {
            is_checked,
            on_toggle: Rc::new(f),
            label: label.into(),
            id: Default::default(),
            width: Length::Shrink,
            style: Default::default(),
        }
    }

    /// Sets the width of the [`Checkbox`].
    ///
    /// [`Checkbox`]: struct.Checkbox.html
    pub fn width(mut self, width: Length) -> Self {
        self.width = width;
        self
    }

    /// Sets the style of the [`Checkbox`].
    ///
    /// [`Checkbox`]: struct.Checkbox.html
    pub fn style(mut self, style: impl Into<Box<dyn StyleSheet>>) -> Self {
        self.style = style.into();
        self
    }

    /// Sets the id of the [`Checkbox`].
    ///
    /// [`Checkbox`]: struct.Checkbox.html
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = id.into();
        self
    }
}

impl<Message> Widget<Message> for Checkbox<Message>
where
    Message: 'static,
{
    fn node<'b>(
        &self,
        bump: &'b bumpalo::Bump,
        bus: &Bus<Message>,
        style_sheet: &mut Css<'b>,
    ) -> dodrio::Node<'b> {
        use dodrio::builder::*;

        let checkbox_label =
            bumpalo::format!(in bump, "{}", self.label).into_bump_str();
        let checkbox_id =
            bumpalo::format!(in bump, "{}", self.id).into_bump_str();

        let event_bus = bus.clone();
        let on_toggle = self.on_toggle.clone();
        let is_checked = self.is_checked;

        let row_class = style_sheet.insert(bump, css::Rule::Row);

        let spacing_class = style_sheet.insert(bump, css::Rule::Spacing(5));

        label(bump)
            .attr("for", checkbox_id)
            .attr(
                "class",
                bumpalo::format!(in bump, "{} {}", row_class, spacing_class)
                    .into_bump_str(),
            )
            .attr(
                "style",
                bumpalo::format!(in bump, "width: {}; align-items: center", css::length(self.width))
                    .into_bump_str(),
            )
            .children(vec![
                // TODO: Checkbox styling
                input(bump)
                    .attr("type", "checkbox")
                    .attr("id", checkbox_id)
                    .bool_attr("checked", self.is_checked)
                    .on("click", move |_root, vdom, _event| {
                        let msg = on_toggle(!is_checked);
                        event_bus.publish(msg);

                        vdom.schedule_render();
                    })
                    .finish(),
                text(checkbox_label),
            ])
            .finish()
    }
}

impl<'a, Message> From<Checkbox<Message>> for Element<'a, Message>
where
    Message: 'static,
{
    fn from(checkbox: Checkbox<Message>) -> Element<'a, Message> {
        Element::new(checkbox)
    }
}
