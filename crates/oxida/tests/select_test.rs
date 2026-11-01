use oxida::prelude::*;

#[test]
fn test_select_opens_and_selects_on_click() {
    let ctx = egui::Context::default();
    oxida::init(&ctx, &Theme::dark());

    let mut selected = "next".to_string();

    // Frame 1: Idle
    let mut out1 = ctx.run_ui(egui::RawInput::default(), |ui| {
        ui.oxida()
            .select("test_sel", &mut selected)
            .item("next".to_string(), "Next.js")
            .item("svelte".to_string(), "SvelteKit")
            .ui(ui);
    });
    out1.textures_delta.clear();

    let popup_id = egui::Id::unique("test_sel").with("popup");
    assert!(!egui::containers::Popup::is_id_open(&ctx, popup_id));

    // Frame 2: Click on trigger
    let mut input = egui::RawInput::default();
    input.events.push(egui::Event::PointerButton {
        pos: egui::pos2(20.0, 20.0),
        button: egui::PointerButton::Primary,
        pressed: true,
        modifiers: egui::Modifiers::default(),
    });
    input.events.push(egui::Event::PointerButton {
        pos: egui::pos2(20.0, 20.0),
        button: egui::PointerButton::Primary,
        pressed: false,
        modifiers: egui::Modifiers::default(),
    });

    let mut out2 = ctx.run_ui(input, |ui| {
        ui.oxida()
            .select("test_sel", &mut selected)
            .item("next".to_string(), "Next.js")
            .item("svelte".to_string(), "SvelteKit")
            .ui(ui);
    });
    out2.textures_delta.clear();

    println!("Is select popup open: {}", egui::containers::Popup::is_id_open(&ctx, popup_id));
    assert!(egui::containers::Popup::is_id_open(&ctx, popup_id));
}
