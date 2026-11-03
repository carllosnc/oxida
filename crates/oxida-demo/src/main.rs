//! Oxida Gallery — a live viewer for every component in the library.
//!
//! Run with: `cargo run -p oxida-demo`

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui::{self, Widget};
use oxida::prelude::*;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([880.0, 720.0])
            .with_min_inner_size([560.0, 420.0])
            .with_title("Oxida Gallery"),
        ..Default::default()
    };

    eframe::run_native(
        "oxida-demo",
        options,
        Box::new(|cc| {
            // OXIDA_THEME=light|dark picks the starting theme (default: dark).
            let dark = std::env::var("OXIDA_THEME").map_or(true, |v| v != "light");
            let theme = if dark { Theme::dark() } else { Theme::light() };
            oxida::init(&cc.egui_ctx, &theme);
            Ok(Box::new(App {
                dark,
                theme,
                ..Default::default()
            }))
        }),
    )
}

struct App {
    dark: bool,
    theme: Theme,
    frames: u32,

    // Widget state shown in the gallery.
    name: String,
    email: String,
    password: String,
    search_query: String,
    palette_open: bool,
    framework: String,
    role: String,
    last_action: String,
    selected_tab: usize,
    progress_val: f32,
    accept_terms: bool,
    marketing_emails: bool,
    disabled_check: bool,
    plan_tier: usize,
    volume: f32,
    speed: f32,
    notifications: bool,
    newsletter: bool,
    agree: bool,
}

impl Default for App {
    fn default() -> Self {
        Self {
            dark: true,
            theme: Theme::dark(),
            frames: 0,
            name: String::new(),
            email: String::new(),
            password: String::new(),
            search_query: String::new(),
            palette_open: std::env::var("OXIDA_PALETTE_OPEN").is_ok(),
            framework: "next".to_string(),
            role: "admin".to_string(),
            last_action: "Nenhuma ação recente".to_string(),
            selected_tab: 0,
            progress_val: 0.65,
            accept_terms: true,
            marketing_emails: false,
            disabled_check: true,
            plan_tier: 1,
            volume: 80.0,
            speed: 1.2,
            notifications: true,
            newsletter: false,
            agree: true,
        }
    }
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        if std::env::var("OXIDA_OPEN_SELECT").is_ok() {
            egui::containers::Popup::open_id(
                ui.ctx(),
                egui::Id::unique("framework_select").with("popup"),
            );
        }
        if std::env::var("OXIDA_OPEN_DROPDOWN").is_ok() {
            egui::containers::Popup::open_id(
                ui.ctx(),
                egui::Id::unique("menu_acoes").with("dropdown_popup"),
            );
        }
        self.header(ui);
        self.gallery(ui);
        self.command_palette(ui.ctx());

        // Auto-screenshot helper for visual verification:
        // OXIDA_SCREENSHOT=path.png cargo run -p oxida-demo
        if let Ok(path) = std::env::var("OXIDA_SCREENSHOT") {
            self.frames += 1;
            // egui is reactive (repaints only on input): keep frames coming
            // until the screenshot has been requested.
            if self.frames < 15 {
                ui.ctx().request_repaint();
            }
            if self.frames == 15 {
                ui.ctx().request_screenshot(move |image| {
                    let [w, h] = image.size;
                    let mut buf = Vec::with_capacity(w * h * 4);
                    for px in &image.pixels {
                        buf.extend_from_slice(&px.to_array());
                    }
                    match image::RgbaImage::from_raw(w as u32, h as u32, buf)
                        .expect("raw buffer size")
                        .save(&path)
                    {
                        Ok(()) => std::process::exit(0),
                        Err(e) => {
                            eprintln!("screenshot save failed: {e}");
                            std::process::exit(1);
                        }
                    }
                });
            }
        }
    }
}

