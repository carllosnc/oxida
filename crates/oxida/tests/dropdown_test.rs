use oxida::prelude::*;

#[test]
fn test_dropdown_opens_on_click() {
    let ctx = egui::Context::default();
    oxida::init(&ctx, &Theme::dark());

    // Frame 1: Render idle
    let mut out1 = ctx.run_ui(egui::RawInput::default(), |ui| {
        ui.oxida().dropdown_menu("test_menu", "Opções", |menu| {
            menu.item("Item 1");
        });
    });
    out1.textures_delta.clear();

    let popup_id = egui::Id::unique("test_menu").with("dropdown_popup");
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

    let mut item_rendered = false;
    let mut out2 = ctx.run_ui(input, |ui| {
        ui.oxida().dropdown_menu("test_menu", "Opções", |menu| {
            menu.item("Item 1");
            item_rendered = true;
        });
    });
    out2.textures_delta.clear();

    // Check if it opened or will be open
    println!("Item rendered during click: {}", item_rendered);
    println!("Is popup open after click: {}", egui::containers::Popup::is_id_open(&ctx, popup_id));

    // Frame 3: Next frame without events (popup should definitely be open and render content)
    let mut frame3_rendered = false;
    let mut out3 = ctx.run_ui(egui::RawInput::default(), |ui| {
        ui.oxida().dropdown_menu("test_menu", "Opções", |menu| {
            menu.item("Item 1");
            frame3_rendered = true;
        });
    });
    out3.textures_delta.clear();

    println!("Item rendered on frame 3: {}", frame3_rendered);
    assert!(frame3_rendered, "Dropdown items should render when open!");
}
