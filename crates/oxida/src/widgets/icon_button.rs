//! Themed icon-only button widget.

use egui::{pos2, vec2, Color32, FontFamily, FontId, Response, Sense, Stroke, StrokeKind, Ui, Widget, WidgetText};

use super::{ButtonVariant, Size};
use crate::theme;
use crate::util::{mix, radius};

/// A square, icon-only button styled after shadcn/ui.
///
/// Ideal for toolbars, search/close actions, pagination, and header controls.
///
/// ```
/// # use oxida::prelude::*;
/// # use oxida::icons;
/// # fn demo(ui: &mut egui::Ui) {
/// IconButton::new(icons::MAGNIFYING_GLASS)
///     .tooltip("Buscar")
///     .ui(ui);
/// # }
/// ```
#[derive(Clone)]
pub struct IconButton {
    icon: WidgetText,
    variant: ButtonVariant,
    size: Size,
    enabled: bool,
    tooltip: Option<WidgetText>,
}

impl IconButton {
    /// Create an icon button. Defaults to [`ButtonVariant::Ghost`] and [`Size::Md`].
    pub fn new(icon: impl Into<WidgetText>) -> Self {
        Self {
            icon: icon.into(),
            variant: ButtonVariant::Ghost,
            size: Size::default(),
            enabled: true,
            tooltip: None,
        }
    }

    /// Primary icon button (accent background).
    pub fn primary(icon: impl Into<WidgetText>) -> Self {
        Self::new(icon).variant(ButtonVariant::Primary)
    }

    /// Secondary icon button (surface background with border).
    pub fn secondary(icon: impl Into<WidgetText>) -> Self {
        Self::new(icon).variant(ButtonVariant::Secondary)
    }

    /// Ghost icon button (transparent background until hovered).
    pub fn ghost(icon: impl Into<WidgetText>) -> Self {
        Self::new(icon).variant(ButtonVariant::Ghost)
    }

    /// Danger icon button (danger background/accent for destructive actions).
    pub fn danger(icon: impl Into<WidgetText>) -> Self {
        Self::new(icon).variant(ButtonVariant::Danger)
    }

    /// Set the visual variant.
    pub fn variant(mut self, variant: ButtonVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Set the size of the icon button.
    pub fn size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }

    /// Enable or disable the button.
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// Set an optional tooltip shown on hover.
    pub fn tooltip(mut self, tooltip: impl Into<WidgetText>) -> Self {
        self.tooltip = Some(tooltip.into());
        self
    }
}

impl Widget for IconButton {
    fn ui(self, ui: &mut Ui) -> Response {
        let theme = theme::current(ui.ctx());
        let p = &theme.palette;
        let m = &theme.metrics;

        let (base, hover, active, fg, border) = match self.variant {
            ButtonVariant::Primary => (p.accent, p.accent_hover, p.accent_active, p.on_accent, p.accent),
            ButtonVariant::Secondary => (p.subtle, mix(p.subtle, p.border, 0.4), p.border, p.text_primary, Color32::TRANSPARENT),
            ButtonVariant::Ghost => (Color32::TRANSPARENT, p.subtle, p.border, p.text_primary, Color32::TRANSPARENT),
            ButtonVariant::Danger => (p.danger, p.danger_hover, mix(p.danger, Color32::BLACK, 0.2), p.on_danger, p.danger),
        };

        let label_color = if self.enabled {
            fg
        } else {
            p.text_disabled
        };

        let side = match self.size {
            Size::Sm => m.control_height_sm.max(28.0),
            Size::Md => m.control_height_md,
            Size::Lg => m.control_height_lg,
        };

        let icon_font_size = match self.size {
            Size::Sm => 15.0,
            Size::Md => 18.0,
            Size::Lg => 22.0,
        };

        let font_id = FontId::new(icon_font_size, FontFamily::Name(crate::fonts::PHOSPHOR.into()));
        let galley = ui
            .scope(|ui| {
                ui.style_mut().visuals.override_text_color = Some(label_color);
                self.icon.into_galley(ui, Some(egui::TextWrapMode::Extend), f32::INFINITY, font_id)
            })
            .inner;

        let desired = vec2(side, side);
        let sense = if self.enabled { Sense::click() } else { Sense::hover() };
        let (rect, mut response) = ui.allocate_exact_size(desired, sense);

        let (bg, border_color) = if self.enabled {
            let hover_t = ui
                .ctx()
                .animate_bool_with_time(response.id, response.hovered(), 0.10);
            let mut bg = mix(base, hover, hover_t);
            if response.is_pointer_button_down_on() {
                bg = active;
            }
            (bg, border)
        } else {
            let dis_bg = if self.variant == ButtonVariant::Ghost {
                Color32::TRANSPARENT
            } else {
                p.subtle
            };
            let dis_border = if self.variant == ButtonVariant::Ghost {
                Color32::TRANSPARENT
            } else {
                p.border
            };
            (dis_bg, dis_border)
        };

        let corner_r = match self.size {
            Size::Sm => m.radius_sm,
            Size::Md => m.radius_md,
            Size::Lg => m.radius_md + 2.0,
        };
        let rounding = radius(corner_r);

        ui.painter().rect_filled(rect, rounding, bg);
        if border_color != Color32::TRANSPARENT {
            ui.painter().rect_stroke(
                rect,
                rounding,
                Stroke::new(m.border_width, border_color),
                StrokeKind::Inside,
            );
        }

        let press_offset = if self.enabled && response.is_pointer_button_down_on() { 0.5 } else { 0.0 };
        let icon_pos = pos2(
            rect.center().x - galley.size().x / 2.0,
            rect.center().y - galley.size().y / 2.0 + press_offset,
        );
        ui.painter().galley(icon_pos, galley, label_color);

        // Focus ring (WCAG 2.4.7 / 2.4.11)
        if response.has_focus() {
            ui.painter().rect_stroke(
                rect.expand(m.focus_ring_offset),
                radius(corner_r + m.focus_ring_offset),
                Stroke::new(m.focus_ring_width, p.accent),
                StrokeKind::Outside,
            );
        }

        if let Some(tooltip) = self.tooltip {
            response = response.on_hover_text(tooltip);
        }

        response
    }
}
