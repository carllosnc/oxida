use oxida::prelude::*;

#[test]
fn test_productivity_widgets_render_without_panics() {
    let ctx = egui::Context::default();
    let theme = Theme::dark();
    oxida::init(&ctx, &theme);

    let mut search_query = "oxida".to_string();
    let mut palette_open = true;

    let mut out = ctx.run_ui(egui::RawInput::default(), |ui| {
        // 1. Kbd keycaps
        Kbd::new("Ctrl").ui(ui);
        Kbd::shortcut("Ctrl", "K").ui(ui);
        Kbd::keys(["Ctrl", "Shift", "P"]).separator(None::<String>).ui(ui);
        ui.oxida().kbd("Enter");

        // 2. SearchInput
        let resp = ui.oxida()
            .search_input(&mut search_query)
            .placeholder("Pesquisar...")
            .shortcut("Ctrl+K")
            .ui(ui);
        assert!(!resp.changed());

        // 3. Breadcrumbs
        let clicked = Breadcrumbs::new()
            .item_with_icon(icons::HOUSE, "Início")
            .item("Projetos")
            .active("Oxida")
            .show(ui);
        assert_eq!(clicked, None);

        // 4. CommandPalette
        let action = CommandPalette::new(&mut palette_open)
            .group("Geral")
            .item("settings", "Configurações", Some(icons::GEAR_SIX), Some("Ctrl+,"))
            .item("help", "Ajuda", Some(icons::QUESTION), Some("F1"))
            .show(ui.ctx());
        assert_eq!(action, None);
    });
    out.textures_delta.clear();
}