impl App {
    /// Floating Command Palette handler (`Ctrl+K` / `⌘K`).
    fn command_palette(&mut self, ctx: &egui::Context) {
        if let Some(cmd) = CommandPalette::new(&mut self.palette_open)
            .group("Ações Rápidas")
            .item("novo_projeto", "Criar Novo Projeto", Some(icons::PLUS), Some("Ctrl+N"))
            .item("salvar", "Salvar Modificações", Some(icons::FLOPPY_DISK), Some("Ctrl+S"))
            .group("Navegação")
            .item("configuracoes", "Abrir Painel de Configurações", Some(icons::GEAR_SIX), Some("Ctrl+,"))
            .item("docs", "Guia & Documentação", Some(icons::BOOK_OPEN), Some("F1"))
            .group("Aparência")
            .item("alternar_tema", "Alternar Tema Claro / Escuro", Some(icons::MOON), Some("Ctrl+T"))
            .show(ctx)
        {
            if cmd == "alternar_tema" {
                self.dark = !self.dark;
                self.theme = if self.dark { Theme::dark() } else { Theme::light() };
                oxida::init(ctx, &self.theme);
                self.last_action = "Alternou o tema via Command Palette".to_string();
            } else {
                self.last_action = format!("Comando executado: '{cmd}'");
            }
        }
    }

