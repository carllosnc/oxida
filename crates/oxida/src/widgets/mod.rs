//! The Oxida widget set and the `ui.oxida()` builder entry point.

mod alert;
mod avatar;
mod badge;
mod button;
mod card;
mod checkbox;
mod dropdown;
mod icon_button;
mod progress;
mod radio;
mod select;
mod separator;
mod slider;
mod tabs;
mod text_input;
mod toggle;

pub use alert::{Alert, AlertVariant};
pub use avatar::{Avatar, AvatarStatus};
pub use badge::{Badge, BadgeVariant};
pub use button::{Button, ButtonVariant};
pub use card::Card;
pub use checkbox::Checkbox;
pub use dropdown::{DropdownMenu, DropdownMenuUi};
pub use icon_button::IconButton;
pub use progress::{ProgressBar, Spinner};
pub use radio::Radio;
pub use select::{Select, SelectItem};
pub use separator::{Separator, SeparatorDirection};
pub use slider::Slider;
pub use tabs::Tabs;
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

    /// Icon button builder (defaults to [`ButtonVariant::Ghost`]).
    pub fn icon_button(self, icon: impl Into<WidgetText>) -> IconButton {
        IconButton::new(icon)
    }

    /// Ghost icon button rendered immediately.
    pub fn ghost_icon(self, icon: impl Into<WidgetText>) -> Response {
        IconButton::ghost(icon).ui(self.ui)
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

    /// Themed Checkbox styled after shadcn/ui.
    pub fn checkbox<'v>(self, checked: &'v mut bool, label: impl Into<WidgetText>) -> Checkbox<'v> {
        Checkbox::new(checked).label(label)
    }

    /// Themed Radio button styled after shadcn/ui.
    pub fn radio<'v, T: PartialEq + 'static>(
        self,
        current_value: &'v mut T,
        selected_value: T,
        label: impl Into<WidgetText>,
    ) -> Radio<'v, T> {
        Radio::new(current_value, selected_value).label(label)
    }

    /// Themed Select dropdown styled after shadcn/ui.
    pub fn select<'v, T: Clone + PartialEq + 'static>(
        self,
        id_salt: impl std::hash::Hash + std::fmt::Debug,
        selected: &'v mut T,
    ) -> Select<'v, T> {
        Select::new(id_salt, selected)
    }

    /// Themed Dropdown Menu styled after shadcn/ui.
    pub fn dropdown_menu<R>(
        self,
        id_salt: impl std::hash::Hash + std::fmt::Debug,
        trigger_label: impl Into<WidgetText>,
        content: impl FnOnce(&mut DropdownMenuUi<'_>) -> R + 'a,
    ) -> Option<R> {
        DropdownMenu::new(id_salt, trigger_label, content).show(self.ui)
    }

    /// Themed badge / tag chip.
    pub fn badge(self, text: impl Into<WidgetText>) -> Response {
        Badge::new(text).ui(self.ui)
    }

    /// Themed horizontal separator / divider.
    pub fn separator(self) -> Response {
        Separator::horizontal().ui(self.ui)
    }

    /// Themed progress bar.
    pub fn progress_bar(self, progress: f32) -> Response {
        ProgressBar::new(progress).ui(self.ui)
    }

    /// Themed indeterminate loading spinner.
    pub fn spinner(self) -> Response {
        Spinner::new().ui(self.ui)
    }

    /// Themed user avatar with initials.
    pub fn avatar(self, name: &str) -> Response {
        Avatar::initials(name).ui(self.ui)
    }
}
