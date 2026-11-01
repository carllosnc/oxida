use oxida::prelude::*;

#[test]
fn test_checkbox_toggles() {
    let ctx = egui::Context::default();
    oxida::init(&ctx, &Theme::dark());

    let mut checked = false;

    // Frame 1: Render checkbox
    let mut out1 = ctx.run_ui(egui::RawInput::default(), |ui| {
        ui.oxida().checkbox(&mut checked, "Aceito os termos").ui(ui);
    });
    out1.textures_delta.clear();

    assert!(!checked);
}

#[test]
fn test_radio_selects() {
    let ctx = egui::Context::default();
    oxida::init(&ctx, &Theme::dark());

    let mut current_tier = 0;

    // Frame 1: Render radios
    let mut out1 = ctx.run_ui(egui::RawInput::default(), |ui| {
        ui.oxida().radio(&mut current_tier, 0, "Starter").ui(ui);
        ui.oxida().radio(&mut current_tier, 1, "Pro").ui(ui);
    });
    out1.textures_delta.clear();

    assert_eq!(current_tier, 0);
}
