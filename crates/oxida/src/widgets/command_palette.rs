//! Themed Command Palette (`Ctrl+K` / `⌘K`) modal widget.

use egui::{
    pos2, vec2, Align, Align2, Area, Color32, FontFamily, FontId, Id, Key, Order, Rect, ScrollArea,
    Sense, Stroke, StrokeKind, TextEdit, UiBuilder, Widget, WidgetText,
};

use super::Kbd;
use crate::icons;
use crate::theme;
use crate::util::{mix, radius};

/// A single actionable command item inside the [`CommandPalette`].
#[derive(Clone, Debug)]
pub struct CommandItem {
    /// Unique command identifier returned when executed.
    pub id: String,
    /// Human-readable title of the command.
    pub label: String,
    /// Optional short secondary description.
    pub description: Option<String>,
    /// Optional leading Phosphor icon.
    pub icon: Option<String>,
    /// Optional keyboard shortcut label (e.g. `"Ctrl+N"`).
    pub shortcut: Option<String>,
    /// Optional grouping category (e.g. `"Navegação"` or `"Ações"`).
    pub group: Option<String>,
}

impl CommandItem {
    /// Create a new command item with an ID and label.
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            description: None,
            icon: None,
            shortcut: None,
            group: None,
        }
    }

    /// Add a secondary description.
    pub fn description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }

    /// Add a leading icon.
    pub fn icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// Add a trailing shortcut hint badge.
    pub fn shortcut(mut self, shortcut: impl Into<String>) -> Self {
        self.shortcut = Some(shortcut.into());
        self
    }

    /// Assign this command to a named group.
    pub fn group(mut self, group: impl Into<String>) -> Self {
        self.group = Some(group.into());
        self
    }
}

#[derive(Clone, Default)]
struct PaletteState {
    query: String,
    selected_idx: usize,
    was_open: bool,
    focus_needed: bool,
    keyboard_navigated: bool,
}

/// Floating command palette modal overlay (`Ctrl+K` / `⌘K`).
///
/// Features:
/// - Global keyboard toggle (`Ctrl+K` or `Cmd+K` when enabled).
/// - Dimmed background overlay that dismisses on click.
/// - Fuzzy/substring filtering on input.
/// - Full keyboard navigation (`↑`/`↓` arrows to navigate, `Enter` to run, `Escape` to close).
/// - Grouping support with clean section dividers.
/// - Integrated [`Kbd`] shortcut badges.
///
/// ```
/// # use oxida::prelude::*;
/// # fn demo(ctx: &egui::Context, mut open: bool) {
/// let action = CommandPalette::new(&mut open)
///     .group("Ações")
///     .item("novo", "Novo Projeto", Some(icons::PLUS), Some("Ctrl+N"))
///     .item("salvar", "Salvar", Some(icons::FLOPPY_DISK), Some("Ctrl+S"))
///     .show(ctx);
///
/// if let Some(id) = action {
///     println!("Executed command: {id}");
/// }
/// # }
/// ```
pub struct CommandPalette<'a> {
    open: &'a mut bool,
    id_salt: Id,
    items: Vec<CommandItem>,
    current_group: Option<String>,
    placeholder: String,
    auto_shortcut: bool,
}

impl<'a> CommandPalette<'a> {
    /// Create a new command palette bound to an `open` visibility state.
    pub fn new(open: &'a mut bool) -> Self {
        Self {
            open,
            id_salt: Id::unique("oxida_command_palette"),
            items: Vec::new(),
            current_group: None,
            placeholder: "Digite um comando ou pesquise...".to_string(),
            auto_shortcut: true,
        }
    }

    /// Set an explicit ID salt.
    pub fn id_salt(mut self, id: impl std::hash::Hash + std::fmt::Debug) -> Self {
        self.id_salt = Id::unique(id);
        self
    }

    /// Set the search input placeholder.
    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    /// Whether to automatically listen for `Ctrl+K` / `Cmd+K` to open the palette. Defaults to `true`.
    pub fn auto_shortcut(mut self, auto: bool) -> Self {
        self.auto_shortcut = auto;
        self
    }

    /// Set the current group for subsequent items.
    pub fn group(mut self, group: impl Into<String>) -> Self {
        self.current_group = Some(group.into());
        self
    }

