//! Small painting/animation helpers shared by the widgets.

use egui::Color32;

/// Linearly interpolate between two colors in sRGB space.
///
/// Alpha-aware: used for fading disabled states as well.
pub(crate) fn mix(a: Color32, b: Color32, t: f32) -> Color32 {
    let t = t.clamp(0.0, 1.0);
    let (ar, ag, ab, aa) = a.to_tuple();
    let (br, bg, bb, ba) = b.to_tuple();
    let l = |x: u8, y: u8| (f32::from(x) + (f32::from(y) - f32::from(x)) * t).round() as u8;
    Color32::from_rgba_premultiplied(l(ar, br), l(ag, bg), l(ab, bb), l(aa, ba))
}

/// Convenience: `CornerRadius::same` with an `f32` token.
pub(crate) fn radius(r: f32) -> egui::CornerRadius {
    egui::CornerRadius::same(r.round() as u8)
}

/// Fade `color` towards `toward` by `amount` — used for disabled controls.
pub(crate) fn dim(color: Color32, toward: Color32, amount: f32) -> Color32 {
    mix(color, toward, amount)
}

/// Paints a rounded rectangle filled with a vertical gradient from top to bottom.
pub(crate) fn paint_vertical_gradient_rounded_rect(
    painter: &egui::Painter,
    rect: egui::Rect,
    corner_radius: f32,
    top_color: Color32,
    bottom_color: Color32,
) {
    if rect.width() <= 0.0 || rect.height() <= 0.0 {
        return;
    }

    if top_color == bottom_color {
        painter.rect_filled(rect, radius(corner_radius), top_color);
        return;
    }

    let r = corner_radius
        .min(rect.width() * 0.5)
        .min(rect.height() * 0.5)
        .max(0.0);

    if r < 0.5 {
        let mut mesh = egui::Mesh::default();
        mesh.reserve_triangles(2);
        mesh.reserve_vertices(4);
        let uv = egui::epaint::WHITE_UV;
        mesh.vertices.push(egui::epaint::Vertex {
            pos: egui::pos2(rect.min.x, rect.min.y),
            uv,
            color: top_color,
        });
        mesh.vertices.push(egui::epaint::Vertex {
            pos: egui::pos2(rect.max.x, rect.min.y),
            uv,
            color: top_color,
        });
        mesh.vertices.push(egui::epaint::Vertex {
            pos: egui::pos2(rect.max.x, rect.max.y),
            uv,
            color: bottom_color,
        });
        mesh.vertices.push(egui::epaint::Vertex {
            pos: egui::pos2(rect.min.x, rect.max.y),
            uv,
            color: bottom_color,
        });
        mesh.add_triangle(0, 1, 2);
        mesh.add_triangle(0, 2, 3);
        painter.add(egui::Shape::Mesh(mesh.into()));
        return;
    }

    let steps = 6;
    let mut levels = Vec::with_capacity(steps * 2 + 2);

    // 1. Top corner arcs (from y = min.y down to y = min.y + r)
    for k in 0..=steps {
        let frac = k as f32 / steps as f32;
        let theta = (std::f32::consts::FRAC_PI_2) * (1.0 - frac);
        let dy = r * (1.0 - theta.sin());
        let dx = r * (1.0 - theta.cos());
        let y = rect.min.y + dy;
        let x_left = rect.min.x + dx;
        let x_right = rect.max.x - dx;
        levels.push((x_left, x_right, y));
    }

    // 2. Middle straight section if any
    let mid_bottom_y = rect.max.y - r;
    if mid_bottom_y > levels.last().unwrap().2 + 0.1 {
        levels.push((rect.min.x, rect.max.x, mid_bottom_y));
    }

    // 3. Bottom corner arcs (from y = max.y - r + dy down to y = max.y)
    for k in 1..=steps {
        let frac = k as f32 / steps as f32;
        let theta = (std::f32::consts::FRAC_PI_2) * frac;
        let dy = r * theta.sin();
        let dx = r * (1.0 - theta.cos());
        let y = rect.max.y - r + dy;
        let x_left = rect.min.x + dx;
        let x_right = rect.max.x - dx;
        levels.push((x_left, x_right, y));
    }

    let h = rect.height().max(1.0);
    let uv = egui::epaint::WHITE_UV;
    let mut mesh = egui::Mesh::default();
    mesh.reserve_vertices(levels.len() * 2);
    mesh.reserve_triangles((levels.len() - 1) * 2);

    for &(x_l, x_r, y) in &levels {
        let t = ((y - rect.min.y) / h).clamp(0.0, 1.0);
        let c = mix(top_color, bottom_color, t);
        mesh.vertices.push(egui::epaint::Vertex {
            pos: egui::pos2(x_l, y),
            uv,
            color: c,
        });
        mesh.vertices.push(egui::epaint::Vertex {
            pos: egui::pos2(x_r, y),
            uv,
            color: c,
        });
    }

    for i in 0..(levels.len() - 1) as u32 {
        let v0 = i * 2;
        let v1 = i * 2 + 1;
        let v2 = (i + 1) * 2 + 1;
        let v3 = (i + 1) * 2;
        mesh.add_triangle(v0, v1, v2);
        mesh.add_triangle(v0, v2, v3);
    }

    painter.add(egui::Shape::Mesh(mesh.into()));
}

