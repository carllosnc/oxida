//! Bundled [Inter](https://rsms.me/inter/) font and installation helper.
//!
//! Inter is licensed under the SIL Open Font License 1.1 — see
//! `assets/fonts/LICENSE-Inter-OFL.txt`.

use std::sync::Arc;

use egui::epaint::text::{HintingTarget, SmoothHinting};
use egui::{FontData, FontDefinitions, FontFamily, FontTweak};

/// Family name of the Inter Regular weight (the default).
pub const REGULAR: &str = "Inter";
/// Family name of the Inter Medium weight.
pub const MEDIUM: &str = "Inter Medium";
/// Family name of the Inter SemiBold weight (used for headings).
pub const SEMI_BOLD: &str = "Inter SemiBold";
/// Family name of the Inter Bold weight.
pub const BOLD: &str = "Inter Bold";

/// Family name of the Phosphor Regular icon font.
pub const PHOSPHOR: &str = "Phosphor";
/// Family name of the Phosphor Bold icon font.
pub const PHOSPHOR_BOLD: &str = "Phosphor Bold";
/// Family name of the Phosphor Fill icon font.
pub const PHOSPHOR_FILL: &str = "Phosphor Fill";

const REGULAR_BYTES: &[u8] = include_bytes!("../assets/fonts/Inter-Regular.ttf");
const MEDIUM_BYTES: &[u8] = include_bytes!("../assets/fonts/Inter-Medium.ttf");
const SEMI_BOLD_BYTES: &[u8] = include_bytes!("../assets/fonts/Inter-SemiBold.ttf");
const BOLD_BYTES: &[u8] = include_bytes!("../assets/fonts/Inter-Bold.ttf");

const PHOSPHOR_BYTES: &[u8] = include_bytes!("../assets/fonts/Phosphor.ttf");
const PHOSPHOR_BOLD_BYTES: &[u8] = include_bytes!("../assets/fonts/Phosphor-Bold.ttf");
const PHOSPHOR_FILL_BYTES: &[u8] = include_bytes!("../assets/fonts/Phosphor-Fill.ttf");

/// Install Inter as the default proportional font of `ctx`.
///
/// * Proportional text (body, buttons, inputs) renders in Inter Regular.
/// * [`FontFamily::Name`] with [`MEDIUM`], [`SEMI_BOLD`] and [`BOLD`]
///   address the heavier weights.
/// * egui's built-in fonts are kept as glyph fallback (emoji, etc).
/// * Every weight gets an aggressive hinting target (position-dependent,
///   horizontally grid-fitted) so stems stay crisp on low-DPI displays.
///
/// Called automatically by [`crate::init`]; call it yourself if you apply
/// themes manually with [`crate::Theme::apply`].
pub fn install(ctx: &egui::Context) {
    let mut defs = FontDefinitions::default();

    // Light, vertical-only hinting that keeps the designed advances.
    //
    // Full grid-fitting (`light: false`) plus snapping advances to whole
    // pixels (`preserve_linear_metrics: false`) distorts Inter's stems and
    // spacing, which reads as jagged/"pixelated" text. Light hinting keeps
    // baselines and x-height sharp while the glyph shapes stay faithful.
    let crisp = FontTweak {
        hinting_target: HintingTarget::Smooth(SmoothHinting {
            light: true,
            symmetric_rendering: true,
            preserve_linear_metrics: true,
        }),
        ..FontTweak::default()
    };

    for (name, bytes) in [
        (REGULAR, REGULAR_BYTES),
        (MEDIUM, MEDIUM_BYTES),
        (SEMI_BOLD, SEMI_BOLD_BYTES),
        (BOLD, BOLD_BYTES),
    ] {
        defs.font_data.insert(
            name.to_owned(),
            Arc::new(FontData::from_owned(bytes.to_vec()).tweak(crisp.clone())),
        );
    }

    for (name, bytes) in [
        (PHOSPHOR, PHOSPHOR_BYTES),
        (PHOSPHOR_BOLD, PHOSPHOR_BOLD_BYTES),
        (PHOSPHOR_FILL, PHOSPHOR_FILL_BYTES),
    ] {
        defs.font_data.insert(
            name.to_owned(),
            Arc::new(FontData::from_static(bytes)),
        );
    }

    // Proportional: Inter first, then Phosphor for icon glyphs, then egui defaults as glyph fallback.
    if let Some(proportional) = defs.families.get_mut(&FontFamily::Proportional) {
        let mut families = vec![REGULAR.to_owned(), PHOSPHOR.to_owned()];
        families.extend(
            proportional
                .iter()
                .filter(|f| *f != REGULAR && *f != PHOSPHOR)
                .cloned(),
        );
        *proportional = families;
    }

    // Heavier weights, addressable via FontFamily::Name.
    // Each includes Phosphor as glyph fallback.
    for (name, weight, icon_font) in [
        (MEDIUM, MEDIUM, PHOSPHOR),
        (SEMI_BOLD, SEMI_BOLD, PHOSPHOR_BOLD),
        (BOLD, BOLD, PHOSPHOR_BOLD),
    ] {
        defs.families.insert(
            FontFamily::Name(name.to_owned().into()),
            vec![weight.to_owned(), icon_font.to_owned(), REGULAR.to_owned()],
        );
    }

    // Dedicated icon font families
    defs.families.insert(
        FontFamily::Name(PHOSPHOR.to_owned().into()),
        vec![PHOSPHOR.to_owned(), REGULAR.to_owned()],
    );
    defs.families.insert(
        FontFamily::Name(PHOSPHOR_BOLD.to_owned().into()),
        vec![PHOSPHOR_BOLD.to_owned(), BOLD.to_owned()],
    );
    defs.families.insert(
        FontFamily::Name(PHOSPHOR_FILL.to_owned().into()),
        vec![PHOSPHOR_FILL.to_owned(), REGULAR.to_owned()],
    );

    ctx.set_fonts(defs);
}

/// The [`FontFamily`] for a given weight — convenient for `RichText::family`.
pub fn family(weight: &str) -> FontFamily {
    FontFamily::Name(weight.to_owned().into())
}
