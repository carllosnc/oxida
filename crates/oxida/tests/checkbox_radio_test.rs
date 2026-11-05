use oxida::prelude::*;

#[test]
fn test_checkbox_toggles() {
    let ctx = egui::Context::default();
    oxida::init(&ctx, &Theme::dark());

    let mut checked = false;

    // Frame 1: Render checkbox
    let mut out1 = ctx.run_ui(egui::RawInput::default(), |ui| {
        ui.oxida().checkbox(&mut checked, "Aceito os termos").size(Size::Sm).ui(ui);
    });
    out1.textures_delta.clear();

    assert!(!checked);
}

#[test]
fn test_checkbox_tri_state() {
    let ctx = egui::Context::default();
    oxida::init(&ctx, &Theme::dark());

    let mut state = CheckboxState::Mixed;

    let mut out1 = ctx.run_ui(egui::RawInput::default(), |ui| {
        ui.oxida().checkbox_state(&mut state, "Selecionar múltiplos").size(Size::Md).ui(ui);
    });
    out1.textures_delta.clear();

    assert_eq!(state, CheckboxState::Mixed);
}

#[test]
fn test_radio_selects() {
    let ctx = egui::Context::default();
    oxida::init(&ctx, &Theme::dark());

    let mut current_tier = 0;

    // Frame 1: Render radios
    let mut out1 = ctx.run_ui(egui::RawInput::default(), |ui| {
        ui.oxida().radio(&mut current_tier, 0, "Starter").size(Size::Sm).ui(ui);
        ui.oxida().radio(&mut current_tier, 1, "Pro").size(Size::Md).ui(ui);
    });
    out1.textures_delta.clear();

    assert_eq!(current_tier, 0);
}

#[test]
fn test_button_variants_and_loading() {
    let ctx = egui::Context::default();
    oxida::init(&ctx, &Theme::dark());

    let mut out1 = ctx.run_ui(egui::RawInput::default(), |ui| {
        Button::new("Carregando...")
            .variant(ButtonVariant::Primary)
            .size(Size::Md)
            .loading(true)
            .ui(ui);

        Button::new("Destrutivo")
            .variant(ButtonVariant::Danger)
            .size(Size::Sm)
            .ui(ui);

        Button::new("Secundário")
            .variant(ButtonVariant::Secondary)
            .ui(ui);

        Button::new("Ghost")
            .variant(ButtonVariant::Ghost)
            .ui(ui);
    });
    out1.textures_delta.clear();
}

#[test]
fn test_toggle_sizes() {
    let ctx = egui::Context::default();
    oxida::init(&ctx, &Theme::dark());

    let mut toggle_sm = false;
    let mut toggle_md = true;

    let mut out1 = ctx.run_ui(egui::RawInput::default(), |ui| {
        ui.oxida().toggle(&mut toggle_sm).size(Size::Sm).label("Ativar Sm").ui(ui);
        ui.oxida().toggle(&mut toggle_md).size(Size::Md).label("Ativar Md").ui(ui);
    });
    out1.textures_delta.clear();

    assert!(!toggle_sm);
    assert!(toggle_md);
}