    /// Fixed top bar: library name on the left, appearance switch on the right.
    fn header(&mut self, ui: &mut egui::Ui) {
        let theme = self.theme.clone();
        let p = &theme.palette;
        let t = &theme.typography;

        // `Panel::top` paints its own hairline separator (theme border color)
        // along the inner edge, dividing the header from the content.
        egui::Panel::top("header")
            .resizable(false)
            .frame(
                egui::Frame::new()
                    .fill(p.window_bg)
                    .inner_margin(egui::Margin::symmetric(36, 16)),
            )
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 10.0;
                    ui.label(
                        egui::RichText::new("Oxida")
                            .size(t.title)
                            .family(egui::FontFamily::Name(oxida::fonts::BOLD.into()))
                            .color(p.text_primary),
                    );
                    ui.label(
                        egui::RichText::new("galeria de componentes")
                            .size(t.sm)
                            .color(p.text_secondary),
                    );

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.oxida().toggle(&mut self.dark).ui(ui).changed() {
                            self.theme = if self.dark { Theme::dark() } else { Theme::light() };
                            oxida::init(ui.ctx(), &self.theme);
                        }
                        ui.add_space(8.0);
                        ui.label(
                            egui::RichText::new(format!(
                                "{} Tema {}",
                                if self.dark { icons::MOON } else { icons::SUN },
                                self.theme.name
                            ))
                            .size(t.sm)
                            .color(p.text_secondary),
                        );
                    });
                });
            });
    }

    fn gallery(&mut self, ui: &mut egui::Ui) {
        let theme = self.theme.clone();

        egui::CentralPanel::default()
            .frame(
                egui::Frame::central_panel(ui.style())
                    .inner_margin(egui::Margin {
                        left: 36,
                        right: 4,
                        top: 24,
                        bottom: 24,
                    }),
            )
            .show(ui, |ui| {
                let mut scroll = egui::ScrollArea::vertical().auto_shrink([false, false]);
                if let Some(y) = std::env::var("OXIDA_SCROLL_Y").ok().and_then(|v| v.parse::<f32>().ok()) {
                    scroll = scroll.vertical_scroll_offset(y);
                }
                scroll.show(ui, |ui| {
                        let content_w = 480.0_f32.min(ui.available_width() - 36.0);

                        // 1. Buttons -------------------------------------------------
                        section_header(ui, "Botões (Buttons)", &theme);

                        subhead(ui, "VARIANTES", &theme);
                        ui.horizontal(|ui| {
                            ui.spacing_mut().item_spacing.x = 10.0;
                            ui.oxida().button("Primário");
                            ui.oxida().secondary_button("Secundário");
                            ui.oxida().ghost_button("Ghost");
                            ui.oxida().danger_button("Excluir");
                        });

                        ui.add_space(14.0);
                        subhead(ui, "COM ÍCONES", &theme);
                        ui.horizontal(|ui| {
                            ui.spacing_mut().item_spacing.x = 10.0;
                            if Button::primary("Novo Projeto").icon(icons::PLUS).ui(ui).clicked() {
                                self.last_action = "Clicou em 'Novo Projeto'".into();
                            }
                            if Button::secondary("Salvar").icon(icons::FLOPPY_DISK).ui(ui).clicked() {
                                self.last_action = "Clicou em 'Salvar'".into();
                            }
                            if Button::ghost("Avançar").icon_after(icons::ARROW_RIGHT).ui(ui).clicked() {
                                self.last_action = "Clicou em 'Avançar'".into();
                            }
                            if Button::danger("Excluir").icon(icons::TRASH).ui(ui).clicked() {
                                self.last_action = "Clicou em 'Excluir'".into();
                            }
                        });

                        ui.add_space(14.0);
                        subhead(ui, "ICON BUTTONS (QUADRADOS)", &theme);
                        ui.horizontal(|ui| {
                            ui.spacing_mut().item_spacing.x = 8.0;
                            if IconButton::primary(icons::PLUS).tooltip("Adicionar item").ui(ui).clicked() {
                                self.last_action = "Clicou no IconButton Plus".into();
                            }
                            if IconButton::secondary(icons::MAGNIFYING_GLASS).tooltip("Pesquisar").ui(ui).clicked() {
                                self.last_action = "Clicou no IconButton Pesquisar".into();
                            }
                            if IconButton::ghost(icons::GEAR_SIX).tooltip("Configurações").ui(ui).clicked() {
                                self.last_action = "Clicou no IconButton Configurações".into();
                            }
                            if IconButton::ghost(icons::BELL).tooltip("Notificações").ui(ui).clicked() {
                                self.last_action = "Clicou no IconButton Notificações".into();
                            }
                            if IconButton::ghost(icons::SHARE_NETWORK).tooltip("Compartilhar").ui(ui).clicked() {
                                self.last_action = "Clicou no IconButton Compartilhar".into();
                            }
                            if IconButton::danger(icons::TRASH).tooltip("Excluir").ui(ui).clicked() {
                                self.last_action = "Clicou no IconButton Excluir".into();
                            }
                            IconButton::ghost(icons::LOCK_KEY).enabled(false).tooltip("Bloqueado").ui(ui);
                        });

                        ui.add_space(8.0);
                        ui.horizontal(|ui| {
                            ui.spacing_mut().item_spacing.x = 8.0;
                            IconButton::secondary(icons::HEART).size(Size::Sm).tooltip("Small (28px)").ui(ui);
                            IconButton::secondary(icons::HEART).size(Size::Md).tooltip("Medium (34px)").ui(ui);
                            IconButton::secondary(icons::HEART).size(Size::Lg).tooltip("Large (40px)").ui(ui);
                        });

                        ui.add_space(14.0);
                        subhead(ui, "TAMANHOS E ESTADOS", &theme);
                        ui.horizontal(|ui| {
                            ui.spacing_mut().item_spacing.x = 10.0;
                            Button::new("Pequeno").size(Size::Sm).ui(ui);
                            Button::new("Médio").size(Size::Md).ui(ui);
                            Button::new("Grande").size(Size::Lg).ui(ui);
                            Button::new("Desabilitado").enabled(false).ui(ui);
                        });

                        ui.add_space(14.0);
                        subhead(ui, "LARGURA TOTAL", &theme);
                        ui.allocate_ui_with_layout(
                            egui::vec2(content_w, theme.metrics.control_height_md),
                            egui::Layout::left_to_right(egui::Align::Center),
                            |ui| {
                                Button::new("Ação em largura total").fill_width(true).ui(ui);
                            },
                        );

                        // Divider ----------------------------------------------------
                        Separator::horizontal().spacing(36.0).ui(ui);

                        // 2. Badges & Tags -------------------------------------------
                        section_header(ui, "Emblemas & Tags (Badges)", &theme);

                        subhead(ui, "VARIANTES", &theme);
                        ui.horizontal(|ui| {
                            ui.spacing_mut().item_spacing.x = 8.0;
                            Badge::default_badge("Padrão").ui(ui);
                            Badge::secondary("Secundário").ui(ui);
                            Badge::outline("Contorno").ui(ui);
                            Badge::success("Sucesso").ui(ui);
                            Badge::warning("Aviso").ui(ui);
                            Badge::destructive("Destrutivo").ui(ui);
                        });

                        ui.add_space(14.0);
                        subhead(ui, "COM PONTO DE STATUS E ÍCONE", &theme);
                        ui.horizontal(|ui| {
                            ui.spacing_mut().item_spacing.x = 8.0;
                            Badge::secondary("Online").dot(theme.palette.success).ui(ui);
                            Badge::secondary("Ocupado").dot(theme.palette.danger).ui(ui);
                            Badge::secondary("Pendente").dot(theme.palette.warning).ui(ui);
                            Badge::outline("Verificado").icon(icons::CHECK).ui(ui);
                            Badge::default_badge("Favorito").icon(icons::STAR).ui(ui);
                        });

                        // Divider ----------------------------------------------------
                        Separator::horizontal().spacing(36.0).ui(ui);

                        // 3. Tabs (Segmented Control) --------------------------------
                        section_header(ui, "Abas (Segmented Control / Tabs)", &theme);

                        subhead(ui, "NAVEGAÇÃO EM SEGMENTOS", &theme);
                        Tabs::new(&mut self.selected_tab, ["Visão Geral", "Métricas", "Notificações", "Avançado"]).ui(ui);

                        ui.add_space(8.0);
                        ui.label(
                            egui::RichText::new(format!(
                                "Aba ativa: {}",
                                match self.selected_tab {
                                    0 => "Visão Geral",
                                    1 => "Métricas",
                                    2 => "Notificações",
                                    _ => "Avançado",
                                }
                            ))
                            .size(theme.typography.sm)
                            .color(theme.palette.text_secondary),
                        );

                        // Divider ----------------------------------------------------
                        Separator::horizontal().spacing(36.0).ui(ui);

                        // 4. Cards ---------------------------------------------------
                        section_header(ui, "Cards de Conteúdo", &theme);

                        ui.allocate_ui_with_layout(
                            egui::vec2(content_w, 0.0),
                            egui::Layout::top_down(egui::Align::Min),
                            |ui| {
                                Card::new().show(ui, |ui| {
                                    Card::header(
                                        ui,
                                        "Plano Enterprise",
                                        "Acesso a computação dedicada, segurança avançada e SLAs personalizados.",
                                    );
                                    ui.horizontal(|ui| {
                                        Badge::success("Ativo").dot(theme.palette.success).ui(ui);
                                        Badge::outline("Renovação anual").ui(ui);
                                    });
                                    ui.add_space(12.0);
                                    ui.horizontal(|ui| {
                                        if Button::primary("Gerenciar Plano").size(Size::Sm).ui(ui).clicked() {
                                            self.last_action = "Clicou em Gerenciar Plano no Card".into();
                                        }
                                        if Button::ghost("Ver Faturas").size(Size::Sm).ui(ui).clicked() {
                                            self.last_action = "Clicou em Ver Faturas no Card".into();
                                        }
                                    });
                                });
                            },
                        );

                        // Divider ----------------------------------------------------
                        Separator::horizontal().spacing(36.0).ui(ui);

                        // 5. Alertas (Alerts & Callouts) -----------------------------
                        section_header(ui, "Alertas & Avisos (Alerts)", &theme);

                        ui.allocate_ui_with_layout(
                            egui::vec2(content_w, 0.0),
                            egui::Layout::top_down(egui::Align::Min),
                            |ui| {
                                Alert::info("Atualização disponível")
                                    .description("A versão 2.0 do Oxida já está pronta com novos componentes.")
                                    .ui(ui);

                                ui.add_space(10.0);
                                Alert::success("Configurações sincronizadas")
                                    .description("Todos os seus dispositivos estão atualizados com o perfil mais recente.")
                                    .ui(ui);

                                ui.add_space(10.0);
                                Alert::destructive("Falha na autenticação")
                                    .description("As credenciais inseridas expiraram. Por favor, faça login novamente.")
                                    .ui(ui);
                            },
                        );

                        // Divider ----------------------------------------------------
                        Separator::horizontal().spacing(36.0).ui(ui);

                        // 6. Progresso, Spinners & Avatares --------------------------
                        section_header(ui, "Progresso & Indicadores (Progress, Spinners & Avatars)", &theme);

                        subhead(ui, "BARRA DE PROGRESSO", &theme);
                        ui.allocate_ui_with_layout(
                            egui::vec2(content_w, 0.0),
                            egui::Layout::top_down(egui::Align::Min),
                            |ui| {
                                ProgressBar::new(self.progress_val).ui(ui);
                                ui.add_space(6.0);
                                ui.horizontal(|ui| {
                                    ui.label(
                                        egui::RichText::new(format!("{:.0}% concluído", self.progress_val * 100.0))
                                            .size(theme.typography.sm)
                                            .color(theme.palette.text_secondary),
                                    );
                                    if Button::ghost("Resetar").size(Size::Sm).ui(ui).clicked() {
                                        self.progress_val = 0.20;
                                    }
                                    if Button::ghost("+20%").size(Size::Sm).ui(ui).clicked() {
                                        self.progress_val = (self.progress_val + 0.20).min(1.0);
                                    }
                                });
                            },
                        );

                        ui.add_space(16.0);
                        subhead(ui, "SPINNERS CIRCULARES (CARREGAMENTO)", &theme);
                        ui.horizontal(|ui| {
                            ui.spacing_mut().item_spacing.x = 16.0;
                            Spinner::new().size(Size::Sm).ui(ui);
                            Spinner::new().size(Size::Md).ui(ui);
                            Spinner::new().size(Size::Lg).ui(ui);
                            ui.label(
                                egui::RichText::new("Processando dados em segundo plano...")
                                    .size(theme.typography.sm)
                                    .color(theme.palette.text_secondary),
                            );
                        });

                        ui.add_space(16.0);
                        subhead(ui, "AVATARES DE USUÁRIO", &theme);
                        ui.horizontal(|ui| {
                            ui.spacing_mut().item_spacing.x = 12.0;
                            Avatar::initials("Alex Martins").status(AvatarStatus::Online).size(Size::Sm).ui(ui);
                            Avatar::initials("Beatriz Costa").status(AvatarStatus::Online).size(Size::Md).ui(ui);
                            Avatar::initials("Carlos Drummond").status(AvatarStatus::Busy).size(Size::Lg).ui(ui);
                            Avatar::initials("Daniel Oliveira").status(AvatarStatus::Away).size(Size::Md).ui(ui);
                            Avatar::new("OX").status(AvatarStatus::Offline).size(Size::Md).ui(ui);
                        });

                        ui.add_space(16.0);
                        subhead(ui, "SEPARADOR COM RÓTULO", &theme);
                        ui.allocate_ui_with_layout(
                            egui::vec2(content_w, 0.0),
                            egui::Layout::top_down(egui::Align::Min),
                            |ui| {
                                Separator::with_label("OU CONTINUE COM").ui(ui);
                            },
                        );

                        // Divider ----------------------------------------------------
                        Separator::horizontal().spacing(36.0).ui(ui);

                        // 7. Produtividade & Navegação Desktop -----------------------
                        section_header(ui, "Produtividade & Navegação Desktop", &theme);

                        subhead(ui, "CAMPO DE BUSCA (SEARCH INPUT COM ÍCONES E CLEAR)", &theme);
                        ui.allocate_ui_with_layout(
                            egui::vec2(content_w, theme.metrics.control_height_md),
                            egui::Layout::top_down(egui::Align::Min),
                            |ui| {
                                let s_resp = ui.oxida()
                                    .search_input(&mut self.search_query)
                                    .placeholder("Pesquisar na biblioteca...")
                                    .shortcut("Ctrl+K")
                                    .ui(ui);
                                if s_resp.changed() {
                                    self.last_action = format!("Buscando por: '{}'", self.search_query);
                                }
                            },
                        );

                        ui.add_space(14.0);
                        subhead(ui, "TECLAS DE ATALHO VISUAIS (KBD)", &theme);
                        ui.horizontal(|ui| {
                            ui.spacing_mut().item_spacing.x = 8.0;
                            Kbd::new("Ctrl").ui(ui);
                            Kbd::shortcut("Ctrl", "K").ui(ui);
                            Kbd::shortcut("⌘", "P").ui(ui);
                            Kbd::keys(["Ctrl", "Shift", "F"]).ui(ui);
                            Kbd::new("Esc").ui(ui);
                            Kbd::new("Enter").ui(ui);
                        });

                        ui.add_space(14.0);
                        subhead(ui, "TRILHA DE NAVEGAÇÃO (BREADCRUMBS)", &theme);
                        if let Some(clicked) = Breadcrumbs::new()
                            .item_with_icon(icons::HOUSE, "Início")
                            .item("Oxida")
                            .item("Componentes")
                            .active("Navegação")
                            .show(ui)
                        {
                            self.last_action = format!("Clicou no segmento {} dos Breadcrumbs", clicked + 1);
                        }

                        ui.add_space(14.0);
                        subhead(ui, "COMMAND PALETTE (MODAL FLUTUANTE)", &theme);
                        ui.horizontal(|ui| {
                            ui.spacing_mut().item_spacing.x = 12.0;
                            if Button::primary("Abrir Command Palette")
                                .icon(icons::COMMAND)
                                .ui(ui)
                                .clicked()
                            {
                                self.palette_open = true;
                            }
                            ui.label(
                                egui::RichText::new("ou pressione")
                                    .size(theme.typography.sm)
                                    .color(theme.palette.text_secondary),
                            );
                            Kbd::shortcut("Ctrl", "K").ui(ui);
                        });

                        // Divider ----------------------------------------------------
                        Separator::horizontal().spacing(36.0).ui(ui);

                        // 8. Text Inputs ---------------------------------------------
                        section_header(ui, "Campos de texto", &theme);

                        field_label(ui, "Nome completo", &theme);
                        ui.add_sized(
                            [content_w, theme.metrics.control_height_md],
                            TextInput::new(&mut self.name).placeholder("Ex: Alex Martins"),
                        );

                        ui.add_space(12.0);
                        field_label(ui, "E-mail de contato", &theme);
                        ui.add_sized(
                            [content_w, theme.metrics.control_height_md],
                            TextInput::new(&mut self.email)
                                .placeholder("nome@empresa.com")
                                .id_salt(egui::Id::unique("email")),
                        );

                        ui.add_space(12.0);
                        field_label(ui, "Senha de acesso", &theme);
                        ui.add_sized(
                            [content_w, theme.metrics.control_height_md],
                            TextInput::new(&mut self.password)
                                .placeholder("Digite sua senha")
                                .password(true)
                                .id_salt(egui::Id::unique("password")),
                        );

                        // Divider ----------------------------------------------------
                        Separator::horizontal().spacing(36.0).ui(ui);

                        // 9. Selects & Dropdown Menus (shadcn/ui style) --------------
                        section_header(ui, "Seleção & Menus (Select & Dropdown)", &theme);

                        ui.horizontal(|ui| {
                            ui.spacing_mut().item_spacing.x = 24.0;

                            // Select 1: Framework
                            ui.vertical(|ui| {
                                field_label(ui, "Framework web", &theme);
                                ui.oxida()
                                    .select("framework_select", &mut self.framework)
                                    .placeholder("Selecione um framework")
                                    .width(220.0)
                                    .item("next".to_string(), "Next.js (App Router)")
                                    .item("svelte".to_string(), "SvelteKit")
                                    .item("astro".to_string(), "Astro")
                                    .item("nuxt".to_string(), "Nuxt 3")
                                    .item("remix".to_string(), "Remix")
                                    .ui(ui);
                            });

                            // Select 2: Role
                            ui.vertical(|ui| {
                                field_label(ui, "Nível de acesso", &theme);
                                ui.oxida()
                                    .select("role_select", &mut self.role)
                                    .placeholder("Permissão...")
                                    .width(180.0)
                                    .item("admin".to_string(), "Administrador")
                                    .item("editor".to_string(), "Editor de conteúdo")
                                    .item("viewer".to_string(), "Visualizador")
                                    .ui(ui);
                            });

                            // Dropdown Menu
                            ui.vertical(|ui| {
                                field_label(ui, "Ações rápidas", &theme);
                                let action = &mut self.last_action;
                                ui.oxida().dropdown_menu("menu_acoes", "Opções", |menu| {
                                    menu.header("GERENCIAMENTO");
                                    if menu.item("Configurações do projeto").clicked() {
                                        *action = "Abriu configurações".to_string();
                                    }
                                    if menu.item("Duplicar repositório").clicked() {
                                        *action = "Duplicou repositório".to_string();
                                    }
                                    menu.disabled_item("Sincronização em nuvem");
                                    menu.separator();
                                    if menu.danger_item("Excluir projeto").clicked() {
                                        *action = "Excluiu projeto!".to_string();
                                    }
                                });
                            });
                        });

                        ui.add_space(8.0);
                        ui.label(
                            egui::RichText::new(format!("Última ação: {}", self.last_action))
                                .size(theme.typography.sm)
                                .color(theme.palette.text_secondary),
                        );

                        // Divider ----------------------------------------------------
                        Separator::horizontal().spacing(36.0).ui(ui);

                        // 10. Seleção (Checkbox & Radio) -----------------------------
                        section_header(ui, "Seleção (Checkbox & Radio)", &theme);

                        ui.horizontal(|ui| {
                            ui.spacing_mut().item_spacing.x = 48.0;

                            // Coluna de Checkboxes
                            ui.vertical(|ui| {
                                subhead(ui, "CHECKBOXES", &theme);
                                ui.oxida()
                                    .checkbox(
                                        &mut self.accept_terms,
                                        "Aceito os termos de serviço e privacidade",
                                    )
                                    .ui(ui);
                                ui.add_space(8.0);
                                ui.oxida()
                                    .checkbox(
                                        &mut self.marketing_emails,
                                        "Receber novidades e alertas por e-mail",
                                    )
                                    .ui(ui);
                                ui.add_space(8.0);
                                Checkbox::new(&mut self.disabled_check)
                                    .label("Opção desabilitada")
                                    .enabled(false)
                                    .ui(ui);
                            });

                            // Coluna de Radio buttons
                            ui.vertical(|ui| {
                                subhead(ui, "PLANOS (RADIO)", &theme);
                                ui.oxida()
                                    .radio(&mut self.plan_tier, 0, "Plano Starter (Gratuito)")
                                    .ui(ui);
                                ui.add_space(8.0);
                                ui.oxida()
                                    .radio(&mut self.plan_tier, 1, "Plano Pro (R$ 49/mês)")
                                    .ui(ui);
                                ui.add_space(8.0);
                                Radio::new(&mut self.plan_tier, 2)
                                    .label("Plano Enterprise (Sob consulta)")
                                    .enabled(false)
                                    .ui(ui);
                            });
                        });

                        // Divider ----------------------------------------------------
                        Separator::horizontal().spacing(36.0).ui(ui);

                        // 11. Sliders ------------------------------------------------
                        section_header(ui, "Deslizadores (Sliders)", &theme);

                        ui.allocate_ui_with_layout(
                            egui::vec2(content_w, theme.metrics.control_height_md),
                            egui::Layout::top_down(egui::Align::Min),
                            |ui| {
                                ui.oxida()
                                    .slider(&mut self.volume, 0.0..=100.0)
                                    .label("Volume")
                                    .suffix("%")
                                    .ui(ui);

                                ui.add_space(8.0);
                                ui.oxida()
                                    .slider(&mut self.speed, 0.0..=2.0)
                                    .label("Velocidade")
                                    .step(0.1)
                                    .suffix("x")
                                    .ui(ui);
                            },
                        );

                        // Divider ----------------------------------------------------
                        Separator::horizontal().spacing(36.0).ui(ui);

                        // 12. Toggles ------------------------------------------------
                        section_header(ui, "Interruptores (Toggles)", &theme);

                        ui.oxida()
                            .toggle(&mut self.notifications)
                            .label("Notificações ativas")
                            .ui(ui);

                        ui.add_space(10.0);
                        ui.oxida()
                            .toggle(&mut self.newsletter)
                            .label("Boletim semanal")
                            .ui(ui);

                        // Divider ----------------------------------------------------
                        Separator::horizontal().spacing(36.0).ui(ui);

                        // 13. Integração com egui nativo ----------------------------
                        section_header(ui, "Integração nativa", &theme);

                        ui.horizontal(|ui| {
                            ui.spacing_mut().item_spacing.x = 20.0;
                            ui.checkbox(&mut self.agree, "Checkbox nativo do egui");
                            if ui.button("Botão nativo do egui").clicked() {
                                println!("native button clicked");
                            }
                        });

                        ui.add_space(36.0);
                    });
            });
    }
}



/// Cabeçalho padronizado para as seções da galeria (título limpo e destacado).
fn section_header(ui: &mut egui::Ui, title: &str, theme: &Theme) {
    let p = &theme.palette;
    let t = &theme.typography;
    ui.label(
        egui::RichText::new(title)
            .size(t.lg)
            .family(t.heading_family.clone())
            .color(p.text_primary),
    );
    ui.add_space(16.0);
}

/// Sub-rótulo de categorização em caixa alta sutil.
fn subhead(ui: &mut egui::Ui, text: &str, theme: &Theme) {
    ui.label(
        egui::RichText::new(text)
            .size(theme.typography.xs + 0.5)
            .family(egui::FontFamily::Name(oxida::fonts::SEMI_BOLD.into()))
            .color(theme.palette.text_secondary),
    );
    ui.add_space(8.0);
}

/// Rótulo para inputs de formulário.
fn field_label(ui: &mut egui::Ui, text: &str, theme: &Theme) {
    ui.label(
        egui::RichText::new(text)
            .size(theme.typography.md)
            .family(egui::FontFamily::Name(oxida::fonts::MEDIUM.into()))
            .color(theme.palette.text_primary),
    );
    ui.add_space(6.0);
}
