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

    // Frame 3: Next frame without events (popup should definitely be open and render content)
    let mut frame3_rendered = false;
    let mut out3 = ctx.run_ui(egui::RawInput::default(), |ui| {
        ui.oxida().dropdown_menu("test_menu", "Opções", |menu| {
            menu.item("Item 1");
            frame3_rendered = true;
        });
    });
    out3.textures_delta.clear();

    assert!(frame3_rendered, "Dropdown items should render when open!");
}

#[test]
fn test_menu_density_metrics() {
    let metrics = Metrics::default();
    assert_eq!(MenuDensity::Compact.row_height(&metrics), 30.0);
    assert_eq!(MenuDensity::Comfortable.row_height(&metrics), 38.0);
    assert_eq!(MenuDensity::Compact.padding_x(), 8.0);
    assert_eq!(MenuDensity::Comfortable.padding_x(), 10.0);
    assert_eq!(MenuDensity::Compact.icon_size(), 15.0);
    assert_eq!(MenuDensity::Comfortable.icon_size(), 17.0);
}

#[test]
fn test_menu_item_builder() {
    let item = MenuItem::new("Excluir")
        .icon(icons::TRASH)
        .shortcut("Del")
        .danger()
        .compact()
        .state(MenuItemState::Focus);

    assert_eq!(item.item_type, MenuItemType::Destructive);
    assert_eq!(item.density, Some(MenuDensity::Compact));
    assert_eq!(item.icon, Some(icons::TRASH));
    assert_eq!(item.state_override, Some(MenuItemState::Focus));
}

#[test]
fn test_menu_panel_static_rendering() {
    for dark in [true, false] {
        let ctx = egui::Context::default();
        let theme = if dark { Theme::dark() } else { Theme::light() };
        oxida::init(&ctx, &theme);

        let mut out = ctx.run_ui(egui::RawInput::default(), |ui| {
            // Dropdown - compacto
            MenuPanel::new(|menu| {
                menu.set_density(MenuDensity::Compact);
                menu.header("DOCUMENTO");
                menu.item_with("Novo documento", Some(icons::FILE_TEXT), Some("Ctrl+N"));
                menu.item_with("Abrir documento", Some(icons::FILE_TEXT), Some("Ctrl+O"));
                menu.item_with("Duplicar", Some(icons::FILE_TEXT), Some("Ctrl+D"));
                menu.separator();
                menu.submenu_item("Exportar", Some(icons::FILE_TEXT));
                menu.submenu_item("Mover para...", Some(icons::FILE_TEXT));
                menu.separator();
                menu.danger_item_with("Excluir", Some(icons::TRASH), Some("Del"));
            }).width(240.0).show(ui);

            // Contexto - confortável
            MenuPanel::new(|menu| {
                menu.set_density(MenuDensity::Comfortable);
                menu.header("AÇÕES DO DOCUMENTO");
                menu.item_with("Abrir documento", Some(icons::FILE_TEXT), Some("Ctrl+O"));
                menu.item_with("Renomear", Some(icons::PENCIL_SIMPLE), Some("F2"));
                menu.item_with("Duplicar", Some(icons::COPY_SIMPLE), Some("Ctrl+D"));
                menu.item_with("Copiar", Some(icons::COPY), Some("Ctrl+C"));
                menu.separator();
                menu.submenu_item("Mover para...", Some(icons::FILE_TEXT));
                menu.separator();
                menu.danger_item_with("Excluir", Some(icons::TRASH), Some("Del"));
            }).width(260.0).show(ui);
        });
        out.textures_delta.clear();
    }
}

#[test]
fn test_menu_20_variants_matrix() {
    let ctx = egui::Context::default();
    oxida::init(&ctx, &Theme::dark());

    let mut out = ctx.run_ui(egui::RawInput::default(), |ui| {
        MenuPanel::new(|menu| {
            let states = [
                MenuItemState::Default,
                MenuItemState::Hover,
                MenuItemState::Pressed,
                MenuItemState::Focus,
                MenuItemState::Disabled,
            ];

            // 1. Padrão 38 px
            for &state in &states {
                menu.add_item(
                    MenuItem::new("Ação de menu")
                        .icon(icons::FILE_TEXT)
                        .shortcut("Ctrl+K")
                        .comfortable()
                        .state(state),
                );
            }

            // 2. Padrão 30 px
            for &state in &states {
                menu.add_item(
                    MenuItem::new("Ação de menu")
                        .icon(icons::FILE_TEXT)
                        .shortcut("Ctrl+K")
                        .compact()
                        .state(state),
                );
            }

            // 3. Destrutivo 38 px
            for &state in &states {
                menu.add_item(
                    MenuItem::new("Ação de menu")
                        .icon(icons::FILE_TEXT)
                        .shortcut("Ctrl+K")
                        .danger()
                        .comfortable()
                        .state(state),
                );
            }

            // 4. Destrutivo 30 px
            for &state in &states {
                menu.add_item(
                    MenuItem::new("Ação de menu")
                        .icon(icons::FILE_TEXT)
                        .shortcut("Ctrl+K")
                        .danger()
                        .compact()
                        .state(state),
                );
            }
        }).show(ui);
    });
    out.textures_delta.clear();
}
