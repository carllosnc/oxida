//! Themed Dropdown Menu component inspired by shadcn/ui.

use egui::{
    containers::{Popup, PopupCloseBehavior},
    pos2, vec2, Color32, Id, Margin, Response, Sense, Shadow, Stroke, Ui, WidgetText,
};

use crate::theme;
use crate::util::radius;

/// Dropdown menu builder and container styled after shadcn/ui.
pub struct DropdownMenu<'a, R> {
    id_salt: Id,
    trigger_label: WidgetText,
    content: Box<dyn FnOnce(&mut DropdownMenuUi<'_>) -> R + 'a>,
    width: Option<f32>,
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
        }
    }

    /// Set a custom width for the dropdown menu popup.
    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    /// Render the dropdown trigger and popup menu if open.
    pub fn show(self, ui: &mut Ui) -> Option<R> {
        let theme = theme::current(ui.ctx());
        let p = &theme.palette;
        let m = &theme.metrics;

        let popup_id = self.id_salt.with("dropdown_popup");
        let is_open = Popup::is_id_open(ui.ctx(), popup_id);

        // Render trigger as a secondary button with chevron
        let height = m.control_height_md;
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

        // Note: Do NOT call Popup::toggle_id manually here.
        // `Popup::menu(&response)` already toggles open state when `response.clicked()` is true.

        // Trigger visuals
        let border_stroke = if is_open {
            Stroke::new(1.5, p.accent)
        } else if response.hovered() {
            Stroke::new(m.border_width, p.border_strong)
        } else {
            Stroke::new(m.border_width, p.border)
        };

        let bg = if is_open || response.hovered() {
            p.subtle
        } else {
            p.surface
        };

        let rounding = radius(m.radius_md);
        ui.painter().rect_filled(rect, rounding, bg);
        ui.painter().rect_stroke(
            rect,
            rounding,
            border_stroke,
            egui::StrokeKind::Inside,
        );

        // Label
        let text_pos = pos2(
            rect.left() + pad_x,
            rect.center().y - galley.size().y / 2.0,
        );
        ui.painter().galley(text_pos, galley, p.text_primary);

        // Chevron
        let chevron_center = pos2(rect.right() - 14.0, rect.center().y);
        paint_chevron_down(ui.painter(), chevron_center, p.text_secondary);

        // Popover Content
        let min_popup_w = self.width.unwrap_or(220.0).max(rect.width());

        let popup = Popup::menu(&response)
            .id(popup_id)
            .width(min_popup_w)
            .gap(4.0)
            .close_behavior(PopupCloseBehavior::CloseOnClick)
            .frame(egui::Frame {
                fill: p.surface,
                stroke: Stroke::new(m.border_width, p.border),
                corner_radius: radius(m.radius_md),
                inner_margin: Margin::same(4),
                shadow: Shadow {
                    color: if theme.dark_mode {
                        Color32::from_black_alpha(100)
                    } else {
                        Color32::from_black_alpha(24)
                    },
                    offset: [0, 4],
                    blur: 10,
                    spread: 0,
                },
                ..Default::default()
            });

        let mut output = None;
        popup.show(|ui| {
            ui.set_min_width(min_popup_w - 8.0);
            ui.spacing_mut().item_spacing.y = 2.0;
            let mut menu_ui = DropdownMenuUi {
                ui,
                theme: &theme,
            };
            output = Some((self.content)(&mut menu_ui));
        });

        output
    }
}

/// Helper context passed into a dropdown menu's body closure.
pub struct DropdownMenuUi<'a> {
    ui: &'a mut Ui,
    theme: &'a theme::Theme,
}

impl DropdownMenuUi<'_> {
    /// Add a regular clickable menu item.
    pub fn item(&mut self, label: impl Into<WidgetText>) -> Response {
        self.render_item(label.into(), false, false)
    }

    /// Add a destructive/danger menu item (styled with the theme's danger color).
    pub fn danger_item(&mut self, label: impl Into<WidgetText>) -> Response {
        self.render_item(label.into(), true, false)
    }

    /// Add a disabled non-clickable menu item.
    pub fn disabled_item(&mut self, label: impl Into<WidgetText>) -> Response {
        self.render_item(label.into(), false, true)
    }

    /// Add a section subhead or label inside the dropdown.
    pub fn header(&mut self, text: impl Into<WidgetText>) {
        let p = &self.theme.palette;
        let t = &self.theme.typography;
        self.ui.add_space(4.0);
        let font_id = egui::FontId::new(
            t.xs - 0.5,
            egui::FontFamily::Name(crate::fonts::SEMI_BOLD.into()),
        );
        let label = text.into();
        let galley = self
            .ui
            .scope(|ui| {
                ui.style_mut().visuals.override_text_color = Some(p.text_secondary);
                label.into_galley(
                    ui,
                    Some(egui::TextWrapMode::Truncate),
                    ui.available_width() - 16.0,
                    font_id,
                )
            })
            .inner;
        let height = 20.0;
        let (rect, _) = self
            .ui
            .allocate_exact_size(vec2(self.ui.available_width(), height), Sense::hover());
        let text_pos = pos2(
            rect.left() + 8.0,
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
            Stroke::new(1.0, p.border),
        );
        self.ui.add_space(4.0);
    }

    fn render_item(&mut self, label: WidgetText, danger: bool, disabled: bool) -> Response {
        let p = &self.theme.palette;
        let m = &self.theme.metrics;

        let height = 28.0;
        let desired = vec2(self.ui.available_width(), height);

        let sense = if disabled {
            Sense::hover()
        } else {
            Sense::click()
        };
        let (rect, response) = self.ui.allocate_exact_size(desired, sense);

        let bg = if disabled {
            Color32::TRANSPARENT
        } else if response.hovered() {
            if danger {
                crate::util::dim(p.danger, p.surface, 0.85)
            } else {
                p.subtle
            }
        } else {
            Color32::TRANSPARENT
        };

        let fg = if disabled {
            p.text_disabled
        } else if danger {
            p.danger
        } else if response.hovered() {
            p.text_primary
        } else {
            p.text_primary
        };

        self.ui
            .painter()
            .rect_filled(rect, radius(m.radius_sm), bg);

        let font_id = self.theme.typography.font_id(crate::widgets::Size::Md);
        let galley = self
            .ui
            .scope(|ui| {
                ui.style_mut().visuals.override_text_color = Some(fg);
                label.into_galley(
                    ui,
                    Some(egui::TextWrapMode::Truncate),
                    rect.width() - 16.0,
                    font_id,
                )
            })
            .inner;

        let text_pos = pos2(
            rect.left() + 8.0,
            rect.center().y - galley.size().y / 2.0,
        );
        self.ui.painter().galley(text_pos, galley, fg);

        if !disabled && response.clicked() {
            Popup::close_all(self.ui.ctx());
        }

        response
    }
}

/// Paint a clean vector chevron pointing down.
fn paint_chevron_down(painter: &egui::Painter, center: egui::Pos2, color: egui::Color32) {
    let stroke = egui::Stroke::new(1.4, color);
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
