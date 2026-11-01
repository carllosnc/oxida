//! Small painting/animation helpers shared by the widgets.

use egui::Color32;

/// Linearly interpolate between two colors in sRGB space.
///
/// Alpha-aware: used for fading disabled states as well.
pub(crate) fn mix(a: Color32, b: Color32, t: f32) -> Color32 {
    let t = t.clamp(0.0, 1.0);
    let (ar, ag, ab, aa) = a.to_tuple();
    let (br, bg, bb, ba) = b.to_tuple();
    let l = |x: u8, y: u8| (f32::from(x) + (f32::from(y) - f32::from(x)) * t).round() as u8;
    Color32::from_rgba_premultiplied(l(ar, br), l(ag, bg), l(ab, bb), l(aa, ba))
}

/// Convenience: `CornerRadius::same` with an `f32` token.
pub(crate) fn radius(r: f32) -> egui::CornerRadius {
    egui::CornerRadius::same(r.round() as u8)
}

/// Fade `color` towards `toward` by `amount` — used for disabled controls.
pub(crate) fn dim(color: Color32, toward: Color32, amount: f32) -> Color32 {
    mix(color, toward, amount)
}
