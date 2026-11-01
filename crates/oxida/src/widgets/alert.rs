//! Themed Alert / Callout banner widget.

use egui::{pos2, vec2, FontFamily, FontId, Response, Sense, TextWrapMode, Ui, Widget, WidgetText};

use crate::icons;
use crate::theme;
use crate::util::{mix, radius};

/// Visual variant of an [`Alert`].
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AlertVariant {
    /// Neutral/informational alert (the default).
    #[default]
    Default,
    /// Success alert (positive completion, valid state).
    Success,
    /// Warning alert (caution, pending action).
    Warning,
    /// Destructive/error alert (errors, failures).
    Destructive,
}

/// An alert callout box styled after shadcn/ui.
///
/// ```
/// # use oxida::prelude::*;
/// # fn demo(ui: &mut egui::Ui) {
/// Alert::success("Configurações salvas")
///     .description("Suas preferências foram aplicadas com sucesso.")
///     .ui(ui);
/// # }
/// ```
#[derive(Clone)]
pub struct Alert {
    title: WidgetText,
    description: Option<WidgetText>,
    variant: AlertVariant,
    icon: Option<WidgetText>,
}

impl Alert {
    /// Create a new alert with the given title.
    pub fn new(title: impl Into<WidgetText>) -> Self {
        Self {
            title: title.into(),
            description: None,
            variant: AlertVariant::Default,
            icon: None,
        }
    }

    /// Informational / default alert.
    pub fn info(title: impl Into<WidgetText>) -> Self {
        Self::new(title).variant(AlertVariant::Default)
    }

    /// Success alert.
    pub fn success(title: impl Into<WidgetText>) -> Self {
        Self::new(title).variant(AlertVariant::Success)
    }

    /// Warning alert.
    pub fn warning(title: impl Into<WidgetText>) -> Self {
        Self::new(title).variant(AlertVariant::Warning)
    }

    /// Destructive / error alert.
    pub fn destructive(title: impl Into<WidgetText>) -> Self {
        Self::new(title).variant(AlertVariant::Destructive)
    }

    /// Set the alert variant.
    pub fn variant(mut self, variant: AlertVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Set an optional longer description text.
    pub fn description(mut self, description: impl Into<WidgetText>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Override the icon.
    pub fn icon(mut self, icon: impl Into<WidgetText>) -> Self {
        self.icon = Some(icon.into());
        self
    }
}

impl Widget for Alert {
    fn ui(self, ui: &mut Ui) -> Response {
        let theme = theme::current(ui.ctx());
        let p = &theme.palette;
        let m = &theme.metrics;
        let t = &theme.typography;

        let (bg, icon_color, default_icon) = match self.variant {
            AlertVariant::Default => (
                p.subtle,
                p.text_primary,
                icons::INFO,
            ),
            AlertVariant::Success => (
                mix(p.success, p.surface, 0.82),
                p.success,
                icons::CHECK_CIRCLE,
            ),
            AlertVariant::Warning => (
                mix(p.warning, p.surface, 0.82),
                p.warning,
                icons::WARNING,
            ),
            AlertVariant::Destructive => (
                mix(p.danger, p.surface, 0.82),
                p.danger,
                icons::WARNING_CIRCLE,
            ),
        };

        let pad_x = 16.0;
        let pad_y = 14.0;
        let icon_size = 18.0;
        let icon_gap = 12.0;

        let icon_str = self.icon.unwrap_or_else(|| default_icon.into());
        let icon_font = FontId::new(icon_size, FontFamily::Name(crate::fonts::PHOSPHOR.into()));
        let icon_galley = ui
            .scope(|ui| {
                ui.style_mut().visuals.override_text_color = Some(icon_color);
                icon_str.into_galley(ui, Some(egui::TextWrapMode::Extend), f32::INFINITY, icon_font)
            })
            .inner;

        let available_w = ui.available_width().max(100.0);
        let text_max_w = (available_w - pad_x * 2.0 - icon_size - icon_gap).max(60.0);

        let title_font = FontId::new(t.md, FontFamily::Name(crate::fonts::SEMI_BOLD.into()));
        let title_galley = ui
            .scope(|ui| {
                ui.style_mut().visuals.override_text_color = Some(p.text_primary);
                self.title.into_galley(ui, Some(TextWrapMode::Wrap), text_max_w, title_font)
            })
            .inner;

        let desc_galley = self.description.map(|desc| {
            let desc_font = FontId::new(t.sm, FontFamily::Name(crate::fonts::REGULAR.into()));
            ui.scope(|ui| {
                ui.style_mut().visuals.override_text_color = Some(p.text_secondary);
                desc.into_galley(ui, Some(TextWrapMode::Wrap), text_max_w, desc_font)
            })
            .inner
        });

        let mut content_h = title_galley.size().y;
        if let Some(ref dg) = desc_galley {
            content_h += 4.0 + dg.size().y;
        }
        let total_h = (content_h.max(icon_size) + pad_y * 2.0).max(44.0);

        let (rect, response) = ui.allocate_exact_size(vec2(available_w, total_h), Sense::hover());
        let rounding = radius(m.radius_lg);

        ui.painter().rect_filled(rect, rounding, bg);

        // Draw icon
        let icon_x = rect.left() + pad_x;
        let icon_y = rect.top() + pad_y + 1.0;
        ui.painter().galley(pos2(icon_x, icon_y), icon_galley, icon_color);

        // Draw title
        let text_x = icon_x + icon_size + icon_gap;
        let title_y = rect.top() + pad_y;
        let title_h = title_galley.size().y;
        ui.painter().galley(pos2(text_x, title_y), title_galley, p.text_primary);

        // Draw description if present
        if let Some(ref dg) = desc_galley {
            let desc_y = title_y + title_h + 4.0;
            ui.painter().galley(pos2(text_x, desc_y), dg.clone(), p.text_secondary);
        }

        response
    }
}
