//! Spacing, sizing and rounding tokens.

/// Spacing, control sizes and corner radii.
///
/// Everything is derived from a 4&nbsp;px base grid, which keeps layouts
/// visually consistent without much thinking.
#[derive(Clone, Debug, PartialEq)]
pub struct Metrics {
    /// Base spacing grid unit. All paddings/margins are multiples of this.
    pub spacing_unit: f32,

    /// Height of small controls (badges, compact buttons).
    pub control_height_sm: f32,
    /// Height of default controls (buttons, inputs, sliders).
    pub control_height_md: f32,
    /// Height of large/primary controls.
    pub control_height_lg: f32,

    /// Corner radius for small elements (badges, tracks).
    pub radius_sm: f32,
    /// Corner radius for default controls (buttons, inputs).
    pub radius_md: f32,
    /// Corner radius for surfaces (cards, popups).
    pub radius_lg: f32,

    /// Border width for controls and surfaces.
    pub border_width: f32,

    /// Width of the keyboard focus ring stroke.
    pub focus_ring_width: f32,
    /// Offset distance of the keyboard focus ring from the element edge.
    pub focus_ring_offset: f32,
}

impl Default for Metrics {
    fn default() -> Self {
        Self {
            spacing_unit: 4.0,
            control_height_sm: 24.0,
            control_height_md: 32.0,
            control_height_lg: 40.0,
            radius_sm: 4.0,
            radius_md: 6.0,
            radius_lg: 10.0,
            border_width: 1.0,
            focus_ring_width: 1.5,
            focus_ring_offset: 2.0,
        }
    }
}

impl Metrics {
    /// Corner radius large enough for a pill/capsule shape for `height`.
    pub fn pill_radius(height: f32) -> f32 {
        height / 2.0
    }
}