    /// Append a command item with full options.
    pub fn item(
        mut self,
        id: impl Into<String>,
        label: impl Into<String>,
        icon: Option<impl Into<String>>,
        shortcut: Option<impl Into<String>>,
    ) -> Self {
        let mut item = CommandItem::new(id, label);
        item.group = self.current_group.clone();
        if let Some(ic) = icon {
            item.icon = Some(ic.into());
        }
        if let Some(sc) = shortcut {
            item.shortcut = Some(sc.into());
        }
        self.items.push(item);
        self
    }

    /// Append a [`CommandItem`] struct directly.
    pub fn add_item(mut self, mut item: CommandItem) -> Self {
        if item.group.is_none() {
            item.group = self.current_group.clone();
        }
        self.items.push(item);
        self
    }

    /// Render the command palette overlay if open, or check the shortcut.
    ///
    /// Returns `Some(item_id)` if an action was executed this frame.
    pub fn show(self, ctx: &egui::Context) -> Option<String> {
        let state_id = self.id_salt.with("state");
        let mut state = ctx.data_mut(|d| d.get_temp::<PaletteState>(state_id)).unwrap_or_default();

        // 1. Check global keyboard shortcut (Ctrl+K or Cmd+K)
        if self.auto_shortcut {
            let pressed_k = ctx.input(|i| {
                (i.modifiers.command || i.modifiers.ctrl) && i.key_pressed(Key::K)
            });
            if pressed_k {
                *self.open = !*self.open;
            }
        }

        // If newly opened, reset search query and flag focus request
        if *self.open && !state.was_open {
            state.query.clear();
            state.selected_idx = 0;
            state.focus_needed = true;
            state.keyboard_navigated = false;
        }
        state.was_open = *self.open;

        if !*self.open {
            ctx.data_mut(|d| d.insert_temp(state_id, state));
            return None;
        }

        let theme = theme::current(ctx);
        let p = &theme.palette;
        let m = &theme.metrics;
        let screen_rect = ctx.viewport_rect();

        let mut executed_id = None;

        // 2. Backdrop layer (dims screen and captures clicks to close)
        let backdrop_id = self.id_salt.with("backdrop");
        Area::new(backdrop_id)
            .order(Order::Foreground)
            .fixed_pos(screen_rect.min)
            .show(ctx, |ui| {
                let (_, resp) = ui.allocate_exact_size(screen_rect.size(), Sense::click());
                let backdrop_color = Color32::from_black_alpha(150);
                ui.painter().rect_filled(screen_rect, 0.0, backdrop_color);

                if resp.clicked() {
                    *self.open = false;
                }
            });

        // 3. Filter items matching query
        let query_lower = state.query.trim().to_lowercase();
        let filtered_items: Vec<(usize, &CommandItem)> = self
            .items
            .iter()
            .enumerate()
            .filter(|(_, it)| {
                if query_lower.is_empty() {
                    true
                } else {
                    it.label.to_lowercase().contains(&query_lower)
                        || it.description.as_ref().is_some_and(|d| d.to_lowercase().contains(&query_lower))
                        || it.group.as_ref().is_some_and(|g| g.to_lowercase().contains(&query_lower))
                }
            })
            .collect();

        // Handle keyboard navigation
        let num_filtered = filtered_items.len();
        if num_filtered > 0 {
            if state.selected_idx >= num_filtered {
                state.selected_idx = 0;
            }
            if ctx.input(|i| i.key_pressed(Key::ArrowDown)) {
                state.selected_idx = (state.selected_idx + 1) % num_filtered;
                state.keyboard_navigated = true;
            }
            if ctx.input(|i| i.key_pressed(Key::ArrowUp)) {
                state.selected_idx = if state.selected_idx == 0 {
                    num_filtered - 1
                } else {
                    state.selected_idx - 1
                };
                state.keyboard_navigated = true;
            }
            if ctx.input(|i| i.key_pressed(Key::Enter)) {
                executed_id = Some(filtered_items[state.selected_idx].1.id.clone());
                *self.open = false;
            }
        }

        if ctx.input(|i| i.key_pressed(Key::Escape)) {
            *self.open = false;
        }

        // 4. Modal Window Card
        let modal_width = 560.0_f32.min(screen_rect.width() - 32.0);
        let modal_id = self.id_salt.with("modal_window");

        Area::new(modal_id)
            .order(Order::Tooltip)
            .anchor(Align2::CENTER_CENTER, vec2(0.0, -60.0))
            .show(ctx, |ui| {
                let frame = egui::Frame {
                    fill: p.surface,
                    stroke: Stroke::new(m.border_width, p.border),
                    corner_radius: radius(m.radius_lg),
                    shadow: egui::epaint::Shadow {
                        offset: [0, 10],
                        blur: 32,
                        spread: 0,
                        color: Color32::from_black_alpha(130),
                    },
                    inner_margin: egui::Margin::same(0),
                    ..Default::default()
                };

                frame.show(ui, |ui| {
                    ui.set_width(modal_width);

                    // ==========================================
                    // --- 1. Search Input Bar (Pixel Perfect) ---
                    // ==========================================
                    let bar_h = 48.0_f32;
                    let (bar_rect, bar_resp) = ui.allocate_exact_size(vec2(modal_width, bar_h), Sense::click());

                    if ui.is_rect_visible(bar_rect) {
                        // Magnifying glass icon in Phosphor font
                        let icon_font = FontId::new(17.0, FontFamily::Name(crate::fonts::PHOSPHOR.into()));
                        let icon_galley = ui.scope(|ui| {
                            ui.style_mut().visuals.override_text_color = Some(p.text_secondary);
                            WidgetText::from(icons::MAGNIFYING_GLASS).into_galley(ui, Some(egui::TextWrapMode::Truncate), f32::INFINITY, icon_font)
                        }).inner;
                        let icon_pos = pos2(
                            bar_rect.min.x + 16.0,
                            bar_rect.center().y - icon_galley.size().y * 0.5,
                        );
                        ui.painter().galley(icon_pos, icon_galley, p.text_secondary);

                        // Esc key badge on trailing edge
                        let esc_kbd = Kbd::new("Esc").size(super::Size::Sm);
                        let esc_w = 32.0_f32;
                        let esc_pos = pos2(bar_rect.max.x - 16.0 - esc_w, bar_rect.center().y - 9.0);
                        let mut esc_ui = ui.new_child(UiBuilder::new().max_rect(Rect::from_min_size(esc_pos, vec2(esc_w, 18.0))));
                        esc_kbd.ui(&mut esc_ui);

                        // Central TextEdit (Vertically Centered)
                        let text_rect = Rect::from_min_max(
                            pos2(bar_rect.min.x + 44.0, bar_rect.min.y),
                            pos2(bar_rect.max.x - 60.0, bar_rect.max.y),
                        );
                        let mut text_ui = ui.new_child(UiBuilder::new().max_rect(text_rect));

                        let search_id = self.id_salt.with("search_input");
                        let edit = TextEdit::singleline(&mut state.query)
                            .id(search_id)
                            .frame(egui::Frame::NONE)
                            .min_size(text_rect.size())
                            .align(Align2::LEFT_CENTER)
                            .hint_text(WidgetText::from(&self.placeholder))
                            .font(FontId::new(theme.typography.md, FontFamily::Name(crate::fonts::REGULAR.into())));

                        let edit_resp = text_ui.add(edit);

                        if state.focus_needed {
                            edit_resp.request_focus();
                            state.focus_needed = false;
                        }

                        if bar_resp.clicked() && !edit_resp.has_focus() {
                            edit_resp.request_focus();
                        }
                    }

                    // Hairline separator below input
                    let (s_rect, _) = ui.allocate_exact_size(vec2(modal_width, 1.0), Sense::hover());
                    ui.painter().hline(s_rect.x_range(), s_rect.center().y, Stroke::new(1.0, p.border));

                    // ==========================================
                    // --- 2. Filtered Results List ---
                    // ==========================================
                    let max_list_height = 360.0_f32;

                    ui.scope(|ui| {
                        // Scrollbar sits right next to the modal border, with smooth non-bouncing kinematics
                        ui.spacing_mut().scroll.bar_outer_margin = 2.0;
                        ui.spacing_mut().scroll.bar_inner_margin = 0.0;
                        ui.spacing_mut().scroll.bar_width = 4.0;
                        ui.spacing_mut().scroll.dormant_handle_opacity = 0.25;
                        ui.spacing_mut().scroll.active_handle_opacity = 0.7;
                        ui.spacing_mut().scroll.kinetic.rubber_band = false;
                        ui.spacing_mut().item_spacing = vec2(0.0, 2.0);

                        ScrollArea::vertical()
                            .max_height(max_list_height)
                            .auto_shrink([false, true])
                            .show(ui, |ui| {
                                if filtered_items.is_empty() {
                                    ui.add_space(28.0);
                                    ui.vertical_centered(|ui| {
                                        ui.label(
                                            egui::RichText::new("Nenhum comando encontrado")
                                                .size(theme.typography.sm)
                                                .color(p.text_secondary)
                                                .family(FontFamily::Name(crate::fonts::MEDIUM.into())),
                                        );
                                    });
                                    ui.add_space(28.0);
                                } else {
                                    ui.add_space(4.0);
                                    let mut last_group: Option<String> = None;
                                    let available_w = ui.available_width();
                                    let item_margin_x = 6.0_f32;
                                    let item_w = (available_w - item_margin_x * 2.0).max(0.0);
                                    let item_h = 36.0_f32;

                                    for (filtered_idx, (_, item)) in filtered_items.iter().enumerate() {
                                        // Group Header
                                        if item.group != last_group {
                                            if let Some(ref grp) = item.group {
                                                if last_group.is_some() {
                                                    ui.add_space(8.0);
                                                } else {
                                                    ui.add_space(2.0);
                                                }
                                                ui.horizontal(|ui| {
                                                    ui.add_space(item_margin_x + 8.0);
                                                    ui.label(
                                                        egui::RichText::new(grp.to_uppercase())
                                                            .size(11.0)
                                                            .family(FontFamily::Name(crate::fonts::SEMI_BOLD.into()))
                                                            .color(p.text_secondary),
                                                    );
                                                });
                                                ui.add_space(2.0);
                                            }
                                            last_group = item.group.clone();
                                        }

                                        // Item Row
                                        let is_selected = filtered_idx == state.selected_idx;
                                        let (rect, resp) = ui.allocate_exact_size(vec2(available_w, item_h), Sense::click());
                                        let row_rect = Rect::from_center_size(rect.center(), vec2(item_w, item_h));

                                        // Only update selected item on hover if pointer actually moved
                                        if resp.hovered() && ui.input(|i| i.pointer.delta() != vec2(0.0, 0.0)) {
                                            state.selected_idx = filtered_idx;
                                            state.keyboard_navigated = false;
                                        }

                                        // Scroll to selected item only during keyboard navigation
                                        if is_selected && state.keyboard_navigated {
                                            resp.scroll_to_me(Some(Align::Center));
                                        }

                                        if resp.clicked() {
                                            executed_id = Some(item.id.clone());
                                            *self.open = false;
                                        }

                                        if ui.is_rect_visible(row_rect) {
                                            if is_selected {
                                                ui.painter().rect(
                                                    row_rect,
                                                    radius(m.radius_md),
                                                    p.subtle,
                                                    Stroke::NONE,
                                                    StrokeKind::Inside,
                                                );
                                            }

                                            let text_color = if is_selected {
                                                p.text_primary
                                            } else {
                                                mix(p.text_primary, p.text_secondary, 0.25)
                                            };

                                            let mut curr_x = row_rect.min.x + 10.0;

                                            // Phosphor Leading Icon
                                            if let Some(ref ic) = item.icon {
                                                let ic_font = FontId::new(16.0, FontFamily::Name(crate::fonts::PHOSPHOR.into()));
                                                let ic_galley = ui.scope(|ui| {
                                                    ui.style_mut().visuals.override_text_color = Some(text_color);
                                                    WidgetText::from(ic.as_str()).into_galley(ui, Some(egui::TextWrapMode::Truncate), f32::INFINITY, ic_font)
                                                }).inner;
                                                let ic_pos = pos2(curr_x, row_rect.center().y - ic_galley.size().y * 0.5);
                                                ui.painter().galley(ic_pos, ic_galley, text_color);
                                                curr_x += 24.0;
                                            }

                                            // Item Title
                                            let title_font = FontId::new(theme.typography.sm + 0.5, FontFamily::Name(crate::fonts::MEDIUM.into()));
                                            let title_galley = ui.scope(|ui| {
                                                ui.style_mut().visuals.override_text_color = Some(text_color);
                                                WidgetText::from(item.label.as_str()).into_galley(ui, Some(egui::TextWrapMode::Truncate), f32::INFINITY, title_font)
                                            }).inner;
                                            let title_pos = pos2(curr_x, row_rect.center().y - title_galley.size().y * 0.5);
                                            ui.painter().galley(title_pos, title_galley, text_color);

                                            // Item Shortcut on right edge
                                            if let Some(ref sc) = item.shortcut {
                                                let sc_font = FontId::new(theme.typography.xs, FontFamily::Name(crate::fonts::REGULAR.into()));
                                                let sc_galley = ui.scope(|ui| {
                                                    ui.style_mut().visuals.override_text_color = Some(p.text_secondary);
                                                    WidgetText::from(sc.as_str()).into_galley(ui, Some(egui::TextWrapMode::Truncate), f32::INFINITY, sc_font)
                                                }).inner;
                                                let sc_pos = pos2(
                                                    row_rect.max.x - sc_galley.size().x - 10.0,
                                                    row_rect.center().y - sc_galley.size().y * 0.5,
                                                );
                                                ui.painter().galley(sc_pos, sc_galley, p.text_secondary);
                                            }
                                        }
                                    }
                                    ui.add_space(4.0);
                                }
                            });
                    });

                    // Hairline separator above footer
                    let (f_sep, _) = ui.allocate_exact_size(vec2(modal_width, 1.0), Sense::hover());
                    ui.painter().hline(f_sep.x_range(), f_sep.center().y, Stroke::new(1.0, p.border));

                    // ==========================================
                    // --- 3. Footer Bar with Kbd Badges ---
                    // ==========================================
                    let footer_h = 32.0_f32;
                    let (hint_rect, _) = ui.allocate_exact_size(vec2(modal_width, footer_h), Sense::hover());

                    let hint_font = FontId::new(theme.typography.xs, FontFamily::Name(crate::fonts::REGULAR.into()));
                    let hint_color = p.text_secondary;
                    let kbd_color = mix(p.text_secondary, p.border, 0.4);

                    let mut curr_x = hint_rect.min.x + 14.0;
                    let center_y = hint_rect.center().y;

                    for (key, label) in [("↑↓", "Navegar"), ("↵", "Executar"), ("Esc", "Fechar")] {
                        // Keycap
                        let key_galley = ui.scope(|ui| {
                            ui.style_mut().visuals.override_text_color = Some(hint_color);
                            WidgetText::from(key).into_galley(ui, Some(egui::TextWrapMode::Truncate), f32::INFINITY, hint_font.clone())
                        }).inner;
                        let key_w = key_galley.size().x + 8.0;
                        let key_rect = Rect::from_center_size(pos2(curr_x + key_w * 0.5, center_y), vec2(key_w, 18.0));

                        ui.painter().rect(
                            key_rect,
                            radius(m.radius_sm),
                            p.subtle,
                            Stroke::new(m.border_width, kbd_color),
                            StrokeKind::Inside,
                        );
                        ui.painter().galley(
                            pos2(key_rect.center().x - key_galley.size().x * 0.5, center_y - key_galley.size().y * 0.5),
                            key_galley,
                            hint_color,
                        );
                        curr_x += key_w + 6.0;

                        // Label
                        let lbl_galley = ui.scope(|ui| {
                            ui.style_mut().visuals.override_text_color = Some(hint_color);
                            WidgetText::from(label).into_galley(ui, Some(egui::TextWrapMode::Truncate), f32::INFINITY, hint_font.clone())
                        }).inner;
                        ui.painter().galley(
                            pos2(curr_x, center_y - lbl_galley.size().y * 0.5),
                            lbl_galley.clone(),
                            hint_color,
                        );
                        curr_x += lbl_galley.size().x + 16.0;
                    }
                });
            });

        ctx.data_mut(|d| d.insert_temp(state_id, state));
        executed_id
    }
}
