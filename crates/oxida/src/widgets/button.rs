//! Themed button with variants and sizes.

use std::f32::consts::TAU;
use egui::{pos2, vec2, Color32, Response, Sense, Stroke, StrokeKind, Ui, Widget, WidgetText};

use crate::theme;
use crate::util::{mix, paint_vertical_gradient_rounded_rect, radius};
use super::Size;

/// Visual variant of a [`Button`].
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ButtonVariant {
    /// Dark linear surface (or accent in light mode) — the default, for the primary action.
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
    icon: Option<WidgetText>,
    icon_after: Option<WidgetText>,
    variant: ButtonVariant,
    size: Size,
    enabled: bool,
    loading: bool,
    fill_width: bool,
}

impl Button {
    /// Create a button with the given label.
    pub fn new(label: impl Into<WidgetText>) -> Self {
        Self {
            label: label.into(),
            icon: None,
            icon_after: None,
            variant: ButtonVariant::default(),
            size: Size::default(),
            enabled: true,
            loading: false,
            fill_width: false,
        }
    }

    /// Primary button (accent background or dark surface gradient).
    pub fn primary(label: impl Into<WidgetText>) -> Self {
        Self::new(label).variant(ButtonVariant::Primary)
    }

    /// Secondary button (surface background with border).
    pub fn secondary(label: impl Into<WidgetText>) -> Self {
        Self::new(label).variant(ButtonVariant::Secondary)
    }

    /// Ghost button (transparent background until hovered).
    pub fn ghost(label: impl Into<WidgetText>) -> Self {
        Self::new(label).variant(ButtonVariant::Ghost)
    }

    /// Danger button (destructive actions).
    pub fn danger(label: impl Into<WidgetText>) -> Self {
        Self::new(label).variant(ButtonVariant::Danger)
    }

