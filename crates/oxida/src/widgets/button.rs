//! Themed button with variants and sizes.

use egui::{pos2, vec2, Color32, Response, Sense, Stroke, StrokeKind, Ui, Widget, WidgetText};

use crate::theme;
use crate::util::{mix, radius};
use super::Size;

/// Visual variant of a [`Button`].
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ButtonVariant {
    /// Accent background — the default, for the primary action.
    #[default]
    Primary,
    /// Surface background with a border.
    Secondary,
    /// Transparent background — low emphasis.
    Ghost,
    /// Danger background — destructive actions.
    Danger,
}

/// A themed button.
///
/// ```
/// # use oxida::prelude::*;
/// # fn demo(ui: &mut egui::Ui) {
/// ui.oxida().button("Save");
/// # }
/// ```
///
/// Or with full control:
///
/// ```
/// # use oxida::prelude::*;
/// # fn demo(ui: &mut egui::Ui) {
/// Button::new("Delete")
///     .variant(ButtonVariant::Danger)
///     .size(Size::Sm)
///     .ui(ui);
/// # }
/// ```
#[derive(Clone)]
pub struct Button {
    label: WidgetText,
    variant: ButtonVariant,
    size: Size,
    enabled: bool,
    fill_width: bool,
}

impl Button {
    /// Create a button with the given label.
    pub fn new(label: impl Into<WidgetText>) -> Self {
        Self {
            label: label.into(),
            variant: ButtonVariant::default(),
            size: Size::default(),
            enabled: true,
            fill_width: false,
        }
    }

    /// Set the visual variant. Defaults to [`ButtonVariant::Primary`].
    pub fn variant(mut self, variant: ButtonVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Set the control size. Defaults to [`Size::Md`].
    pub fn size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }

    /// Enable/disable the button. Disabled buttons don't respond to input.
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// Stretch the button to fill the available width.
    pub fn fill_width(mut self, fill_width: bool) -> Self {
        self.fill_width = fill_width;
        self
    }
}

impl Widget for Button {
    fn ui(self, ui: &mut Ui) -> Response {
        let theme = theme::current(ui.ctx());
        let p = &theme.palette;
        let m = &theme.metrics;

        let height = match self.size {
            Size::Sm => m.control_height_sm,
            Size::Md => m.control_height_md,
            Size::Lg => m.control_height_lg,
        };
        let pad_x = match self.size {
            Size::Sm => m.spacing_unit * 2.0,
            Size::Md => m.spacing_unit * 3.0,
            Size::Lg => m.spacing_unit * 4.0,
        };

        let font_id = theme.typography.font_id(self.size);
        let galley = self.label.into_galley(
            ui,
            Some(egui::TextWrapMode::Extend),
            f32::INFINITY,
            font_id,
        );

        let mut desired = vec2(galley.size().x + pad_x * 2.0, height);
        if self.fill_width {
            desired.x = ui.available_width().max(desired.x);
        }

        let sense = if self.enabled { Sense::click() } else { Sense::hover() };
        let (rect, response) = ui.allocate_exact_size(desired, sense);

        // Resolve colors for this variant ------------------------------------
        let (base, hover, active, fg, border) = match self.variant {
            ButtonVariant::Primary => (p.accent, p.accent_hover, p.accent_active, p.on_accent, p.accent),
            ButtonVariant::Secondary => (p.surface, p.subtle, p.border, p.text_primary, p.border),
            ButtonVariant::Ghost => (Color32::TRANSPARENT, p.subtle, p.border, p.text_primary, Color32::TRANSPARENT),
            ButtonVariant::Danger => (p.danger, p.danger_hover, mix(p.danger, Color32::BLACK, 0.2), p.on_danger, p.danger),
        };

        let (bg, fg) = if self.enabled {
            let hover_t = ui
                .ctx()
                .animate_bool_with_time(response.id, response.hovered(), 0.10);
            let mut bg = mix(base, hover, hover_t);
            if response.is_pointer_button_down_on() {
                bg = active;
            }
            (bg, fg)
        } else {
            (crate::util::dim(base, p.window_bg, 0.6), p.text_disabled)
        };

        // Paint ---------------------------------------------------------------
        let rounding = radius(m.radius_md.min(height / 2.0));
        ui.painter().rect_filled(rect, rounding, bg);
        if border != Color32::TRANSPARENT {
            ui.painter().rect_stroke(
                rect,
                rounding,
                Stroke::new(m.border_width, border),
                StrokeKind::Inside,
            );
        }

        let text_pos = pos2(
            rect.center().x - galley.size().x / 2.0,
            rect.center().y - galley.size().y / 2.0,
        );
        ui.painter().galley(text_pos, galley, fg);

        response
    }
}
