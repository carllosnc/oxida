//! Bundled [Inter](https://rsms.me/inter/) font and installation helper.
//!
//! Inter is licensed under the SIL Open Font License 1.1 — see
//! `assets/fonts/LICENSE-Inter-OFL.txt`.

use std::sync::Arc;

use egui::{FontData, FontDefinitions, FontFamily};

/// Family name of the Inter Regular weight (the default).
pub const REGULAR: &str = "Inter";
/// Family name of the Inter Medium weight.
pub const MEDIUM: &str = "Inter Medium";
/// Family name of the Inter SemiBold weight (used for headings).
pub const SEMI_BOLD: &str = "Inter SemiBold";
/// Family name of the Inter Bold weight.
pub const BOLD: &str = "Inter Bold";

const REGULAR_BYTES: &[u8] = include_bytes!("../assets/fonts/Inter-Regular.ttf");
const MEDIUM_BYTES: &[u8] = include_bytes!("../assets/fonts/Inter-Medium.ttf");
const SEMI_BOLD_BYTES: &[u8] = include_bytes!("../assets/fonts/Inter-SemiBold.ttf");
const BOLD_BYTES: &[u8] = include_bytes!("../assets/fonts/Inter-Bold.ttf");

/// Install Inter as the default proportional font of `ctx`.
///
/// * Proportional text (body, buttons, inputs) renders in Inter Regular.
/// * [`FontFamily::Name`] with [`MEDIUM`], [`SEMI_BOLD`] and [`BOLD`]
///   address the heavier weights.
/// * egui's built-in fonts are kept as glyph fallback (emoji, etc).
///
/// Called automatically by [`crate::init`]; call it yourself if you apply
/// themes manually with [`crate::Theme::apply`].
pub fn install(ctx: &egui::Context) {
    let mut defs = FontDefinitions::default();

    for (name, bytes) in [
        (REGULAR, REGULAR_BYTES),
        (MEDIUM, MEDIUM_BYTES),
        (SEMI_BOLD, SEMI_BOLD_BYTES),
        (BOLD, BOLD_BYTES),
    ] {
        defs.font_data.insert(
            name.to_owned(),
            Arc::new(FontData::from_owned(bytes.to_vec())),
        );
    }

    // Proportional: Inter first, egui defaults as glyph fallback.
    if let Some(proportional) = defs.families.get_mut(&FontFamily::Proportional) {
        let mut families = vec![REGULAR.to_owned()];
        families.extend(proportional.iter().filter(|f| *f != REGULAR).cloned());
        *proportional = families;
    }

    // Heavier weights, addressable via FontFamily::Name.
    for (name, weight) in [(MEDIUM, MEDIUM), (SEMI_BOLD, SEMI_BOLD), (BOLD, BOLD)] {
        defs.families.insert(
            FontFamily::Name(name.to_owned().into()),
            vec![weight.to_owned(), REGULAR.to_owned()],
        );
    }

    ctx.set_fonts(defs);
}

/// The [`FontFamily`] for a given weight — convenient for `RichText::family`.
pub fn family(weight: &str) -> FontFamily {
    FontFamily::Name(weight.to_owned().into())
}
