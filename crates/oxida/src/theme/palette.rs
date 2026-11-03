//! Semantic color tokens.

use egui::Color32;

/// Semantic colors used by every Oxida widget.
///
/// Instead of hard-coding colors, widgets read intent-based tokens
/// (`accent`, `danger`, `text_primary`, ...). This makes custom themes
/// easy: build a [`Palette`] with your own colors and you are done.
#[derive(Clone, Debug, PartialEq)]
pub struct Palette {
    /// Root background, behind everything (e.g. the central panel).
    pub window_bg: Color32,
    /// Background for panels, sidebars and cards.
    pub surface: Color32,
    /// Elevated background for floating overlays, popovers, dropdowns and selects.
    pub popover: Color32,
    /// Background for editable fields (text inputs, selects, etc).
    pub field_bg: Color32,
    /// Subtle background for secondary buttons, hover states and tracks.
    pub subtle: Color32,
    /// Default border color (used for dividers and separators).
    pub border: Color32,
    /// Stronger border, used for active dividers.
    pub border_strong: Color32,

    /// Main text color.
    pub text_primary: Color32,
    /// Secondary/supporting text color (labels, captions).
    pub text_secondary: Color32,
    /// Text color for disabled controls.
    pub text_disabled: Color32,

    /// Brand/primary accent (primary buttons, focus, selection).
    pub accent: Color32,
    /// Accent, hovered.
    pub accent_hover: Color32,
    /// Accent, pressed.
    pub accent_active: Color32,
    /// Text/icon color drawn on top of the accent.
    pub on_accent: Color32,

    /// Destructive actions.
    pub danger: Color32,
    /// Danger, hovered.
    pub danger_hover: Color32,
    /// Text/icon color drawn on top of danger.
    pub on_danger: Color32,

    /// Positive state (success messages, valid input, ...).
    pub success: Color32,
    /// Caution state (warnings).
    pub warning: Color32,
}

impl Palette {
    /// A clean, neutral light palette with soft contrasting surfaces.
    pub fn light() -> Self {
        Self {
            window_bg: Color32::from_rgb(250, 250, 250),
            surface: Color32::from_rgb(255, 255, 255),
            popover: Color32::from_rgb(255, 255, 255),
            field_bg: Color32::from_rgb(242, 242, 245),
            subtle: Color32::from_rgb(235, 235, 239),
            border: Color32::from_rgb(228, 228, 231),
            border_strong: Color32::from_rgb(161, 161, 170),

            text_primary: Color32::from_rgb(24, 24, 27),
            text_secondary: Color32::from_rgb(113, 113, 122),
            text_disabled: Color32::from_rgb(161, 161, 170),

            accent: Color32::from_rgb(24, 24, 27),
            accent_hover: Color32::from_rgb(63, 63, 70),
            accent_active: Color32::from_rgb(82, 82, 91),
            on_accent: Color32::from_rgb(250, 250, 250),

            danger: Color32::from_rgb(220, 38, 38),
            danger_hover: Color32::from_rgb(185, 28, 28),
            on_danger: Color32::from_rgb(255, 255, 255),

            success: Color32::from_rgb(22, 163, 74),
            warning: Color32::from_rgb(217, 119, 6),
        }
    }

    /// A clean, neutral dark palette with soft contrasting surfaces.
    pub fn dark() -> Self {
        Self {
            window_bg: Color32::from_rgb(16, 16, 18),
            surface: Color32::from_rgb(24, 24, 27),
            popover: Color32::from_rgb(36, 36, 42),
            field_bg: Color32::from_rgb(32, 32, 36),
            subtle: Color32::from_rgb(48, 48, 54),
            border: Color32::from_rgb(48, 48, 54),
            border_strong: Color32::from_rgb(85, 85, 92),

            text_primary: Color32::from_rgb(228, 228, 231),
            text_secondary: Color32::from_rgb(161, 161, 170),
            text_disabled: Color32::from_rgb(113, 113, 122),

            accent: Color32::from_rgb(228, 228, 231),
            accent_hover: Color32::from_rgb(244, 244, 245),
            accent_active: Color32::from_rgb(255, 255, 255),
            on_accent: Color32::from_rgb(24, 24, 27),

            danger: Color32::from_rgb(248, 113, 113),
            danger_hover: Color32::from_rgb(254, 160, 160),
            on_danger: Color32::from_rgb(24, 24, 27),

            success: Color32::from_rgb(74, 222, 128),
            warning: Color32::from_rgb(251, 146, 60),
        }
    }
}
