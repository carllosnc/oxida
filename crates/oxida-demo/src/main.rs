//! Oxida Gallery — a live viewer for every component in the library.
//!
//! Run with: `cargo run -p oxida-demo`

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui::{self, Widget};
use oxida::prelude::*;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([880.0, 620.0])
            .with_min_inner_size([560.0, 420.0])
            .with_title("Oxida Gallery"),
        ..Default::default()
    };

    eframe::run_native(
        "oxida-demo",
        options,
        Box::new(|cc| {
            let theme = Theme::dark();
            oxida::init(&cc.egui_ctx, &theme);
            Ok(Box::new(App {
                dark: true,
                theme,
                ..Default::default()
            }))
        }),
    )
}

#[derive(Default)]
struct App {
    dark: bool,
    theme: Theme,

    // Widget state shown in the gallery.
    name: String,
    email: String,
    password: String,
    volume: f32,
    speed: f32,
    notifications: bool,
    newsletter: bool,
    agree: bool,
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.sidebar(ui);
        self.gallery(ui);
    }
}

impl App {
    fn sidebar(&mut self, ui: &mut egui::Ui) {
        egui::Panel::left("sidebar")
            .resizable(false)
            .default_size(220.0)
            .show(ui, |ui| {
                let p = self.theme.palette.clone();

                ui.add_space(12.0);
                ui.label(egui::RichText::new("Oxida").text_style(egui::TextStyle::Heading));
                ui.label(
                    egui::RichText::new("component gallery")
                        .text_style(egui::TextStyle::Small)
                        .color(p.text_secondary),
                );

                ui.add_space(12.0);
                ui.separator();
                ui.add_space(8.0);

                ui.label(
                    egui::RichText::new("APARÊNCIA")
                        .text_style(egui::TextStyle::Small)
                        .color(p.text_secondary),
                );
                ui.add_space(4.0);
                if ui
                    .oxida()
                    .toggle(&mut self.dark)
                    .label("Tema escuro")
                    .ui(ui)
                    .changed()
                {
                    self.theme = if self.dark { Theme::dark() } else { Theme::light() };
                    oxida::init(ui.ctx(), &self.theme);
                }

                ui.add_space(8.0);
                ui.label(
                    egui::RichText::new(
                        "O tema estiliza também os widgets nativos do egui (painéis, menus, checkboxes...).",
                    )
                    .text_style(egui::TextStyle::Small)
                    .color(p.text_secondary),
                );
            });
    }

    fn gallery(&mut self, ui: &mut egui::Ui) {
        egui::CentralPanel::default().show(ui, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                let theme = self.theme.clone();

                ui.add_space(8.0);
                ui.label(egui::RichText::new("Gallery").text_style(egui::TextStyle::Heading));
                ui.label(
                    egui::RichText::new(format!("Tema ativo: {}", theme.name))
                        .text_style(egui::TextStyle::Small)
                        .color(theme.palette.text_secondary),
                );

                // Buttons -----------------------------------------------------
                section(ui, &theme, "Buttons", |ui| {
                    ui.horizontal_wrapped(|ui| {
                        ui.spacing_mut().item_spacing.x = 8.0;
                        ui.oxida().button("Primary");
                        ui.oxida().secondary_button("Secondary");
                        ui.oxida().ghost_button("Ghost");
                        ui.oxida().danger_button("Delete");
                    });
                    ui.add_space(8.0);
                    ui.horizontal_wrapped(|ui| {
                        ui.spacing_mut().item_spacing.x = 8.0;
                        Button::new("Small").size(Size::Sm).ui(ui);
                        Button::new("Medium").size(Size::Md).ui(ui);
                        Button::new("Large").size(Size::Lg).ui(ui);
                        Button::new("Disabled").enabled(false).ui(ui);
                    });
                    ui.add_space(8.0);
                    Button::new("Full width").fill_width(true).ui(ui);
                });

                // Text inputs -------------------------------------------------
                section(ui, &theme, "Text inputs", |ui| {
                    ui.add_sized(
                        [280.0, theme.metrics.control_height_md],
                        TextInput::new(&mut self.name).placeholder("Nome completo"),
                    );
                    ui.add_space(8.0);
                    ui.add_sized(
                        [280.0, theme.metrics.control_height_md],
                        TextInput::new(&mut self.email)
                            .placeholder("E-mail")
                            .id_salt(egui::Id::unique("email")),
                    );
                    ui.add_space(8.0);
                    ui.add_sized(
                        [280.0, theme.metrics.control_height_md],
                        TextInput::new(&mut self.password)
                            .placeholder("Senha")
                            .password(true)
                            .id_salt(egui::Id::unique("password")),
                    );
                });

                // Sliders -----------------------------------------------------
                section(ui, &theme, "Sliders", |ui| {
                    ui.oxida()
                        .slider(&mut self.volume, 0.0..=100.0)
                        .label("Volume")
                        .suffix("%")
                        .ui(ui);
                    ui.add_space(8.0);
                    ui.oxida()
                        .slider(&mut self.speed, 0.0..=1.0)
                        .label("Velocidade")
                        .step(0.05)
                        .ui(ui);
                });

                // Toggles -----------------------------------------------------
                section(ui, &theme, "Toggles", |ui| {
                    ui.oxida()
                        .toggle(&mut self.notifications)
                        .label("Notificações")
                        .ui(ui);
                    ui.add_space(8.0);
                    ui.oxida()
                        .toggle(&mut self.newsletter)
                        .label("Newsletter")
                        .ui(ui);
                });

                // Cards / integration -----------------------------------------
                section(ui, &theme, "Cards & integração", |ui| {
                    theme.card_frame().show(ui, |ui| {
                        ui.label(egui::RichText::new("Card de exemplo").strong());
                        ui.label(
                            egui::RichText::new(
                                "Cartões usam os tokens do tema (surface, border, radius).",
                            )
                            .color(theme.palette.text_secondary),
                        );
                        ui.add_space(8.0);
                        ui.checkbox(
                            &mut self.agree,
                            "Checkbox nativa do egui (estilizada pelo tema)",
                        );
                        ui.add_space(4.0);
                        if ui.button("Botão nativo do egui").clicked() {
                            println!("native button clicked");
                        }
                    });
                });

                ui.add_space(24.0);
            });
        });
    }
}

/// A titled section wrapped in a themed card.
fn section(ui: &mut egui::Ui, theme: &Theme, title: &str, add: impl FnOnce(&mut egui::Ui)) {
    ui.add_space(16.0);
    ui.label(
        egui::RichText::new(title)
            .size(theme.typography.lg)
            .strong(),
    );
    ui.add_space(6.0);
    theme.card_frame().show(ui, add);
}
