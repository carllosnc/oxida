use oxida::prelude::*;

#[test]
fn test_new_components_render_without_panics() {
    let ctx = egui::Context::default();
    let theme = Theme::dark();
    oxida::init(&ctx, &theme);

    let mut out = ctx.run_ui(egui::RawInput::default(), |ui| {
        // Badges
        Badge::new("Novo").ui(ui);
        Badge::success("Ativo").dot(egui::Color32::GREEN).ui(ui);
        Badge::destructive("Erro").ui(ui);
        Badge::warning("Pendente").ui(ui);
        Badge::outline("Tag").ui(ui);

        // Card
        Card::new().show(ui, |ui| {
            Card::header(ui, "Título do Card", "Descrição do card");
            ui.label("Conteúdo");
        });

        // Separators
        Separator::horizontal().ui(ui);
        Separator::with_label("OU").ui(ui);

        // Tabs
        let mut tab = 0;
        Tabs::new(&mut tab, ["Aba 1", "Aba 2", "Aba 3"]).ui(ui);

        // Alert
        Alert::info("Aviso informativo").description("Mais detalhes aqui").ui(ui);
        Alert::success("Salvo com sucesso").ui(ui);

        // Progress & Spinner
        ProgressBar::new(0.75).ui(ui);
        Spinner::new().ui(ui);

        // Avatar
        Avatar::initials("Alex Martins").status(AvatarStatus::Online).ui(ui);
    });
    out.textures_delta.clear();
}
