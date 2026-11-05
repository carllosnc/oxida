//! Themed Accordion / Collapsible section widget.

use egui::{
    pos2, vec2, Color32, CornerRadius, FontFamily, FontId, Id, Margin, Response, Sense,
    Stroke, Ui, WidgetText,
};

use super::Size;
use crate::theme;
use crate::util::{mix, radius};

/// Visual variant of an [`Accordion`].
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AccordionVariant {
    /// Borderless item with subtle hover feedback and an optional bottom divider line (shadcn/ui style).
    #[default]
    Default,
    /// Enclosed in a soft surface card container (`theme.palette.surface`).
    Card,
    /// Minimal ghost item without background or bottom divider.
    Ghost,
}

/// The response returned after rendering an [`Accordion`].
pub struct AccordionResponse<R> {
    /// The response from interacting with the accordion header trigger.
    pub header_response: Response,
    /// The return value of the content closure, if it was executed.
    pub inner: Option<R>,
    /// Whether the accordion is currently open.
    pub is_open: bool,
    /// The animated openness factor (`0.0` when fully closed, `1.0` when fully open).
    pub openness: f32,
}

impl<R> AccordionResponse<R> {
    /// Was the header clicked this frame?
    pub fn clicked(&self) -> bool {
        self.header_response.clicked()
    }

    /// Is the header currently hovered?
    pub fn hovered(&self) -> bool {
        self.header_response.hovered()
    }

    /// Does the header currently have keyboard focus?
    pub fn has_focus(&self) -> bool {
        self.header_response.has_focus()
    }
}

/// An expandable/collapsible accordion widget styled after shadcn/ui and modern desktop OSes.
///
/// Features:
/// - Supports both uncontrolled (automatic internal state) and controlled (`&mut bool`) modes.
/// - Smooth animated chevron rotation and content opacity transition.
/// - Rich header: title, optional subtitle, leading Phosphor icon, and tag badge.
/// - Full keyboard accessibility (Focus ring, `Enter` / `Space` to toggle, `ArrowRight` to open, `ArrowLeft` to close).
/// - Three visual variants: [`AccordionVariant::Default`], [`AccordionVariant::Card`], and [`AccordionVariant::Ghost`].
///
/// ```
/// # use oxida::prelude::*;
/// # fn demo(ui: &mut egui::Ui) {
/// Accordion::new("Geral")
///     .subtitle("Preferências e configurações do aplicativo")
///     .show(ui, |ui| {
///         ui.label("Conteúdo da seção geral...");
///     });
/// # }
/// ```
pub struct Accordion<'a> {
    id_source: Option<Id>,
    title: WidgetText,
    subtitle: Option<WidgetText>,
    icon: Option<WidgetText>,
    badge: Option<WidgetText>,
    variant: AccordionVariant,
    size: Size,
    default_open: bool,
    open_state: Option<&'a mut bool>,
    show_divider: bool,
    enabled: bool,
}

impl Accordion<'static> {
    /// Create a new accordion with the given title.
    pub fn new(title: impl Into<WidgetText>) -> Self {
        Self {
            id_source: None,
            title: title.into(),
            subtitle: None,
            icon: None,
            badge: None,
            variant: AccordionVariant::Default,
            size: Size::Md,
            default_open: false,
            open_state: None,
            show_divider: true,
            enabled: true,
        }
    }

    /// Create an accordion styled as a soft surface card ([`AccordionVariant::Card`]).
    pub fn card(title: impl Into<WidgetText>) -> Self {
        Self::new(title).variant(AccordionVariant::Card)
    }

    /// Create a minimal ghost accordion ([`AccordionVariant::Ghost`]).
    pub fn ghost(title: impl Into<WidgetText>) -> Self {
        Self::new(title).variant(AccordionVariant::Ghost)
    }
}

