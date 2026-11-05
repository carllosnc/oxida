use oxida::prelude::*;

#[test]
fn test_block_slider_renders_without_panics() {
    let ctx = egui::Context::default();
    let theme = Theme::dark();
    oxida::init(&ctx, &theme);

    let mut val_90 = 90.0;
    let mut val_50 = 50.0;
    let mut val_custom = 3.5;

    let mut out = ctx.run_ui(egui::RawInput::default(), |ui| {
        // Percentage block slider (matches 90% in screenshot)
        BlockSlider::percentage(&mut val_90).ui(ui);

        // Percentage block slider (matches 50% in screenshot)
        BlockSlider::percentage(&mut val_50).ui(ui);

        // Custom range, step and prefix
        BlockSlider::new(&mut val_custom, 0.0..=10.0)
            .step(0.5)
            .prefix("$")
            .size(Size::Lg)
            .accent()
            .ui(ui);

        // Fluent OxidaUi builder
        let mut fluent_val = 42.0;
        ui.oxida().block_slider(&mut fluent_val, 0.0..=100.0).suffix("%").ui(ui);
    });
    out.textures_delta.clear();

    assert_eq!(val_90, 90.0);
    assert_eq!(val_50, 50.0);
}
