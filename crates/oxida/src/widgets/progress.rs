//! Themed Progress Bar and Spinner widgets.

use std::f32::consts::TAU;

use egui::{pos2, vec2, Color32, Response, Sense, Stroke, Ui, Widget};

use super::Size;
use crate::theme;
use crate::util::radius;

/// A linear progress indicator styled after shadcn/ui.
///
/// ```
/// # use oxida::prelude::*;
/// # fn demo(ui: &mut egui::Ui) {
/// ProgressBar::new(0.65).ui(ui);
/// # }
/// ```
#[derive(Clone, Debug)]
pub struct ProgressBar {
    progress: f32,
    height: Option<f32>,
    fill: Option<Color32>,
}

impl ProgressBar {
    /// Create a progress bar with a progress value in `0.0..=1.0`.
    pub fn new(progress: f32) -> Self {
        Self {
            progress: progress.clamp(0.0, 1.0),
            height: None,
            fill: None,
        }
    }

    /// Custom height in pixels. Defaults to `8.0`.
    pub fn height(mut self, height: f32) -> Self {
        self.height = Some(height);
        self
    }

    /// Custom fill color. Defaults to the theme accent.
    pub fn fill(mut self, fill: Color32) -> Self {
        self.fill = Some(fill);
        self
    }
}

impl Widget for ProgressBar {
    fn ui(self, ui: &mut Ui) -> Response {
        let theme = theme::current(ui.ctx());
        let p = &theme.palette;

        let height = self.height.unwrap_or(8.0);
        let fill_color = self.fill.unwrap_or(p.accent);

        let width = ui.available_width().max(40.0);
        let (rect, response) = ui.allocate_exact_size(vec2(width, height), Sense::hover());

        let rounding = radius(height / 2.0);

        // Track
        ui.painter().rect_filled(rect, rounding, p.subtle);

        // Fill bar
        let filled_w = (rect.width() * self.progress).max(0.0);
        if filled_w > 0.0 {
            let filled_rect = egui::Rect::from_min_size(rect.min, vec2(filled_w, height));
            ui.painter().rect_filled(filled_rect, rounding, fill_color);
        }

        response
    }
}

/// An indeterminate circular loading spinner.
///
/// ```
/// # use oxida::prelude::*;
/// # fn demo(ui: &mut egui::Ui) {
/// Spinner::new().size(Size::Md).ui(ui);
/// # }
/// ```
#[derive(Clone, Copy, Debug, Default)]
pub struct Spinner {
    size: Size,
    color: Option<Color32>,
}

impl Spinner {
    /// Create a new spinner. Defaults to [`Size::Md`].
    pub fn new() -> Self {
        Self {
            size: Size::Md,
            color: None,
        }
    }

    /// Set the size of the spinner.
    pub fn size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }

    /// Override the spinner stroke color.
    pub fn color(mut self, color: Color32) -> Self {
        self.color = Some(color);
        self
    }
}

impl Widget for Spinner {
    fn ui(self, ui: &mut Ui) -> Response {
        let theme = theme::current(ui.ctx());
        let p = &theme.palette;

        let (dim, stroke_w) = match self.size {
            Size::Sm => (16.0, 2.0),
            Size::Md => (22.0, 2.5),
            Size::Lg => (30.0, 3.0),
        };

        let color = self.color.unwrap_or(p.accent);
        let track_color = p.subtle;

        let (rect, response) = ui.allocate_exact_size(vec2(dim, dim), Sense::hover());

        // Keep animating
        ui.ctx().request_repaint();

        let time = ui.input(|i| i.time) as f32;
        let start_angle = (time * 4.0) % TAU;
        let sweep = TAU * 0.75;

        let center = rect.center();
        let radius = (dim - stroke_w) / 2.0;

        // Faint track ring
        ui.painter().circle_stroke(
            center,
            radius,
            Stroke::new(stroke_w, track_color),
        );

        // Animated arc
        let points = 24;
        let arc_pts: Vec<egui::Pos2> = (0..=points)
            .map(|i| {
                let angle = start_angle + (i as f32 / points as f32) * sweep;
                pos2(
                    center.x + radius * angle.cos(),
                    center.y + radius * angle.sin(),
                )
            })
            .collect();

        for i in 0..arc_pts.len() - 1 {
            ui.painter().line_segment(
                [arc_pts[i], arc_pts[i + 1]],
                Stroke::new(stroke_w, color),
            );
        }

        response
    }
}
