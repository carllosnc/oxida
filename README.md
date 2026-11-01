# Oxida

A **minimal, neutral design system and component library** for [egui](https://github.com/emilk/egui) — theme tokens plus hand-crafted widgets for building clean desktop interfaces.

```
crates/
├── oxida          # the library (theme + widgets)
└── oxida-demo     # gallery app to view every component live
```

## Features

- **Design tokens** — [`Palette`](crates/oxida/src/theme/palette.rs) (semantic colors), [`Typography`](crates/oxida/src/theme/typography.rs) and [`Metrics`](crates/oxida/src/theme/metrics.rs), with built-in `Light` and `Dark` themes
- **Inter by default** — the [Inter](https://rsms.me/inter/) font (Regular/Medium/SemiBold/Bold, OFL-licensed) is bundled and installed by `oxida::init`; headings render in SemiBold
- **Widgets** — `Button` (4 variants × 3 sizes), `IconButton`, `TextInput`, `Slider`, `Toggle`, `Checkbox`, `Radio`, `Select`, `DropdownMenu`, `Badge`, `Card`, `Alert`, `Tabs`, `Separator`, `ProgressBar`, `Spinner`, `Avatar`
- **Phosphor Icons** — 3,000+ bundled Phosphor icons (`oxida::icons`) across Regular, Bold and Fill weights
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
ui.oxida().text_input(&mut self.name).placeholder("Name").ui(ui);
ui.oxida().slider(&mut self.volume, 0.0..=100.0).label("Volume").suffix("%").ui(ui);

if ui.oxida().button("Save").clicked() {
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

Useful environment variables:

| Variable | Effect |
| --- | --- |
| `OXIDA_THEME=light\|dark` | Starting theme (default: `dark`) |
| `OXIDA_SCREENSHOT=path.png` | Save a PNG of the gallery after a few frames and exit (used for visual testing) |

## Roadmap

### Completed
- [x] **Milestone 1** — Theme tokens (`Palette`, `Typography`, `Metrics`) + `Button`, `TextInput`, `Slider`, `Toggle` + interactive gallery
- [x] **Typography & Fonts** — Automated loading of the Inter font family (Regular, Medium, SemiBold, Bold)
- [x] **Icon Pack** — Built-in Phosphor Icons support (3,000+ icons across Regular, Bold and Fill) and `IconButton` widget
- [x] **Core Selection Widgets** — `Select` (shadcn style), `DropdownMenu`, `Checkbox`, `Radio`
- [x] **Display & Feedback Widgets** — `Badge`, `Card` (borderless), `Alert` (styled), `Tabs` (segmented pills), `Separator`, `ProgressBar`, `Spinner`, `Avatar`

### Desktop Shell & Layout
- [ ] **Custom Titlebar** — Frameless window header with minimize, maximize, close controls, drag area, and action slots
- [ ] **Resizable Splitter / Panels** — Resizable split layout (horizontal/vertical with persistence)
- [ ] **Status Bar** — Compact window footer for state, sync indicator, and shortcuts

### Desktop Productivity & Navigation
- [ ] **`CommandPalette` (`Ctrl + K` / `⌘K`)** — Floating modal with fuzzy search and quick actions
- [ ] **`Kbd`** — Visual keyboard shortcut badge for buttons, menus, and tooltips
- [ ] **`Sidebar` / `NavRail`** — Collapsible side navigation drawer with icons and labels
- [ ] **`TreeView`** — Hierarchical tree viewer for directories, layers, and settings nodes
- [ ] **`Breadcrumbs`** — Hierarchical breadcrumb navigation trail

### Overlays & Feedback
- [ ] **`Toast`** — Floating stacked notifications system (with auto-dismiss and action hooks)
- [ ] **`Dialog` / `Modal`** — Centered modal dialogs with backdrop for confirmations and action flows
- [ ] **`ContextMenu`** — First-class right-click contextual menu support for arbitrary widgets

### Desktop Forms & Data
- [ ] **`SearchInput`** — Specialized search field with Phosphor search icon, clear button, and shortcut badge
- [ ] **`NumberInput` / `DragValue`** — Stepper with mouse wheel scrubbing and horizontal drag support
- [ ] **`Table` / Data Grid** — Desktop table with column sorting and row selection
- [ ] **`EmptyState`** — Visual placeholder with icon, message, and primary call-to-action for empty lists/views

### Ergonomics & Infrastructure
- [ ] Full keyboard navigation and accessible focus rings (`focus_ring`) for all widgets
- [ ] Density modes (Compact vs Comfortable)
- [ ] Theme serialization (`serde`) and live theme editor
- [ ] CI visual regression testing (`egui_kittest`)

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.
