//! The Oxida widget set and the `ui.oxida()` builder entry point.

mod button;
mod slider;
mod text_input;
mod toggle;

pub use button::{Button, ButtonVariant};
pub use slider::Slider;
pub use text_input::TextInput;
pub use toggle::Toggle;

use egui::{Response, Ui, Widget, WidgetText};

/// Control size shared by the widgets.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Size {
    /// Compact: dense toolbars, badges.
    Sm,
    /// Default control size.
    #[default]
    Md,
    /// Large: prominent/primary controls.
    Lg,
}

/// Entry point for Oxida widgets: `ui.oxida().button("Save")`.
///
/// Created by [`OxidaUi::oxida`](crate::OxidaUi).
///
/// Two usage styles:
///
/// * Widgets that render immediately return a [`Response`]:
///   `ui.oxida().button("Save").clicked()`
/// * Widgets with extra configuration return a builder — finish with `.ui(ui)`:
///   `ui.oxida().slider(&mut v, 0.0..=1.0).label("Speed").ui(ui)`
pub struct Oxida<'a> {
    pub(crate) ui: &'a mut Ui,
}

impl<'a> Oxida<'a> {
    /// Wrap `ui` for widget construction.
    pub fn new(ui: &'a mut Ui) -> Self {
        Self { ui }
    }

    /// Primary button (accent background) — the main action of a view.
    pub fn button(self, label: impl Into<WidgetText>) -> Response {
        Button::new(label).variant(ButtonVariant::Primary).ui(self.ui)
    }

    /// Secondary button (surface background + border).
    pub fn secondary_button(self, label: impl Into<WidgetText>) -> Response {
        Button::new(label).variant(ButtonVariant::Secondary).ui(self.ui)
    }

    /// Ghost button (transparent background) — low-emphasis actions.
    pub fn ghost_button(self, label: impl Into<WidgetText>) -> Response {
        Button::new(label).variant(ButtonVariant::Ghost).ui(self.ui)
    }

    /// Danger button — destructive actions.
    pub fn danger_button(self, label: impl Into<WidgetText>) -> Response {
        Button::new(label).variant(ButtonVariant::Danger).ui(self.ui)
    }

    /// Themed single-line text input bound to `text`.
    pub fn text_input<'t>(self, text: &'t mut String) -> TextInput<'t> {
        TextInput::new(text)
    }

    /// Themed horizontal slider bound to `value`.
    pub fn slider<'v>(self, value: &'v mut f32, range: std::ops::RangeInclusive<f32>) -> Slider<'v> {
        Slider::new(value, range)
    }

    /// Themed on/off switch bound to `value`.
    pub fn toggle<'v>(self, value: &'v mut bool) -> Toggle<'v> {
        Toggle::new(value)
    }
}
