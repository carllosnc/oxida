//! Themed single-line text input.

use egui::{Margin, Response, Stroke, StrokeKind, TextEdit, TextStyle, Ui, Vec2, Widget, WidgetText};

use crate::theme;
use crate::util::radius;

/// A themed single-line text input.
///
/// Wraps [`egui::TextEdit`] (keeping caret, selection, IME and undo for free)
/// inside a themed frame with a focus ring.
///
/// ```
/// # use oxida::prelude::*;
/// # fn demo(ui: &mut egui::Ui, mut name: String) {
/// ui.oxida()
///     .text_input(&mut name)
///     .placeholder("Your name")
///     .ui(ui);
/// # }
/// ```
pub struct TextInput<'t> {
    text: &'t mut String,
    placeholder: Option<WidgetText>,
    id_salt: Option<egui::Id>,
    password: bool,
}

impl<'t> TextInput<'t> {
    /// Create an input bound to `text`.
    pub fn new(text: &'t mut String) -> Self {
        Self {
            text,
            placeholder: None,
            id_salt: None,
            password: false,
        }
    }

    /// Hint text shown while the input is empty.
    pub fn placeholder(mut self, placeholder: impl Into<WidgetText>) -> Self {
        self.placeholder = Some(placeholder.into());
        self
    }

    /// Explicit identity — useful when inputs are created in loops.
    pub fn id_salt(mut self, salt: egui::Id) -> Self {
        self.id_salt = Some(salt);
        self
    }

    /// Mask the contents (password field).
    pub fn password(mut self, password: bool) -> Self {
        self.password = password;
        self
    }
}

impl Widget for TextInput<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let theme = theme::current(ui.ctx());
        let p = &theme.palette;
        let m = &theme.metrics;

        let frame = egui::Frame {
            inner_margin: Margin::symmetric(
                (m.spacing_unit * 2.5) as i8,
                (m.spacing_unit * 1.5) as i8,
            ),
            fill: p.field_bg,
            stroke: Stroke::new(m.border_width, p.border),
            corner_radius: radius(m.radius_md),
            ..Default::default()
        };

        let mut edit = TextEdit::singleline(self.text)
            .frame(egui::Frame::new())
            .font(TextStyle::Body)
            .desired_width(f32::INFINITY)
            .margin(Vec2::ZERO)
            .password(self.password);
        if let Some(placeholder) = self.placeholder {
            edit = edit.hint_text(placeholder);
        }
        if let Some(salt) = self.id_salt {
            edit = edit.id_salt(salt);
        }

        let inner = frame.show(ui, |ui| edit.ui(ui));

        // Focus ring, painted over the frame border (outside the text area).
        if inner.inner.has_focus() {
            ui.painter().rect_stroke(
                inner.response.rect,
                radius(m.radius_md),
                Stroke::new(1.5, p.accent),
                StrokeKind::Inside,
            );
        }

        inner.inner
    }
}
