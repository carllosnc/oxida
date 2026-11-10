//! Themed Dropdown & Context Menu components inspired by shadcn/ui and modern desktop standards.

use egui::{
    containers::{Popup, PopupCloseBehavior},
    pos2, vec2, Color32, FontFamily, FontId, Id, Margin, Response, Sense, Shadow, Stroke,
    StrokeKind, Ui, WidgetText,
};

use crate::icons;
use crate::theme;
use crate::util::{mix, radius};

/// Density of menu items and menus.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum MenuDensity {
    /// Compact density: 30 px row height (ideal for dense toolbars, code editors and small utility menus).
    Compact,
    /// Comfortable density: 38 px row height (default desktop standard for high-polish apps).
    #[default]
    Comfortable,
}

impl MenuDensity {
    /// Row height in logical points based on metrics.
    #[inline]
    pub fn row_height(self, metrics: &theme::Metrics) -> f32 {
        match self {
            Self::Compact => metrics.control_height_sm,
            Self::Comfortable => metrics.control_height_md,
        }
    }

    /// Horizontal inset padding for menu items and headers.
    #[inline]
    pub fn padding_x(self) -> f32 {
        match self {
            Self::Compact => 8.0,
            Self::Comfortable => 10.0,
        }
    }

    /// Icon glyph font size in Phosphor font.
    #[inline]
    pub fn icon_size(self) -> f32 {
        match self {
            Self::Compact => 15.0,
            Self::Comfortable => 17.0,
        }
    }
}

/// Visual type of a menu item.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum MenuItemType {
    /// Standard neutral menu action.
    #[default]
    Standard,
    /// Destructive menu action styled with the danger token (e.g. Delete, Remove).
    Destructive,
}

/// Interactive state of a menu item.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MenuItemState {
    /// Normal default idle state.
    Default,
    /// Pointer hovering over the item.
    Hover,
    /// Pointer button pressed down on the item.
    Pressed,
    /// Keyboard focus active with accent focus ring.
    Focus,
    /// Disabled state.
    Disabled,
}

/// A configurable menu item with icon, label, shortcut, and submenu support.
#[derive(Clone, Debug)]
pub struct MenuItem<'a> {
    pub label: WidgetText,
    pub icon: Option<&'a str>,
    pub shortcut: Option<WidgetText>,
    pub item_type: MenuItemType,
    pub disabled: bool,
    pub submenu: bool,
    pub density: Option<MenuDensity>,
    pub state_override: Option<MenuItemState>,
    pub show_icon: bool,
    pub show_shortcut: bool,
}

impl<'a> MenuItem<'a> {
    /// Create a new menu item with a label.
    pub fn new(label: impl Into<WidgetText>) -> Self {
        Self {
            label: label.into(),
            icon: None,
            shortcut: None,
            item_type: MenuItemType::Standard,
            disabled: false,
            submenu: false,
            density: None,
            state_override: None,
            show_icon: true,
            show_shortcut: true,
        }
    }

    /// Attach a leading Phosphor icon glyph (e.g. `icons::FILE_TEXT`).
    pub fn icon(mut self, icon: &'a str) -> Self {
        self.icon = Some(icon);
        self
    }

    /// Set a keyboard shortcut label displayed on the right edge (e.g. "Ctrl+N", "F2", "Del").
    pub fn shortcut(mut self, shortcut: impl Into<WidgetText>) -> Self {
        self.shortcut = Some(shortcut.into());
        self
    }

    /// Set the item visual type (Standard vs Destructive).
    pub fn item_type(mut self, item_type: MenuItemType) -> Self {
        self.item_type = item_type;
        self
    }

    /// Set whether this is a destructive action.
    pub fn destructive(mut self, destructive: bool) -> Self {
        self.item_type = if destructive {
            MenuItemType::Destructive
        } else {
            MenuItemType::Standard
        };
        self
    }

