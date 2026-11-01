# Oxida

A **minimal, neutral design system and component library** for [egui](https://github.com/emilk/egui) — theme tokens plus hand-crafted widgets for building clean desktop interfaces.

```
crates/
├── oxida          # the library (theme + widgets)
└── oxida-demo     # gallery app to view every component live
```

## Features

- **Design tokens** — [`Palette`](crates/oxida/src/theme/palette.rs) (semantic colors), [`Typography`](crates/oxida/src/theme/typography.rs) and [`Metrics`](crates/oxida/src/theme/metrics.rs), with built-in `Light` and `Dark` themes
- **Widgets** — `Button` (4 variants × 3 sizes), `TextInput`, `Slider`, `Toggle`
- **Extension trait API** — `ui.oxida().button("Save")`, following the modern egui ecosystem pattern (`egui_extras`, `egui_flex`, ...)
- **Themes style egui itself too** — panels, menus, `TextEdit`, checkboxes and other built-in widgets match the active theme

## Quick start

```toml
[dependencies]
oxida = { path = "../oxida" }  # or version from crates.io, once published
egui = "0.37"
eframe = "0.37"
```

```rust
use oxida::prelude::*;

// At startup (inside your eframe creation closure):
oxida::init(&cc.egui_ctx, &Theme::dark());

// Each frame:
ui.oxida().text_input(&mut self.name).placeholder("Nome").ui(ui);
ui.oxida().slider(&mut self.volume, 0.0..=100.0).label("Volume").suffix("%").ui(ui);

if ui.oxida().button("Salvar").clicked() {
    println!("{}", self.name);
}
```

## Custom themes

Every widget reads intent-based tokens, so a custom theme is just a modified token bundle:

```rust
use oxida::Theme;

let mut theme = Theme::light();
theme.palette.accent = egui::Color32::from_rgb(79, 70, 229); // indigo
theme.metrics.radius_md = 8.0;

oxida::init(ctx, &theme); // live — switch themes anytime
```

## Gallery

A live viewer for every component, with a dark/light switcher:

```
cargo run -p oxida-demo
```

## Roadmap

- [x] Milestone 1 — theme tokens + Button, TextInput, Slider, Toggle + gallery
- [ ] Select/Combo, Checkbox, Radio, Progress bar, Spinner, Tabs
- [ ] Keyboard navigation and focus rings for all custom widgets
- [ ] Font loading helper (e.g. bundling Inter)
- [ ] Theme serialization (`serde`) and live theme editor
- [ ] Screenshots in CI (`egui_kittest`)

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.