    /// Add a leading icon to the button.
    pub fn icon(mut self, icon: impl Into<WidgetText>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// Add a leading icon to the button (alias for [`Self::icon`]).
    pub fn leading_icon(self, icon: impl Into<WidgetText>) -> Self {
        self.icon(icon)
    }

    /// Add a trailing icon to the button (after the label).
    pub fn icon_after(mut self, icon: impl Into<WidgetText>) -> Self {
        self.icon_after = Some(icon.into());
        self
    }

    /// Add a trailing icon to the button (alias for [`Self::icon_after`]).
    pub fn trailing_icon(self, icon: impl Into<WidgetText>) -> Self {
        self.icon_after(icon)
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

    /// Set the button in loading state. Shows an animated spinner and disables interaction.
    pub fn loading(mut self, loading: bool) -> Self {
        self.loading = loading;
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

        // Resolve colors first: the text color must be known before layout,
        // because galley colors are baked in at layout time.
        // Figma spec: Primary button uses a vertical gradient (top to bottom).
        struct ButtonColors {
            base_top: Color32,
            base_bottom: Color32,
            hover_top: Color32,
            hover_bottom: Color32,
            active_top: Color32,
            active_bottom: Color32,
            fg: Color32,
            border: Color32,
        }

        let colors = if theme.dark_mode {
            match self.variant {
                ButtonVariant::Primary => ButtonColors {
                    // Figma spec: bg-gradient-to-b from-[#404040] to-[#272727]
                    base_top: Color32::from_rgb(0x40, 0x40, 0x40),
                    base_bottom: Color32::from_rgb(0x27, 0x27, 0x27),
                    // Hover: subtle elevation
                    hover_top: Color32::from_rgb(0x4C, 0x4C, 0x51),
                    hover_bottom: Color32::from_rgb(0x30, 0x30, 0x34),
                    // Active (pressed): recessed
                    active_top: Color32::from_rgb(0x28, 0x28, 0x2B),
                    active_bottom: Color32::from_rgb(0x20, 0x20, 0x22),
                    fg: Color32::WHITE,
                    border: p.border,
                },
                ButtonVariant::Secondary => ButtonColors {
                    base_top: p.popover,
                    base_bottom: p.popover,
                    hover_top: Color32::from_rgb(0x30, 0x30, 0x34),
                    hover_bottom: Color32::from_rgb(0x30, 0x30, 0x34),
                    active_top: Color32::from_rgb(0x1C, 0x1C, 0x1F),
                    active_bottom: Color32::from_rgb(0x1C, 0x1C, 0x1F),
                    fg: p.text_primary,
                    border: p.border,
                },
                ButtonVariant::Ghost => ButtonColors {
                    base_top: Color32::TRANSPARENT,
                    base_bottom: Color32::TRANSPARENT,
                    hover_top: p.subtle,
                    hover_bottom: p.subtle,
                    active_top: p.border_subtle,
                    active_bottom: p.border_subtle,
                    fg: p.text_primary,
                    border: Color32::TRANSPARENT,
                },
                ButtonVariant::Danger => ButtonColors {
                    base_top: p.danger,
                    base_bottom: p.danger,
                    hover_top: p.danger_hover,
                    hover_bottom: p.danger_hover,
                    active_top: Color32::from_rgb(0x8C, 0x1F, 0x2C),
                    active_bottom: Color32::from_rgb(0x8C, 0x1F, 0x2C),
                    fg: p.on_danger,
                    border: Color32::TRANSPARENT,
                },
            }
        } else {
            match self.variant {
                ButtonVariant::Primary => ButtonColors {
                    base_top: mix(p.accent, Color32::WHITE, 0.08),
                    base_bottom: p.accent,
                    hover_top: mix(p.accent_hover, Color32::WHITE, 0.08),
                    hover_bottom: p.accent_hover,
                    active_top: p.accent_active,
                    active_bottom: p.accent_active,
                    fg: p.on_accent,
                    border: p.accent,
                },
                ButtonVariant::Secondary => ButtonColors {
                    base_top: p.surface,
                    base_bottom: p.surface,
                    hover_top: p.subtle,
                    hover_bottom: p.subtle,
                    active_top: p.border,
                    active_bottom: p.border,
                    fg: p.text_primary,
                    border: p.border,
                },
                ButtonVariant::Ghost => ButtonColors {
                    base_top: Color32::TRANSPARENT,
                    base_bottom: Color32::TRANSPARENT,
                    hover_top: p.subtle,
                    hover_bottom: p.subtle,
                    active_top: p.border,
                    active_bottom: p.border,
                    fg: p.text_primary,
                    border: Color32::TRANSPARENT,
                },
                ButtonVariant::Danger => ButtonColors {
                    base_top: p.danger,
                    base_bottom: p.danger,
                    hover_top: p.danger_hover,
                    hover_bottom: p.danger_hover,
                    active_top: mix(p.danger, Color32::BLACK, 0.2),
                    active_bottom: mix(p.danger, Color32::BLACK, 0.2),
                    fg: p.on_danger,
                    border: p.danger,
                },
            }
        };

        let is_active = self.enabled && !self.loading;
        let label_color = if self.enabled {
            colors.fg
        } else {
            p.text_disabled
        };

        let height = match self.size {
            Size::Sm => m.control_height_sm,
            Size::Md => m.control_height_md,
            Size::Lg => m.control_height_lg,
        };
        let pad_x = match self.size {
            Size::Sm => 12.0,
            Size::Md => 16.0,
            Size::Lg => 20.0,
        };

        // Layout with `label_color` baked into the galley (the theme's
        // `override_text_color` would otherwise win over the paint color).
        let mut font_id = theme.typography.font_id(self.size);
        font_id.family = theme.typography.button_family.clone();

        let icon_font_size = match self.size {
            Size::Sm => 14.0,
            Size::Md => 16.0,
            Size::Lg => 18.0,
        };
        let icon_font_id = egui::FontId::new(icon_font_size, egui::FontFamily::Name(crate::fonts::PHOSPHOR.into()));

        let icon_galley = self.icon.map(|ic| {
            ui.scope(|ui| {
                ui.style_mut().visuals.override_text_color = Some(label_color);
                ic.into_galley(ui, Some(egui::TextWrapMode::Extend), f32::INFINITY, icon_font_id.clone())
            })
            .inner
        });

        let icon_after_galley = self.icon_after.map(|ic| {
            ui.scope(|ui| {
                ui.style_mut().visuals.override_text_color = Some(label_color);
                ic.into_galley(ui, Some(egui::TextWrapMode::Extend), f32::INFINITY, icon_font_id.clone())
            })
            .inner
        });

        let text_galley = ui
            .scope(|ui| {
                ui.style_mut().visuals.override_text_color = Some(label_color);
                self.label.into_galley(ui, Some(egui::TextWrapMode::Extend), f32::INFINITY, font_id)
            })
            .inner;

        let has_text = !text_galley.text().is_empty();
        let gap = 8.0;

        let spinner_size = match self.size {
            Size::Sm => 14.0,
            Size::Md => 16.0,
            Size::Lg => 18.0,
        };

        let mut content_width = 0.0;
        if self.loading {
            content_width += spinner_size;
            if has_text || icon_galley.is_some() {
                content_width += gap;
            }
        }
        if let Some(ref ig) = icon_galley {
            content_width += ig.size().x;
            if has_text {
                content_width += gap;
            }
        }
        if has_text {
            content_width += text_galley.size().x;
        }
        if let Some(ref tag) = icon_after_galley {
            if has_text || icon_galley.is_some() || self.loading {
                content_width += gap;
            }
            content_width += tag.size().x;
        }

        let mut desired = vec2(content_width + pad_x * 2.0, height);
        if self.fill_width {
            desired.x = ui.available_width().max(desired.x);
        }

        let sense = if is_active { Sense::click() } else { Sense::hover() };
        let (rect, response) = ui.allocate_exact_size(desired, sense);

        let (bg_top, bg_bottom, border_color) = if self.enabled {
            let hover_t = ui
                .ctx()
                .animate_bool_with_time(response.id, is_active && response.hovered(), 0.10);
            let mut top = mix(colors.base_top, colors.hover_top, hover_t);
            let mut bot = mix(colors.base_bottom, colors.hover_bottom, hover_t);
            if is_active && response.is_pointer_button_down_on() {
                top = colors.active_top;
                bot = colors.active_bottom;
            }
            (top, bot, colors.border)
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
            (dis_bg, dis_bg, dis_border)
        };

        // Paint ---------------------------------------------------------------
        let corner_r = match self.size {
            Size::Sm => m.radius_sm + 2.0, // 8px
            Size::Md => m.radius_md,       // 9px
            Size::Lg => m.radius_md + 2.0,
        };
        let rounding = radius(corner_r);

        paint_vertical_gradient_rounded_rect(
            ui.painter(),
            rect,
            corner_r,
            bg_top,
            bg_bottom,
        );
        if border_color != Color32::TRANSPARENT {
            ui.painter().rect_stroke(
                rect,
                rounding,
                Stroke::new(m.border_width, border_color),
                StrokeKind::Inside,
            );
        }

        // Subtle physical depression on press
        let press_offset = if is_active && response.is_pointer_button_down_on() { 0.5 } else { 0.0 };

        let mut curr_x = rect.center().x - content_width / 2.0;

        // Render loading spinner
        if self.loading {
            ui.ctx().request_repaint();
            let time = ui.input(|i| i.time) as f32;
            let start_angle = (time * 6.0) % TAU;
            let sweep = TAU * 0.70;
            let spinner_center = pos2(curr_x + spinner_size / 2.0, rect.center().y + press_offset);
            let spinner_radius = (spinner_size - 2.0) / 2.0;

            let stroke_w = if self.size == Size::Sm { 1.75 } else { 2.0 };
            // Faint track
            ui.painter().circle_stroke(
                spinner_center,
                spinner_radius,
                Stroke::new(stroke_w, mix(label_color, Color32::TRANSPARENT, 0.75)),
            );

            // Active arc
            let points = 16;
            let arc_pts: Vec<egui::Pos2> = (0..=points)
                .map(|i| {
                    let angle = start_angle + (i as f32 / points as f32) * sweep;
                    pos2(
                        spinner_center.x + spinner_radius * angle.cos(),
                        spinner_center.y + spinner_radius * angle.sin(),
                    )
                })
                .collect();

            for i in 0..arc_pts.len() - 1 {
                ui.painter().line_segment(
                    [arc_pts[i], arc_pts[i + 1]],
                    Stroke::new(stroke_w, label_color),
                );
            }

            curr_x += spinner_size;
            if has_text || icon_galley.is_some() {
                curr_x += gap;
            }
        }

        if let Some(ref ig) = icon_galley {
            let icon_pos = pos2(
                curr_x,
                rect.center().y - ig.size().y / 2.0 + press_offset,
            );
            ui.painter().galley(icon_pos, ig.clone(), label_color);
            curr_x += ig.size().x;
            if has_text {
                curr_x += gap;
            }
        }

        if has_text {
            let text_pos = pos2(
                curr_x,
                rect.center().y - text_galley.size().y / 2.0 + press_offset,
            );
            curr_x += text_galley.size().x;
            ui.painter().galley(text_pos, text_galley, label_color);
            if icon_after_galley.is_some() {
                curr_x += gap;
            }
        }

        if let Some(ref tag) = icon_after_galley {
            let icon_pos = pos2(
                curr_x,
                rect.center().y - tag.size().y / 2.0 + press_offset,
            );
            ui.painter().galley(icon_pos, tag.clone(), label_color);
        }

        // Focus ring (WCAG 2.4.7 / 2.4.11 / Figma focus/ring #B6A4FF)
        if response.has_focus() {
            ui.painter().rect_stroke(
                rect.expand(m.focus_ring_offset),
                radius(corner_r + m.focus_ring_offset),
                Stroke::new(m.focus_ring_width, p.focus_ring),
                StrokeKind::Outside,
            );
        }

        response
    }
}