    /// Convenience helper for destructive items.
    pub fn danger(self) -> Self {
        self.destructive(true)
    }

    /// Set whether the item is disabled.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Set whether this item opens a submenu (renders a trailing right chevron `>`).
    pub fn submenu(mut self, submenu: bool) -> Self {
        self.submenu = submenu;
        self
    }

    /// Explicitly override density for this individual item.
    pub fn density(mut self, density: MenuDensity) -> Self {
        self.density = Some(density);
        self
    }

    /// Set density to Compact (30 px).
    pub fn compact(self) -> Self {
        self.density(MenuDensity::Compact)
    }

    /// Set density to Comfortable (38 px).
    pub fn comfortable(self) -> Self {
        self.density(MenuDensity::Comfortable)
    }

    /// Explicitly override interactive state (used for testing or showcasing exact states).
    pub fn state(mut self, state: MenuItemState) -> Self {
        self.state_override = Some(state);
        self
    }

    /// Control icon visibility.
    pub fn show_icon(mut self, show: bool) -> Self {
        self.show_icon = show;
        self
    }

    /// Control shortcut visibility.
    pub fn show_shortcut(mut self, show: bool) -> Self {
        self.show_shortcut = show;
        self
    }
}

/// Dropdown menu builder and container styled after shadcn/ui and modern desktop HIG.
pub struct DropdownMenu<'a, R> {
    id_salt: Id,
    trigger_label: WidgetText,
    content: Box<dyn FnOnce(&mut DropdownMenuUi<'_>) -> R + 'a>,
    width: Option<f32>,
    density: MenuDensity,
}

