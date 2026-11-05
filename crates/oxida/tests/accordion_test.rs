use oxida::prelude::*;

#[test]
fn test_accordion_renders_without_panics() {
    let ctx = egui::Context::default();
    let theme = Theme::dark();
    oxida::init(&ctx, &theme);

    let mut is_controlled_open = true;
    let mut active_section: Option<usize> = Some(0);

    let mut out = ctx.run_ui(egui::RawInput::default(), |ui| {
        // 1. Uncontrolled accordion
        let res1 = Accordion::new("Geral")
            .subtitle("Configurações básicas do aplicativo")
            .icon(icons::GEAR)
            .badge("Novo")
            .show(ui, |ui| {
                ui.label("Conteúdo da seção Geral");
            });
        assert!(!res1.is_open);

        // 2. Controlled accordion
        let res2 = Accordion::card("Segurança & Chaves")
            .open(&mut is_controlled_open)
            .icon(icons::SHIELD_CHECK)
            .show(ui, |ui| {
                ui.label("Conteúdo de segurança");
            });
        assert!(res2.is_open);
        assert!(res2.inner.is_some());

        // 3. Ghost variant
        let _res3 = Accordion::ghost("Avançado")
            .size(Size::Sm)
            .show(ui, |ui| {
                ui.label("Configurações para desenvolvedores");
            });

        // 4. Fluent OxidaUi builder
        let _res4 = ui.oxida().accordion("Notificações").show(ui, |ui| {
            ui.label("Alertas por email e push");
        });

        // 5. Exclusive AccordionGroup
        let mut group = AccordionGroup::new(&mut active_section);
        let g0 = group.item(0, "Grupo Item 0", ui, |ui| {
            ui.label("Item 0 ativo");
        });
        assert!(g0.is_open);

        let g1 = group.item(1, "Grupo Item 1", ui, |ui| {
            ui.label("Item 1 fechado");
        });
        assert!(!g1.is_open);
    });
    out.textures_delta.clear();

    assert!(is_controlled_open);
    assert_eq!(active_section, Some(0));
}
