use crate::core::object::PathPoint;

pub fn measure_text(content: &str, font_size: f32) -> (f32, f32) {
    use iced::advanced::text::Paragraph as _;

    let text = iced::advanced::Text {
        content,
        bounds: iced::Size::new(f32::INFINITY, f32::INFINITY),
        size: iced::Pixels(font_size),
        line_height: iced::advanced::text::LineHeight::default(),
        font: iced::Font::default(),
        align_x: iced::alignment::Horizontal::Left.into(),
        align_y: iced::alignment::Vertical::Top.into(),
        shaping: iced::advanced::text::Shaping::Basic,
        wrapping: iced::advanced::text::Wrapping::None,
    };

    let paragraph = <iced::Renderer as iced::advanced::text::Renderer>::Paragraph::with_text(text);
    let bounds = paragraph.min_bounds();
    (bounds.width.max(1.0), bounds.height.max(1.0))
}

pub fn is_valid_hex(s: &str) -> bool {
    let clean = s.trim_start_matches('#');
    clean.len() == 6 || clean.len() == 8
}

pub fn hex_to_rgb6(s: &str) -> String {
    let clean = s.trim_start_matches('#');
    if clean.len() >= 6 {
        format!("#{}", &clean[..6])
    } else {
        s.to_string()
    }
}

pub fn smoothing_to_tolerance(smoothing: f32) -> f32 {
    1.0 + smoothing * 10.0
}

pub fn smoothing_to_sample_distance(smoothing: f32) -> f32 {
    1.0 + smoothing * 3.0
}

pub fn simplify_path(points: &mut Vec<PathPoint>, tolerance: f32) {
    if points.len() <= 2 {
        return;
    }
    let tol_sq = tolerance * tolerance;
    let mut keep = vec![false; points.len()];
    keep[0] = true;
    keep[points.len() - 1] = true;

    dp_recursive(points, &mut keep, 0, points.len() - 1, tol_sq);

    let mut simplified = Vec::new();
    for (i, p) in points.iter().enumerate() {
        if keep[i] {
            simplified.push(p.clone());
        }
    }
    *points = simplified;
}

fn dp_recursive(points: &[PathPoint], keep: &mut [bool], start: usize, end: usize, tol_sq: f32) {
    if end - start <= 1 {
        return;
    }

    let p1 = &points[start];
    let p2 = &points[end];
    let dx = p2.x - p1.x;
    let dy = p2.y - p1.y;
    let len_sq = dx * dx + dy * dy;

    let mut max_dist_sq = 0.0;
    let mut max_idx = start;

    for i in (start + 1)..end {
        let p = &points[i];
        let dist_sq = if len_sq < 1e-6 {
            let ddx = p.x - p1.x;
            let ddy = p.y - p1.y;
            ddx * ddx + ddy * ddy
        } else {
            let t = ((p.x - p1.x) * dx + (p.y - p1.y) * dy) / len_sq;
            let t = t.clamp(0.0, 1.0);
            let proj_x = p1.x + t * dx;
            let proj_y = p1.y + t * dy;
            let ddx = p.x - proj_x;
            let ddy = p.y - proj_y;
            ddx * ddx + ddy * ddy
        };

        if dist_sq > max_dist_sq {
            max_dist_sq = dist_sq;
            max_idx = i;
        }
    }

    if max_dist_sq > tol_sq {
        keep[max_idx] = true;
        dp_recursive(points, keep, start, max_idx, tol_sq);
        dp_recursive(points, keep, max_idx, end, tol_sq);
    }
}