impl<'a, R> DropdownMenu<'a, R> {
    /// Create a new dropdown menu trigger with a label and menu body closure.
    pub fn new(
        id_salt: impl std::hash::Hash + std::fmt::Debug,
        trigger_label: impl Into<WidgetText>,
        content: impl FnOnce(&mut DropdownMenuUi<'_>) -> R + 'a,
    ) -> Self {
        Self {
            id_salt: Id::unique(id_salt),
            trigger_label: trigger_label.into(),
            content: Box::new(content),
            width: None,
            density: MenuDensity::Comfortable,
        }
    }

    /// Set a custom width for the dropdown menu popup.
    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    /// Set the menu density (Compact = 30 px, Comfortable = 38 px).
    pub fn density(mut self, density: MenuDensity) -> Self {
        self.density = density;
        self
    }

    /// Convenience helper for compact density (30 px row height).
    pub fn compact(self) -> Self {
        self.density(MenuDensity::Compact)
    }

    /// Convenience helper for comfortable density (38 px row height).
    pub fn comfortable(self) -> Self {
        self.density(MenuDensity::Comfortable)
    }

    /// Render the dropdown trigger and popup menu if open.
    pub fn show(self, ui: &mut Ui) -> Option<R> {
        let theme = theme::current(ui.ctx());
        let p = &theme.palette;
        let m = &theme.metrics;

        let popup_id = self.id_salt.with("dropdown_popup");
        let is_open = Popup::is_id_open(ui.ctx(), popup_id);

        // Render trigger as a soft surface button with chevron
        let height = self.density.row_height(m);
        let pad_x = m.spacing_unit * 3.0;

        let font_id = theme.typography.font_id(crate::widgets::Size::Md);
        let galley = self.trigger_label.into_galley(
            ui,
            Some(egui::TextWrapMode::Extend),
            f32::INFINITY,
            font_id,
        );

        let desired_w = galley.size().x + pad_x * 2.0 + 16.0;
        let (rect, response) = ui.allocate_exact_size(vec2(desired_w, height), Sense::click());

        // Trigger visuals (borderless soft surface)
        let border_stroke = if response.has_focus() {
            Stroke::new(m.focus_ring_width, p.accent)
        } else {
            Stroke::NONE
        };

        let bg = if is_open {
            mix(p.subtle, p.accent, 0.08)
        } else if response.hovered() {
            p.subtle
        } else {
            p.field_bg
        };

        let rounding = radius(m.radius_md);
        ui.painter().rect_filled(rect, rounding, bg);
        ui.painter().rect_stroke(
            rect,
            rounding,
            border_stroke,
            StrokeKind::Inside,
        );

        // Label
        let text_pos = pos2(
            rect.left() + pad_x,
            rect.center().y - galley.size().y / 2.0,
        );
        ui.painter().galley(text_pos, galley, p.text_primary);

        // Chevron (points up when open with higher contrast)
        let chevron_center = pos2(rect.right() - 14.0, rect.center().y);
        let chevron_color = if is_open {
            p.text_primary
        } else {
            p.text_secondary
        };
        if is_open {
            paint_chevron_up(ui.painter(), chevron_center, chevron_color);
        } else {
            paint_chevron_down(ui.painter(), chevron_center, chevron_color);
        }

        // Popover Content
        let min_popup_w = self.width.unwrap_or(224.0).max(rect.width());

        let popup = Popup::menu(&response)
            .id(popup_id)
            .width(min_popup_w)
            .gap(4.0)
            .close_behavior(PopupCloseBehavior::CloseOnClick)
            .frame(menu_popup_frame(&theme));

        let mut output = None;
        popup.show(|ui| {
            ui.set_min_width(min_popup_w - 8.0);
            ui.spacing_mut().item_spacing.y = 2.0;
            let mut menu_ui = DropdownMenuUi {
                ui,
                theme: &theme,
                density: self.density,
            };
            output = Some((self.content)(&mut menu_ui));
        });

        output
    }
}

/// Context menu container triggered on secondary click (right-click) of any widget.
pub struct ContextMenu<'a, R> {
    id_salt: Id,
    content: Box<dyn FnOnce(&mut DropdownMenuUi<'_>) -> R + 'a>,
    width: Option<f32>,
    density: MenuDensity,
}

impl<'a, R> ContextMenu<'a, R> {
    /// Create a context menu definition bound to an identifier.
    pub fn new(
        id_salt: impl std::hash::Hash + std::fmt::Debug,
        content: impl FnOnce(&mut DropdownMenuUi<'_>) -> R + 'a,
    ) -> Self {
        Self {
            id_salt: Id::unique(id_salt),
            content: Box::new(content),
            width: None,
            density: MenuDensity::Comfortable,
        }
    }

    /// Set a custom width for the context menu popup.
    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    /// Set menu density.
    pub fn density(mut self, density: MenuDensity) -> Self {
        self.density = density;
        self
    }

    /// Convenience helper for compact density (30 px).
    pub fn compact(self) -> Self {
        self.density(MenuDensity::Compact)
    }

    /// Convenience helper for comfortable density (38 px).
    pub fn comfortable(self) -> Self {
        self.density(MenuDensity::Comfortable)
    }

    /// Attach this context menu to a widget response and show it when right-clicked.
    pub fn show_for_response(self, ui: &mut Ui, response: &Response) -> Option<R> {
        let theme = theme::current(ui.ctx());
        let popup_id = self.id_salt.with("context_menu_popup");
        let min_popup_w = self.width.unwrap_or(224.0);

        let popup = Popup::context_menu(response)
            .id(popup_id)
            .width(min_popup_w)
            .gap(4.0)
            .close_behavior(PopupCloseBehavior::CloseOnClick)
            .frame(menu_popup_frame(&theme));

        let mut output = None;
        popup.show(|ui| {
            ui.set_min_width(min_popup_w - 8.0);
            ui.spacing_mut().item_spacing.y = 2.0;
            let mut menu_ui = DropdownMenuUi {
                ui,
                theme: &theme,
                density: self.density,
            };
            output = Some((self.content)(&mut menu_ui));
        });

        output
    }
}

/// Standalone menu panel container (useful for static menus, preview galleries, or custom cards).
pub struct MenuPanel<'a, R> {
    content: Box<dyn FnOnce(&mut DropdownMenuUi<'_>) -> R + 'a>,
    width: Option<f32>,
    density: MenuDensity,
    header: Option<WidgetText>,
}

impl<'a, R> MenuPanel<'a, R> {
    /// Create a standalone menu panel.
    pub fn new(content: impl FnOnce(&mut DropdownMenuUi<'_>) -> R + 'a) -> Self {
        Self {
            content: Box::new(content),
            width: None,
            density: MenuDensity::Comfortable,
            header: None,
        }
    }

    /// Set panel width.
    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    /// Set menu density.
    pub fn density(mut self, density: MenuDensity) -> Self {
        self.density = density;
        self
    }

    /// Set compact density (30 px).
    pub fn compact(self) -> Self {
        self.density(MenuDensity::Compact)
    }

    /// Set comfortable density (38 px).
    pub fn comfortable(self) -> Self {
        self.density(MenuDensity::Comfortable)
    }

    /// Optional section title at top of the panel.
    pub fn header(mut self, title: impl Into<WidgetText>) -> Self {
        self.header = Some(title.into());
        self
    }

    /// Render the menu panel.
    pub fn show(self, ui: &mut Ui) -> R {
        let theme = theme::current(ui.ctx());
        let p = &theme.palette;
        let m = &theme.metrics;

        let frame = egui::Frame {
            fill: p.popover,
            stroke: Stroke::new(1.0, mix(p.popover, p.border, 0.4)),
            corner_radius: radius(m.radius_md),
            inner_margin: Margin::same(5),
            shadow: Shadow {
                color: if theme.dark_mode {
                    Color32::from_black_alpha(120)
                } else {
                    Color32::from_black_alpha(35)
                },
                offset: [0, 6],
                blur: 18,
                spread: 0,
            },
            ..Default::default()
        };

        frame
            .show(ui, |ui| {
                if let Some(w) = self.width {
                    ui.set_width(w);
                }
                ui.spacing_mut().item_spacing.y = 2.0;

                let mut menu_ui = DropdownMenuUi {
                    ui,
                    theme: &theme,
                    density: self.density,
                };

                if let Some(hdr) = self.header {
                    menu_ui.header(hdr);
                }

                (self.content)(&mut menu_ui)
            })
            .inner
    }
}

/// Helper context passed into a dropdown or context menu's body closure.
pub struct DropdownMenuUi<'a> {
    ui: &'a mut Ui,
    theme: &'a theme::Theme,
    density: MenuDensity,
}

impl<'a> DropdownMenuUi<'a> {
    /// Create a new `DropdownMenuUi` context.
    pub fn new(ui: &'a mut Ui, theme: &'a theme::Theme, density: MenuDensity) -> Self {
        Self { ui, theme, density }
    }

