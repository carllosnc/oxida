# AGENTS.md — AI Agent Guidelines for Oxida

This document defines the project context, design guidelines, architectural rules, and engineering best practices for any AI agent or contributor working in the **Oxida** repository.

---

## 1. Project Overview

**Oxida** is a minimalist, neutral design system and component library for **[egui](https://github.com/emilk/egui)** built specifically for **modern desktop applications** in Rust.

### The Problem We Solve
Out of the box, `egui` defaults to a classic "developer / debug tool" appearance (harsh contrast, borders on almost every element, unrefined spacing, and monospace-leaning typography). The mission of **Oxida** is to bring the sophistication, ergonomics, and visual polish of contemporary design systems (such as **shadcn/ui**, **Radix UI**, **Linear**, and **macOS/Windows 11 HIG**) into Rust desktop applications:
- Clean, neutral, and borderless soft surfaces (*borderless soft surfaces*).
- Refined typography utilizing the bundled **Inter** font family across multiple weights (Regular, Medium, SemiBold, Bold).
- Complete modern vector icon suite (**Phosphor Icons** bundled out-of-the-box).
- Cohesive `Light` and `Dark` themes powered by semantic design tokens.
- Desktop ergonomics: calibrated density, keyboard navigation, shortcuts, and clear interactive feedback.

---

## 2. Workspace Structure

The repository is organized as a Cargo workspace:
```
oxida/
├── crates/
│   ├── oxida/            # Core library (tokens, widgets, font loader, icons)
│   └── oxida-demo/       # Interactive component gallery and verification app
├── assets/fonts/         # Bundled TTF fonts (Inter + Phosphor)
├── README.md             # Project overview, usage, and roadmap
└── AGENTS.md             # Contributor & AI Agent guidelines
```

---

## 3. Design Philosophy and Visual Guidelines

1. **Soft Surfaces Over Hard Borders:**
   - Avoid using borders (`Stroke`) as the default delimiter. Rely instead on surface contrast and elevation (`p.surface`, `p.subtle`, `p.field_bg`).
   - Components such as `Card`, `Badge`, `Alert`, and `Tabs` must be **borderless by default** (`Stroke::NONE`), using subtle background tints.
2. **Strict Use of Semantic Tokens:**
   - **Never** hardcode raw RGB color values inside widgets.
   - Always consume colors from the semantic palette: `theme.palette` (`text_primary`, `text_secondary`, `text_disabled`, `accent`, `danger`, `success`, `warning`, `info`, `surface`, `subtle`, `border`).
   - Always adhere to theme metrics: `theme.metrics` (`radius_sm`, `radius_md`, `radius_lg`, `border_width`).
   - For color mixing or tinted backgrounds, use helper functions from [`oxida::util`](crates/oxida/src/util.rs) (`dim`, `mix`, `lerp_color`).
3. **Typography Hierarchy:**
   - **Section Titles:** Prominent, bold, and free of redundant subtitle noise (`Typography::lg` / `title` with `FontFamily::Name(crate::fonts::SEMI_BOLD)`).
   - **Subheadings & Menu Group Labels:** Small, muted, and semi-bold (`11.5px`–`12px` SemiBold in `text_secondary`).
   - **Buttons & Interactive Actions:** `Medium` weight.
   - **Body Text & Form Inputs:** `Regular` weight.

---

## 4. `egui` Peculiarities and Known Pitfalls

### ⚠️ The `override_text_color` Trap in Text Galleys
In `egui`, converting text into a galley via `text.into_galley(ui, ...)` bakes `ui.style().visuals.override_text_color` directly into the vertex colors of the typography mesh.
- **Consequence:** Calling `painter.galley(pos, galley, custom_color)` **ignores** `custom_color` if the galley already has vertex colors baked by a parent container style (e.g. inside `Popup`s or Menus where text inherits `text_primary`).
- **Mandatory Solution:** Always create text galleys with custom colors (disabled items, secondary text, headers, alerts) inside a scoped UI block:
  ```rust
  let galley = ui.scope(|ui| {
      ui.style_mut().visuals.override_text_color = Some(desired_color);
      label.into_galley(ui, Some(egui::TextWrapMode::Truncate), max_width, font_id)
  }).inner;
  ui.painter().galley(pos, galley, desired_color);
  ```

### Interactive Widget `Response` Contract
- Every interactive component must allocate its space using `ui.allocate_exact_size` or `ui.allocate_response` and return an `egui::Response`.
- Always allow the caller to chain checks like `.clicked()`, `.hovered()`, or `.changed()`.

### Extension Trait Pattern (`OxidaUi`)
When implementing or adding a new widget:
1. Implement the widget in `crates/oxida/src/widgets/<name>.rs`.
2. Re-export the widget in `crates/oxida/src/widgets/mod.rs` and `crates/oxida/src/lib.rs`.
3. Add the corresponding builder method to `struct Oxida<'a>` in `crates/oxida/src/widgets/mod.rs`.
4. Support both fluent invocation (`ui.oxida().my_widget(...)`) and direct struct instantiation (`MyWidget::new(...)`).

---

## 5. Rust Best Practices

- **Clean, Idiomatic Code:** Modern Rust with zero warnings in `cargo clippy`.
- **Documentation:** All public items (`pub struct`, `pub fn`, `pub enum`) must have comprehensive doc comments in English (`///`) with testable doctests (`cargo test --doc`) wherever practical.
- **Prelude:** Keep common types and the extension trait re-exported in `oxida::prelude`.
- **Zero Unsafe:** No `unsafe` blocks unless strictly necessary for FFI with clear documentation and justification.
- **Minimal Dependencies:** Prioritize standard library and existing `egui` / `epaint` primitives before introducing external crates.

---

## 6. Windows Environment and Process Execution

- **Binary File Locking (OS Error 5):**
  - If the demo application (`oxida-demo.exe`) is running while running `cargo build` or `cargo test --workspace`, Windows will lock the executable file and fail with `Access is denied. (os error 5)`.
  - **Best Practice:** For rapid checks and testing, run:
    ```powershell
    cargo test -p oxida
    cargo check --workspace
    ```
  - Close or terminate any running `oxida-demo` instance before launching a new one via `cargo run -p oxida-demo`.

---

## 7. Checklist for New Components

Before submitting or finishing work on an Oxida component:
- [ ] Does the component strictly respect `Theme` tokens (colors, radii, spacing)?
- [ ] Has the component been verified in both **Dark Theme** and **Light Theme**?
- [ ] Are surfaces elegant and free of unnecessary default borders?
- [ ] Does text rendering use `ui.scope` to prevent the `override_text_color` vertex baking issue?
- [ ] Is the component showcased in the interactive gallery (`crates/oxida-demo/src/main.rs`)?
- [ ] Are there unit tests or doctests ensuring panic-free rendering and interaction?
- [ ] Is all code documentation in English, and was the roadmap in `README.md` updated if milestones progressed?
