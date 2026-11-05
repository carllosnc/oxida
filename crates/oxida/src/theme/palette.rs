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
    /// Subtle border color for soft surfaces and cards.
    pub border_subtle: Color32,
    /// Stronger border, used for active dividers.
    pub border_strong: Color32,

    /// Main text color.
    pub text_primary: Color32,
    /// Secondary/supporting text color (labels, captions).
    pub text_secondary: Color32,
    /// Text color for disabled controls.
    pub text_disabled: Color32,

    /// Keyboard focus ring highlight.
    pub focus_ring: Color32,

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
            border_subtle: Color32::from_rgb(240, 240, 243),
            border_strong: Color32::from_rgb(161, 161, 170),

            text_primary: Color32::from_rgb(24, 24, 27),
            text_secondary: Color32::from_rgb(113, 113, 122),
            text_disabled: Color32::from_rgb(161, 161, 170),

            focus_ring: Color32::from_rgb(124, 58, 237),

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

    /// A clean, neutral dark palette aligned with Oxida design system foundations.
    pub fn dark() -> Self {
        Self {
            window_bg: Color32::from_rgb(0x14, 0x14, 0x16),     // #141416 - background/canvas
            surface: Color32::from_rgb(0x1C, 0x1C, 0x1F),       // #1C1C1F - surface/default
            popover: Color32::from_rgb(0x24, 0x24, 0x27),       // #242427 - surface/raised
            field_bg: Color32::from_rgb(0x1C, 0x1C, 0x1F),      // #1C1C1F
            subtle: Color32::from_rgb(0x24, 0x24, 0x27),        // #242427
            border: Color32::from_rgb(0x4B, 0x4B, 0x51),        // #4B4B51 - border/default
            border_subtle: Color32::from_rgb(0x30, 0x30, 0x34), // #303034 - border/subtle
            border_strong: Color32::from_rgb(0x71, 0x71, 0x7A),

            text_primary: Color32::from_rgb(0xF5, 0xF5, 0xF7),   // #F5F5F7 - text/primary
            text_secondary: Color32::from_rgb(0xA7, 0xA7, 0xB1), // #A7A7B1 - text/secondary
            text_disabled: Color32::from_rgb(0x85, 0x85, 0x8F),  // #85858F - text/disabled

            focus_ring: Color32::from_rgb(0xB6, 0xA4, 0xFF),     // #B6A4FF - focus/ring

            accent: Color32::from_rgb(0xF5, 0xF5, 0xF7),
            accent_hover: Color32::from_rgb(0xFF, 0xFF, 0xFF),
            accent_active: Color32::from_rgb(0xE4, 0xE4, 0xE7),
            on_accent: Color32::from_rgb(0x14, 0x14, 0x16),

            danger: Color32::from_rgb(0xAD, 0x28, 0x38),         // #AD2838 - button/destructive
            danger_hover: Color32::from_rgb(0xC4, 0x32, 0x44),
            on_danger: Color32::from_rgb(0xFF, 0xFF, 0xFF),

            success: Color32::from_rgb(0x73, 0xD6, 0xA4),        // #73D6A4 - status/success
            warning: Color32::from_rgb(0xF1, 0xCA, 0x72),        // #F1CA72 - status/warning
        }
    }
}