    /// Get current density.
    pub fn density(&self) -> MenuDensity {
        self.density
    }

    /// Override density for subsequent items.
    pub fn set_density(&mut self, density: MenuDensity) {
        self.density = density;
    }

    /// Add a regular clickable menu item.
    pub fn item(&mut self, label: impl Into<WidgetText>) -> Response {
        self.add_item(MenuItem::new(label))
    }

    /// Add a menu item with a leading icon and trailing shortcut.
    pub fn item_with(
        &mut self,
        label: impl Into<WidgetText>,
        icon: Option<&str>,
        shortcut: Option<&str>,
    ) -> Response {
        let mut item = MenuItem::new(label);
        if let Some(ic) = icon {
            item = item.icon(ic);
        }
        if let Some(sc) = shortcut {
            item = item.shortcut(sc);
        }
        self.add_item(item)
    }

    /// Add a destructive/danger menu item (styled with the theme's danger color).
    pub fn danger_item(&mut self, label: impl Into<WidgetText>) -> Response {
        self.add_item(MenuItem::new(label).danger())
    }

    /// Add a destructive menu item with an icon and shortcut.
    pub fn danger_item_with(
        &mut self,
        label: impl Into<WidgetText>,
        icon: Option<&str>,
        shortcut: Option<&str>,
    ) -> Response {
        let mut item = MenuItem::new(label).danger();
        if let Some(ic) = icon {
            item = item.icon(ic);
        }
        if let Some(sc) = shortcut {
            item = item.shortcut(sc);
        }
        self.add_item(item)
    }

