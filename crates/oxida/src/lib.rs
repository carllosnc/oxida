//! # Oxida
//!
//! A minimal, neutral design system and component library for
//! [egui](https://github.com/emilk/egui) — theme tokens plus hand-crafted
//! widgets for building clean desktop interfaces.
//!
//! ## Quick start
//!
//! ```no_run
//! use oxida::prelude::*;
//!
//! # struct MyApp {
//! #     name: String,
//! #     volume: f32,
//! # }
//! impl MyApp {
//!     fn ui(&mut self, ui: &mut egui::Ui) {
//!         ui.oxida().text_input(&mut self.name).placeholder("Nome").ui(ui);
//!         ui.oxida().slider(&mut self.volume, 0.0..=100.0).label("Volume").ui(ui);
//!         if ui.oxida().button("Salvar").clicked() {
//!             println!("{}", self.name);
//!         }
//!     }
//! }
//! ```
//!
//! Apply a theme at startup (e.g. inside your `eframe::App` creation closure):
//!
//! ```
//! use oxida::Theme;
//!
//! let ctx = egui::Context::default();
//! oxida::init(&ctx, &Theme::dark());
//! ```
//!
//! ## Architecture
//!
//! * [`Theme`] — design tokens ([`Palette`], [`Typography`], [`Metrics`]);
//!   apply with [`init`] (or [`Theme::apply`]).
//! * [`widgets`] — the components: [`Button`](widgets::Button),
//!   [`TextInput`](widgets::TextInput), [`Slider`](widgets::Slider),
//!   [`Toggle`](widgets::Toggle), [`Kbd`](widgets::Kbd),
//!   [`SearchInput`](widgets::SearchInput), [`CommandPalette`](widgets::CommandPalette), etc.
//! * [`OxidaUi`] — the extension trait adding `Ui::oxida()`.

mod style;
mod util;

pub mod fonts;
pub mod icons;
pub mod theme;
pub mod widgets;

pub use theme::{Metrics, Palette, Theme, Typography};
pub use widgets::{
    Alert, AlertVariant, Avatar, AvatarStatus, Badge, BadgeVariant, BreadcrumbItem, Breadcrumbs,
    Button, ButtonVariant, Card, Checkbox, CommandItem, CommandPalette, DropdownMenu, DropdownMenuUi,
    IconButton, Kbd, Oxida, ProgressBar, Radio, SearchInput, SearchInputResponse, Select,
    SelectItem, Separator, SeparatorDirection, Size, Slider, Spinner, Tabs, TextInput, Toggle,
};

/// Extension trait that adds the `oxida()` builder entry point to [`egui::Ui`].
///
/// ```ignore
/// ui.oxida().button("Save");                       // renders immediately
/// ui.oxida().slider(&mut v, 0.0..=1.0).ui(ui);     // builder → .ui(ui)
/// ```
pub trait OxidaUi {
    /// Access the Oxida widget builders for this `Ui`.
    fn oxida(&mut self) -> Oxida<'_>;
}

impl OxidaUi for egui::Ui {
    fn oxida(&mut self) -> Oxida<'_> {
        Oxida::new(self)
    }
}

/// Apply `theme` to `ctx`: installs the bundled Inter font, restyles egui's
/// built-in widgets and makes the theme available to every Oxida widget.
///
/// Call once at startup (e.g. inside `eframe::App`'s creation closure)
/// and again whenever the active theme changes.
///
/// If you need custom fonts, call [`fonts::install`] (or set your own
/// [`egui::FontDefinitions`]) after this.
pub fn init(ctx: &egui::Context, theme: &Theme) {
    fonts::install(ctx);
    theme.apply(ctx);
}

/// Commonly used items, ready to glob-import.
///
/// Also re-exports [`egui::Widget`] so that builder widgets can be finished
/// with `.ui(ui)` after importing the prelude.
pub mod prelude {
    pub use crate::icons;
    pub use crate::theme::{Metrics, Palette, Theme, Typography};
    pub use crate::widgets::{
        Alert, AlertVariant, Avatar, AvatarStatus, Badge, BadgeVariant, BreadcrumbItem, Breadcrumbs,
        Button, ButtonVariant, Card, Checkbox, CommandItem, CommandPalette, DropdownMenu,
        DropdownMenuUi, IconButton, Kbd, Oxida, ProgressBar, Radio, SearchInput,
        SearchInputResponse, Select, SelectItem, Separator, SeparatorDirection, Size, Slider,
        Spinner, Tabs, TextInput, Toggle,
    };
    pub use crate::{init, OxidaUi};
    pub use egui::Widget;
}
