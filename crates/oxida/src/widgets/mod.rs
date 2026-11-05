//! The Oxida widget set and the `ui.oxida()` builder entry point.

mod accordion;
mod alert;
mod avatar;
mod badge;
mod block_slider;
mod breadcrumbs;
mod button;
mod card;
mod checkbox;
mod command_palette;
mod dropdown;
mod icon_button;
mod kbd;
mod progress;
mod radio;
mod search_input;
mod select;
mod separator;
mod slider;
mod tabs;
mod text_input;
mod toggle;

pub use accordion::{Accordion, AccordionGroup, AccordionResponse, AccordionVariant};
pub use alert::{Alert, AlertVariant};
pub use avatar::{Avatar, AvatarStatus};
pub use badge::{Badge, BadgeVariant};
pub use block_slider::BlockSlider;
pub use breadcrumbs::{BreadcrumbItem, Breadcrumbs};
pub use button::{Button, ButtonVariant};
pub use card::Card;
pub use checkbox::{Checkbox, CheckboxState};
pub use command_palette::{CommandItem, CommandPalette};
pub use dropdown::{DropdownMenu, DropdownMenuUi};
pub use icon_button::IconButton;
pub use kbd::Kbd;
pub use progress::{ProgressBar, Spinner};
pub use radio::Radio;
pub use search_input::{SearchInput, SearchInputResponse};
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
    /// Compact: dense toolbars, badges (30 px row / small icons).
    Sm,
    /// Default control size (38 px row / standard buttons).
    #[default]
    Md,
    /// Large: prominent/primary controls (46 px row).
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

    /// Themed block slider / filled scrubber pill bound to `value`.
    pub fn block_slider<'v>(
        self,
        value: &'v mut f32,
        range: std::ops::RangeInclusive<f32>,
    ) -> BlockSlider<'v> {
        BlockSlider::new(value, range)
    }

    /// Themed on/off switch bound to `value`.
    pub fn toggle<'v>(self, value: &'v mut bool) -> Toggle<'v> {
        Toggle::new(value)
    }

    /// Themed Checkbox styled after modern desktop HIG.
    pub fn checkbox<'v>(self, checked: &'v mut bool, label: impl Into<WidgetText>) -> Checkbox<'v> {
        Checkbox::new(checked).label(label)
    }

    /// Themed Checkbox bound to a tri-state [`CheckboxState`].
    pub fn checkbox_state<'v>(self, state: &'v mut CheckboxState, label: impl Into<WidgetText>) -> Checkbox<'v> {
        Checkbox::state(state).label(label)
    }

    /// Themed Radio button styled after modern desktop HIG.
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

    /// Themed keyboard shortcut badge (`<kbd>`).
    pub fn kbd(self, key: impl Into<String>) -> Response {
        Kbd::new(key).ui(self.ui)
    }

    /// Themed search input with magnifying glass, clear button and shortcut.
    pub fn search_input<'t>(self, text: &'t mut String) -> SearchInput<'t> {
        SearchInput::new(text)
    }

    /// Themed breadcrumbs navigation trail.
    pub fn breadcrumbs(self) -> Breadcrumbs {
        Breadcrumbs::new()
    }

    /// Themed Command Palette overlay builder (`Ctrl+K` / `⌘K`).
    pub fn command_palette<'b>(self, open: &'b mut bool) -> CommandPalette<'b> {
        CommandPalette::new(open)
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

    /// Themed expandable/collapsible accordion section.
    pub fn accordion(self, title: impl Into<WidgetText>) -> Accordion<'static> {
        Accordion::new(title)
    }
}