    /// Add a submenu indicator item (renders trailing right chevron `>`).
    pub fn submenu_item(
        &mut self,
        label: impl Into<WidgetText>,
        icon: Option<&str>,
    ) -> Response {
        let mut item = MenuItem::new(label).submenu(true);
        if let Some(ic) = icon {
            item = item.icon(ic);
        }
        self.add_item(item)
    }

    /// Add a disabled non-clickable menu item.
    pub fn disabled_item(&mut self, label: impl Into<WidgetText>) -> Response {
        self.add_item(MenuItem::new(label).disabled(true))
    }

    /// Add a fully configured [`MenuItem`].
    pub fn add_item(&mut self, item: MenuItem<'_>) -> Response {
        let p = &self.theme.palette;
        let m = &self.theme.metrics;
        let t = &self.theme.typography;

        let density = item.density.unwrap_or(self.density);
        let height = density.row_height(m);
        let pad_x = density.padding_x();

        let desired = vec2(self.ui.available_width(), height);
        let sense = if item.disabled {
            Sense::hover()
        } else {
            Sense::click()
        };
        let (rect, response) = self.ui.allocate_exact_size(desired, sense);

        // Determine effective visual state
        let state = if let Some(st) = item.state_override {
            st
        } else if item.disabled {
            MenuItemState::Disabled
        } else if response.is_pointer_button_down_on() {
            MenuItemState::Pressed
        } else if response.hovered() {
            MenuItemState::Hover
        } else if response.has_focus() {
            MenuItemState::Focus
        } else {
            MenuItemState::Default
        };

        // 1. Background Fill
        let is_destructive = item.item_type == MenuItemType::Destructive;
        let bg = match state {
            MenuItemState::Disabled | MenuItemState::Default => Color32::TRANSPARENT,
            MenuItemState::Focus => mix(p.popover, p.accent, 0.04),
            MenuItemState::Hover => {
                if is_destructive {
                    crate::util::dim(p.danger, p.popover, 0.88)
                } else {
                    p.subtle
                }
            }
            MenuItemState::Pressed => {
                if is_destructive {
                    crate::util::dim(p.danger, p.popover, 0.74)
                } else {
                    mix(p.subtle, p.accent, 0.12)
                }
            }
        };

        // 2. Focus Ring Stroke
        let border_stroke = if state == MenuItemState::Focus {
            Stroke::new(m.focus_ring_width, p.accent)
        } else {
            Stroke::NONE
        };

        let rounding = radius(m.radius_sm);
        if bg != Color32::TRANSPARENT {
            self.ui.painter().rect_filled(rect, rounding, bg);
        }
        if border_stroke != Stroke::NONE {
            self.ui
                .painter()
                .rect_stroke(rect, rounding, border_stroke, StrokeKind::Inside);
        }

        // 3. Foreground Colors
        let (icon_color, label_color, shortcut_color, chevron_color) = match state {
            MenuItemState::Disabled => (
                p.text_disabled,
                p.text_disabled,
                p.text_disabled,
                p.text_disabled,
            ),
            MenuItemState::Hover | MenuItemState::Pressed => {
                if is_destructive {
                    (p.danger, p.danger, p.text_secondary, p.danger)
                } else {
                    (
                        p.text_primary,
                        p.text_primary,
                        p.text_secondary,
                        p.text_primary,
                    )
                }
            }
            MenuItemState::Focus | MenuItemState::Default => {
                if is_destructive {
                    (p.danger, p.danger, p.text_secondary, p.text_secondary)
                } else {
                    let fg = mix(p.text_primary, p.text_secondary, 0.18);
                    (fg, fg, p.text_secondary, p.text_secondary)
                }
            }
        };

        // 4. Paint Elements
        let mut curr_x = rect.left() + pad_x;

        // Leading Icon
        if item.show_icon {
            if let Some(ic) = item.icon {
                let ic_size = density.icon_size();
                let ic_font = FontId::new(
                    ic_size,
                    FontFamily::Name(crate::fonts::PHOSPHOR.into()),
                );
                let ic_galley = self
                    .ui
                    .scope(|ui| {
                        ui.style_mut().visuals.override_text_color = Some(icon_color);
                        WidgetText::from(ic).into_galley(
                            ui,
                            Some(egui::TextWrapMode::Truncate),
                            f32::INFINITY,
                            ic_font,
                        )
                    })
                    .inner;

                let ic_pos = pos2(
                    curr_x,
                    rect.center().y - ic_galley.size().y * 0.5,
                );
                self.ui.painter().galley(ic_pos, ic_galley, icon_color);

                let icon_gap = if density == MenuDensity::Compact { 8.0 } else { 10.0 };
                curr_x += ic_size + icon_gap;
            }
        }

        // Trailing elements (Shortcut or Submenu Chevron)
        let mut right_limit = rect.right() - pad_x;

        if item.submenu {
            let chevron_font = FontId::new(
                if density == MenuDensity::Compact { 13.0 } else { 14.0 },
                FontFamily::Name(crate::fonts::PHOSPHOR.into()),
            );
            let chev_galley = self
                .ui
                .scope(|ui| {
                    ui.style_mut().visuals.override_text_color = Some(chevron_color);
                    WidgetText::from(icons::CARET_RIGHT).into_galley(
                        ui,
                        Some(egui::TextWrapMode::Truncate),
                        f32::INFINITY,
                        chevron_font,
                    )
                })
                .inner;

            let chev_w = chev_galley.size().x;
            let chev_pos = pos2(
                right_limit - chev_w,
                rect.center().y - chev_galley.size().y * 0.5,
            );
            self.ui.painter().galley(chev_pos, chev_galley, chevron_color);
            right_limit -= chev_w + 8.0;
        } else if item.show_shortcut {
            if let Some(ref sc) = item.shortcut {
                let sc_font = FontId::new(
                    t.xs,
                    FontFamily::Name(crate::fonts::REGULAR.into()),
                );
                let sc_galley = self
                    .ui
                    .scope(|ui| {
                        ui.style_mut().visuals.override_text_color = Some(shortcut_color);
                        sc.clone().into_galley(
                            ui,
                            Some(egui::TextWrapMode::Truncate),
                            f32::INFINITY,
                            sc_font,
                        )
                    })
                    .inner;

                let sc_w = sc_galley.size().x;
                let sc_pos = pos2(
                    right_limit - sc_w,
                    rect.center().y - sc_galley.size().y * 0.5,
                );
                self.ui.painter().galley(sc_pos, sc_galley, shortcut_color);
                right_limit -= sc_w + 8.0;
            }
        }

        // Label
        let label_size = if density == MenuDensity::Compact {
            t.sm
        } else {
            t.sm + 0.5
        };
        let label_font = FontId::new(
            label_size,
            FontFamily::Name(crate::fonts::MEDIUM.into()),
        );
        let max_label_w = (right_limit - curr_x).max(0.0);

        let label_galley = self
            .ui
            .scope(|ui| {
                ui.style_mut().visuals.override_text_color = Some(label_color);
                item.label.into_galley(
                    ui,
                    Some(egui::TextWrapMode::Truncate),
                    max_label_w,
                    label_font,
                )
            })
            .inner;

        let label_pos = pos2(
            curr_x,
            rect.center().y - label_galley.size().y * 0.5,
        );
        self.ui.painter().galley(label_pos, label_galley, label_color);

        if !item.disabled && response.clicked() {
            Popup::close_all(self.ui.ctx());
        }

        response
    }

