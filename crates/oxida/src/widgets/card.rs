//! Themed Card container widget.

use egui::{Color32, Frame, Margin, Stroke, Ui, WidgetText};

use crate::theme;
use crate::util::radius;

/// A surface card container styled after shadcn/ui.
///
/// Cards group related content and actions with a refined background,
/// border and corner radius.
///
/// ```
/// # use oxida::prelude::*;
/// # fn demo(ui: &mut egui::Ui) {
/// Card::new().show(ui, |ui| {
///     Card::header(ui, "Visão Geral", "Métricas principais da conta.");
///     ui.label("Conteúdo do card...");
/// });
/// # }
/// ```
#[derive(Clone, Debug)]
pub struct Card {
    fill: Option<Color32>,
    stroke: Option<Stroke>,
    radius: Option<f32>,
    inner_margin: Option<Margin>,
}

impl Default for Card {
    fn default() -> Self {
        Self::new()
    }
}

impl Card {
    /// Create a new card with default theme styles.
    pub fn new() -> Self {
        Self {
            fill: None,
            stroke: None,
            radius: None,
            inner_margin: None,
        }
    }

    /// Override the background fill color.
    pub fn fill(mut self, fill: Color32) -> Self {
        self.fill = Some(fill);
        self
    }

    /// Override the border stroke.
    pub fn stroke(mut self, stroke: Stroke) -> Self {
        self.stroke = Some(stroke);
        self
    }

    /// Override the corner radius.
    pub fn radius(mut self, radius: f32) -> Self {
        self.radius = Some(radius);
        self
    }

    /// Override the inner padding/margin.
    pub fn inner_margin(mut self, margin: impl Into<Margin>) -> Self {
        self.inner_margin = Some(margin.into());
        self
    }

    /// Render a standard card header with title and optional description.
    pub fn header(ui: &mut Ui, title: impl Into<WidgetText>, description: impl Into<WidgetText>) {
        let theme = theme::current(ui.ctx());
        let p = &theme.palette;
        let t = &theme.typography;

        ui.vertical(|ui| {
            ui.label(
                egui::RichText::new(title.into().text())
                    .size(t.lg - 4.0)
                    .family(t.heading_family.clone())
                    .color(p.text_primary),
            );
            ui.add_space(2.0);
            ui.add(
                egui::Label::new(
                    egui::RichText::new(description.into().text())
                        .size(t.sm)
                        .color(p.text_secondary),
                )
                .wrap_mode(egui::TextWrapMode::Wrap),
            );
            ui.add_space(14.0);
        });
    }

    /// Show the card and its content closure.
    pub fn show<R>(self, ui: &mut Ui, add_contents: impl FnOnce(&mut Ui) -> R) -> egui::InnerResponse<R> {
        let theme = theme::current(ui.ctx());
        let p = &theme.palette;
        let m = &theme.metrics;

        let fill = self.fill.unwrap_or(p.surface);
        let stroke = self.stroke.unwrap_or(Stroke::NONE);
        let corner_r = self.radius.unwrap_or(m.radius_lg);
        let margin = self.inner_margin.unwrap_or_else(|| {
            let pad = (m.spacing_unit * 5.0).round() as i8;
            Margin::symmetric(pad, pad)
        });

        let mut frame = Frame::new()
            .fill(fill)
            .corner_radius(radius(corner_r))
            .inner_margin(margin);

        if stroke != Stroke::NONE {
            frame = frame.stroke(stroke);
        }

        frame.show(ui, add_contents)
    }
}