impl<'a> Accordion<'a> {
    /// Bind the accordion to an external mutable boolean (controlled state).
    pub fn open(self, is_open: &'a mut bool) -> Accordion<'a> {
        Accordion {
            id_source: self.id_source,
            title: self.title,
            subtitle: self.subtitle,
            icon: self.icon,
            badge: self.badge,
            variant: self.variant,
            size: self.size,
            default_open: self.default_open,
            open_state: Some(is_open),
            show_divider: self.show_divider,
            enabled: self.enabled,
        }
    }

    /// Override the unique ID salt for egui state storage and animations.
    pub fn id_source(mut self, id_source: impl std::hash::Hash + std::fmt::Debug) -> Self {
        self.id_source = Some(Id::unique(id_source));
        self
    }

    /// Optional subtitle/description rendered beneath the title.
    pub fn subtitle(mut self, subtitle: impl Into<WidgetText>) -> Self {
        self.subtitle = Some(subtitle.into());
        self
    }

    /// Optional leading Phosphor icon.
    pub fn icon(mut self, icon: impl Into<WidgetText>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// Optional badge/tag label displayed alongside the title.
    pub fn badge(mut self, badge: impl Into<WidgetText>) -> Self {
        self.badge = Some(badge.into());
        self
    }

    /// Set the visual variant. Defaults to [`AccordionVariant::Default`].
    pub fn variant(mut self, variant: AccordionVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Set the control size. Defaults to [`Size::Md`].
    pub fn size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }

    /// Whether the accordion is open by default when in uncontrolled mode.
    pub fn default_open(mut self, default_open: bool) -> Self {
        self.default_open = default_open;
        self
    }

    /// Whether to display a subtle divider line.
    pub fn divider(mut self, show: bool) -> Self {
        self.show_divider = show;
        self
    }

    /// Enable or disable user interaction.
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// Show the accordion header and conditionally execute `add_contents` when expanded.
    pub fn show<R>(
        mut self,
        ui: &mut Ui,
        add_contents: impl FnOnce(&mut Ui) -> R,
    ) -> AccordionResponse<R> {
        let theme = theme::current(ui.ctx());
        let p = &theme.palette;
        let m = &theme.metrics;
        let t = &theme.typography;

        // Determine widget ID
        let id = self.id_source.unwrap_or_else(|| {
            ui.make_persistent_id(self.title.text())
        });

        // Determine open state
        let mut is_open = if let Some(ref external) = self.open_state {
            **external
        } else {
            ui.data_mut(|d| d.get_temp::<bool>(id).unwrap_or(self.default_open))
        };

        // Smooth 200ms ease animation
        let openness = ui.ctx().animate_bool_with_time(id, is_open, 0.20);

        let is_card = self.variant == AccordionVariant::Card;

        let outer_corner_r = if is_card { m.radius_lg } else { m.radius_md };

        // Frame wrapper for Card variant
        let outer_frame = if is_card {
            egui::Frame::new()
                .fill(p.surface)
                .corner_radius(radius(outer_corner_r))
                .inner_margin(Margin::same(0))
        } else {
            egui::Frame::new().inner_margin(Margin::same(0))
        };

        let mut inner_result: Option<R> = None;
        let mut header_resp: Option<Response> = None;

        outer_frame.show(ui, |ui| {
            // Compute heights & font sizes
            let (title_size, header_min_h, icon_size, pad_x) = match self.size {
                Size::Sm => (t.sm, 34.0, 14.0, 10.0),
                Size::Md => (t.md, 40.0, 16.0, 12.0),
                Size::Lg => (t.md + 2.0, 48.0, 18.0, 14.0),
            };

            let title_font = FontId::new(title_size, FontFamily::Name(crate::fonts::SEMI_BOLD.into()));
            let sub_font = FontId::new(t.xs, FontFamily::Name(crate::fonts::REGULAR.into()));
            let icon_font = FontId::new(icon_size, FontFamily::Name(crate::fonts::PHOSPHOR.into()));
            let badge_font = FontId::new(t.xs - 1.0, FontFamily::Name(crate::fonts::MEDIUM.into()));

            let available_w = ui.available_width().max(120.0);
            let right_reserved = pad_x + 16.0 + 8.0; // chevron width & margins
            let text_max_w = (available_w - pad_x * 2.0 - right_reserved).max(40.0);

            // Galleys
            let title_color = if !self.enabled {
                p.text_disabled
            } else {
                p.text_primary
            };

            let title_galley = ui.scope(|ui| {
                ui.style_mut().visuals.override_text_color = Some(title_color);
                self.title.clone().into_galley(ui, Some(egui::TextWrapMode::Truncate), text_max_w, title_font)
            }).inner;

            let sub_galley = if let Some(ref sub) = self.subtitle {
                let galley = ui.scope(|ui| {
                    ui.style_mut().visuals.override_text_color = Some(p.text_secondary);
                    sub.clone().into_galley(ui, Some(egui::TextWrapMode::Truncate), text_max_w, sub_font)
                }).inner;
                Some(galley)
            } else {
                None
            };

            let icon_galley = if let Some(ref icon) = self.icon {
                let icon_color = if !self.enabled {
                    p.text_disabled
                } else {
                    p.text_secondary
                };
                let galley = ui.scope(|ui| {
                    ui.style_mut().visuals.override_text_color = Some(icon_color);
                    icon.clone().into_galley(ui, Some(egui::TextWrapMode::Extend), f32::INFINITY, icon_font)
                }).inner;
                Some(galley)
            } else {
                None
            };

            let badge_galley = if let Some(ref badge) = self.badge {
                let galley = ui.scope(|ui| {
                    ui.style_mut().visuals.override_text_color = Some(p.text_secondary);
                    badge.clone().into_galley(ui, Some(egui::TextWrapMode::Extend), f32::INFINITY, badge_font)
                }).inner;
                Some(galley)
            } else {
                None
            };

            // Calculate header height
            let text_block_h = if let Some(ref sg) = sub_galley {
                title_galley.size().y + 2.0 + sg.size().y
            } else {
                title_galley.size().y
            };
            let header_h = (text_block_h + 16.0).max(header_min_h);

            // Allocate header trigger
            let sense = if self.enabled {
                Sense::click()
            } else {
                Sense::hover()
            };
            let (header_rect, response) = ui.allocate_exact_size(vec2(available_w, header_h), sense);

            // Handle user interaction
            let mut toggled = response.clicked();
            if self.enabled && response.has_focus() {
                let enter_or_space = ui.input(|i| {
                    i.key_pressed(egui::Key::Enter) || i.key_pressed(egui::Key::Space)
                });
                let arrow_right = ui.input(|i| i.key_pressed(egui::Key::ArrowRight));
                let arrow_left = ui.input(|i| i.key_pressed(egui::Key::ArrowLeft));

                if enter_or_space || (arrow_right && !is_open) || (arrow_left && is_open) {
                    toggled = true;
                }
            }

            if toggled {
                is_open = !is_open;
                if let Some(ref mut external) = self.open_state {
                    **external = is_open;
                } else {
                    ui.data_mut(|d| d.insert_temp(id, is_open));
                }
            }

            // Paint Header Background
            let painter = ui.painter();
            let hover_t = ui.ctx().animate_bool_with_time(response.id.with("hover"), response.hovered() && self.enabled, 0.12);

            let header_rounding = if is_card {
                if openness > 0.05 {
                    CornerRadius {
                        nw: outer_corner_r as u8,
                        ne: outer_corner_r as u8,
                        sw: 0,
                        se: 0,
                    }
                } else {
                    radius(outer_corner_r)
                }
            } else {
                radius(m.radius_md)
            };

            if hover_t > 0.0 {
                let hover_bg = if is_card {
                    mix(Color32::TRANSPARENT, mix(p.surface, p.text_secondary, 0.08), hover_t)
                } else {
                    mix(Color32::TRANSPARENT, p.subtle, hover_t)
                };
                painter.rect_filled(header_rect, header_rounding, hover_bg);
            }

            // Focus ring
            if response.has_focus() {
                let focus_stroke = Stroke::new(1.5, p.accent);
                painter.rect_stroke(header_rect.shrink(0.5), header_rounding, focus_stroke, egui::StrokeKind::Inside);
            }

            // Paint leading icon
            let mut cursor_x = header_rect.left() + pad_x;
            if let Some(ref ig) = icon_galley {
                let icon_pos = pos2(cursor_x, header_rect.center().y - ig.size().y * 0.5);
                let ic_color = if response.hovered() && self.enabled {
                    p.text_primary
                } else {
                    p.text_secondary
                };
                painter.galley(icon_pos, ig.clone(), ic_color);
                cursor_x += ig.size().x + 8.0;
            }

            // Paint title & subtitle
            let title_pos = if let Some(ref sg) = sub_galley {
                let block_y = header_rect.center().y - text_block_h * 0.5;
                let sub_pos = pos2(cursor_x, block_y + title_galley.size().y + 2.0);
                painter.galley(sub_pos, sg.clone(), p.text_secondary);
                pos2(cursor_x, block_y)
            } else {
                pos2(cursor_x, header_rect.center().y - title_galley.size().y * 0.5)
            };

            let active_title_color = if response.hovered() && self.enabled {
                p.text_primary
            } else {
                title_color
            };
            painter.galley(title_pos, title_galley.clone(), active_title_color);
            cursor_x += title_galley.size().x;

            // Paint optional badge
            if let Some(ref bg) = badge_galley {
                let badge_x = cursor_x + 8.0;
                let badge_pad_x = 6.0;
                let badge_pad_y = 2.0;
                let badge_w = bg.size().x + badge_pad_x * 2.0;
                let badge_h = bg.size().y + badge_pad_y * 2.0;
                let badge_rect = egui::Rect::from_min_size(
                    pos2(badge_x, title_pos.y + (title_galley.size().y - badge_h) * 0.5),
                    vec2(badge_w, badge_h),
                );
                painter.rect_filled(badge_rect, radius(m.radius_sm), p.subtle);
                painter.galley(
                    pos2(badge_rect.left() + badge_pad_x, badge_rect.top() + badge_pad_y),
                    bg.clone(),
                    p.text_secondary,
                );
            }

            // Paint Rotating Chevron Indicator
            let chevron_center = pos2(header_rect.right() - pad_x - 4.0, header_rect.center().y);
            let chevron_color = if !self.enabled {
                p.text_disabled
            } else if response.hovered() || is_open {
                p.text_primary
            } else {
                p.text_secondary
            };

            paint_rotating_chevron(painter, chevron_center, openness, chevron_color);

            // Card internal divider when open
            if is_card && openness > 0.0 && self.show_divider {
                let div_alpha = (openness * 255.0).round() as u8;
                let div_color = Color32::from_rgba_unmultiplied(
                    p.border.r(),
                    p.border.g(),
                    p.border.b(),
                    ((p.border.a() as f32 / 255.0) * (div_alpha as f32 / 255.0) * 160.0) as u8,
                );
                let sep_stroke = Stroke::new(1.0, div_color);
                painter.line_segment(
                    [
                        pos2(header_rect.left(), header_rect.bottom()),
                        pos2(header_rect.right(), header_rect.bottom()),
                    ],
                    sep_stroke,
                );
            }

            header_resp = Some(response);

            // Render Content
            if openness > 0.0 {
                ui.scope(|ui| {
                    if openness < 1.0 {
                        ui.set_opacity(openness);
                    }

                    let content_pad_left = if is_card {
                        pad_x
                    } else if icon_galley.is_some() {
                        pad_x + icon_size + 8.0
                    } else {
                        pad_x
                    };

                    let content_margin = Margin {
                        left: content_pad_left as i8,
                        right: pad_x as i8,
                        top: if is_card { 10 } else { 4 },
                        bottom: if is_card { 12 } else { 8 },
                    };

                    egui::Frame::new()
                        .inner_margin(content_margin)
                        .show(ui, |ui| {
                            inner_result = Some(add_contents(ui));
                        });
                });
            }

            // Default variant bottom divider line
            if !is_card && self.show_divider && self.variant == AccordionVariant::Default {
                ui.add_space(2.0);
                let sep_rect = egui::Rect::from_min_size(
                    pos2(header_rect.left(), ui.cursor().top()),
                    vec2(available_w, 1.0),
                );
                let sep_color = mix(p.border, p.surface, 0.4);
                ui.painter().line_segment(
                    [sep_rect.left_top(), sep_rect.right_top()],
                    Stroke::new(1.0, sep_color),
                );
                ui.add_space(4.0);
            }
        });

        AccordionResponse {
            header_response: header_resp.expect("header response always produced"),
            inner: inner_result,
            is_open,
            openness,
        }
    }
}

/// Paint a clean vector chevron that smoothly rotates 180° based on `openness`.
fn paint_rotating_chevron(
    painter: &egui::Painter,
    center: egui::Pos2,
    openness: f32,
    color: Color32,
) {
    let angle = openness * std::f32::consts::PI;
    let cos_a = angle.cos();
    let sin_a = angle.sin();

    let span = 3.6;
    let half_h = 2.0;

    let rot = |dx: f32, dy: f32| -> egui::Vec2 {
        vec2(
            dx * cos_a - dy * sin_a,
            dx * sin_a + dy * cos_a,
        )
    };

    let p_left = center + rot(-span, -half_h);
    let p_tip = center + rot(0.0, half_h);
    let p_right = center + rot(span, -half_h);

    let stroke = Stroke::new(1.4, color);
    painter.line_segment([p_left, p_tip], stroke);
    painter.line_segment([p_tip, p_right], stroke);
}

/// An accordion group controller for single-expanded (exclusive) accordions.
///
/// In an [`AccordionGroup`], expanding one item automatically collapses any previously open item.
///
/// ```
/// # use oxida::prelude::*;
/// # fn demo(ui: &mut egui::Ui) {
/// let mut active_section: Option<usize> = Some(0);
/// let mut group = AccordionGroup::new(&mut active_section);
///
/// group.item(0, "Seção 1", ui, |ui| {
///     ui.label("Conteúdo da seção 1");
/// });
/// group.item(1, "Seção 2", ui, |ui| {
///     ui.label("Conteúdo da seção 2");
/// });
/// # }
/// ```
pub struct AccordionGroup<'a, T: PartialEq + Clone> {
    active_key: &'a mut Option<T>,
}

impl<'a, T: PartialEq + Clone> AccordionGroup<'a, T> {
    /// Create a new exclusive accordion group bound to `active_key`.
    pub fn new(active_key: &'a mut Option<T>) -> Self {
        Self { active_key }
    }

    /// Render an exclusive item belonging to this group.
    pub fn item<R>(
        &mut self,
        key: T,
        title: impl Into<WidgetText>,
        ui: &mut Ui,
        add_contents: impl FnOnce(&mut Ui) -> R,
    ) -> AccordionResponse<R> {
        let is_current = self.active_key.as_ref() == Some(&key);
        let mut open_flag = is_current;
        let accordion = Accordion::new(title).open(&mut open_flag);
        let res = accordion.show(ui, add_contents);

        if res.clicked() {
            if is_current {
                *self.active_key = None;
            } else {
                *self.active_key = Some(key);
            }
        }
        res
    }
}