    /// Add a section subhead or label inside the dropdown (e.g. "DOCUMENTO").
    pub fn header(&mut self, text: impl Into<WidgetText>) {
        let p = &self.theme.palette;
        let t = &self.theme.typography;
        let pad_x = self.density.padding_x();

        self.ui.add_space(4.0);
        let font_id = FontId::new(
            t.xs - 0.5,
            FontFamily::Name(crate::fonts::SEMI_BOLD.into()),
        );
        let label = text.into();
        let galley = self
            .ui
            .scope(|ui| {
                ui.style_mut().visuals.override_text_color = Some(p.text_secondary);
                label.into_galley(
                    ui,
                    Some(egui::TextWrapMode::Truncate),
                    ui.available_width() - pad_x * 2.0,
                    font_id,
                )
            })
            .inner;

        let height = 22.0;
        let (rect, _) = self
            .ui
            .allocate_exact_size(vec2(self.ui.available_width(), height), Sense::hover());
        let text_pos = pos2(
            rect.left() + pad_x,
            rect.center().y - galley.size().y / 2.0,
        );
        self.ui.painter().galley(text_pos, galley, p.text_secondary);
        self.ui.add_space(2.0);
    }

    /// Add a thin hairline divider between menu groups.
    pub fn separator(&mut self) {
        let p = &self.theme.palette;
        self.ui.add_space(4.0);
        let (rect, _) = self
            .ui
            .allocate_exact_size(vec2(self.ui.available_width(), 1.0), Sense::hover());
        self.ui.painter().hline(
            rect.x_range(),
            rect.center().y,
            Stroke::new(1.0, mix(p.popover, p.subtle, 0.7)),
        );
        self.ui.add_space(4.0);
    }
}

/// Helper function to build a consistent popup frame for dropdowns and context menus.
fn menu_popup_frame(theme: &theme::Theme) -> egui::Frame {
    let p = &theme.palette;
    let m = &theme.metrics;

    egui::Frame {
        fill: p.popover,
        stroke: Stroke::new(1.0, mix(p.popover, p.border, 0.4)),
        corner_radius: radius(m.radius_md),
        inner_margin: Margin::same(4),
        shadow: Shadow {
            color: if theme.dark_mode {
                Color32::from_black_alpha(160)
            } else {
                Color32::from_black_alpha(45)
            },
            offset: [0, 8],
            blur: 20,
            spread: if theme.dark_mode { 0 } else { 2 },
        },
        ..Default::default()
    }
}

/// Paint a clean vector chevron pointing up.
fn paint_chevron_up(painter: &egui::Painter, center: egui::Pos2, color: egui::Color32) {
    let stroke = Stroke::new(1.4, color);
    let size = 3.2;
    painter.line_segment(
        [
            pos2(center.x - size, center.y + 1.2),
            pos2(center.x, center.y - 2.0),
        ],
        stroke,
    );
    painter.line_segment(
        [
            pos2(center.x, center.y - 2.0),
            pos2(center.x + size, center.y + 1.2),
        ],
        stroke,
    );
}

/// Paint a clean vector chevron pointing down.
fn paint_chevron_down(painter: &egui::Painter, center: egui::Pos2, color: egui::Color32) {
    let stroke = Stroke::new(1.4, color);
    let size = 3.2;
    painter.line_segment(
        [
            pos2(center.x - size, center.y - 1.2),
            pos2(center.x, center.y + 2.0),
        ],
        stroke,
    );
    painter.line_segment(
        [
            pos2(center.x, center.y + 2.0),
            pos2(center.x + size, center.y - 1.2),
        ],
        stroke,
    );
}
