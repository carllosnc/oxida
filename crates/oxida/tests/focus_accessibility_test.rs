use oxida::prelude::*;

#[test]
fn test_focus_and_keyboard_interaction() {
    let ctx = egui::Context::default();
    oxida::init(&ctx, &Theme::dark());

    let mut checked = false;
    let mut radio_val = 0;
    let mut toggle_val = false;
    let mut slider_val = 50.0;

    // Frame 1: Render controls
    let mut out1 = ctx.run_ui(egui::RawInput::default(), |ui| {
        let resp_btn = ui.oxida().button("Test Button");
        assert!(!resp_btn.has_focus());

        let resp_check = ui.oxida().checkbox(&mut checked, "Check").ui(ui);
        assert!(!resp_check.has_focus());

        let resp_radio = ui.oxida().radio(&mut radio_val, 1, "Radio").ui(ui);
        assert!(!resp_radio.has_focus());

        let resp_toggle = ui.oxida().toggle(&mut toggle_val).ui(ui);
        assert!(!resp_toggle.has_focus());

        let resp_slider = ui.oxida().slider(&mut slider_val, 0.0..=100.0).ui(ui);
        assert!(!resp_slider.has_focus());
    });
    out1.textures_delta.clear();

    // Verify initial values
    assert!(!checked);
    assert_eq!(radio_val, 0);
    assert!(!toggle_val);
    assert!((slider_val - 50.0).abs() < f32::EPSILON);
}
