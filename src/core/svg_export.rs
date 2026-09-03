use crate::core::document::Document;
use crate::core::object::{Color, ObjectKind};

pub fn document_to_svg(doc: &Document) -> String {
    let mut svg = format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{}\" height=\"{}\" viewBox=\"0 0 {} {}\">\n",
        doc.width, doc.height, doc.width, doc.height
    );

    svg.push_str(&format!(
        "  <rect width=\"{}\" height=\"{}\" fill=\"#1E1E1E\"/>\n",
        doc.width, doc.height
    ));

    for obj in &doc.objects {
        if !obj.visible {
            continue;
        }
        svg.push_str(&object_to_svg(obj));
    }

    svg.push_str("</svg>\n");
    svg
}

fn color_to_svg(c: &Color) -> String {
    format!(
        "rgba({},{},{},{})",
        c.r, c.g, c.b,
        c.a as f32 / 255.0
    )
}

fn opacity_attr(opacity: f32) -> String {
    if opacity >= 1.0 {
        String::new()
    } else {
        format!(" opacity=\"{:.3}\"", opacity)
    }
}

fn object_to_svg(obj: &crate::core::object::ObjectData) -> String {
    let t = &obj.transform;
    let opacity = opacity_attr(t.opacity);
    let transform = if t.rotation != 0.0 {
        let cx = t.x + t.width / 2.0;
        let cy = t.y + t.height / 2.0;
        format!(
            " transform=\"rotate({} {} {})\"",
            t.rotation, cx, cy
        )
    } else {
        String::new()
    };

    match obj.kind {
        ObjectKind::Rectangle => {
            format!(
                "  <rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" fill=\"{}\"{}{}/>\n",
                t.x, t.y, t.width, t.height,
                color_to_svg(&obj.fill_color),
                opacity, transform
            )
        }
        ObjectKind::Text => {
            format!(
                "  <text x=\"{}\" y=\"{}\" font-size=\"{}\" fill=\"{}\"{}{}>{}</text>\n",
                t.x, t.y + obj.font_size,
                obj.font_size,
                color_to_svg(&obj.text_color),
                opacity, transform,
                escape_xml(&obj.text_content)
            )
        }
        ObjectKind::Image => {
            format!(
                "  <rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" fill=\"#2A2A2E\" stroke=\"#3A3A3E\" stroke-width=\"1\"{}{}/>\n",
                t.x, t.y, t.width, t.height,
                opacity, transform
            )
        }
        ObjectKind::Path => {
            if obj.points.len() >= 2 {
                let mut d = String::new();
                if let Some(first) = obj.points.first() {
                    d.push_str(&format!("M {} {}", first.x, first.y));
                }
                for p in obj.points.iter().skip(1) {
                    d.push_str(&format!(" L {} {}", p.x, p.y));
                }
                if obj.closed {
                    d.push_str(" Z");
                }
                let fill = if obj.closed { color_to_svg(&obj.fill_color) } else { "none".to_string() };
                format!(
                    "  <path d=\"{}\" fill=\"{}\" stroke=\"{}\" stroke-width=\"2\"{}{}/>\n",
                    d,
                    fill,
                    color_to_svg(&obj.fill_color),
                    opacity, transform
                )
            } else {
                String::new()
            }
        }
    }
}

fn escape_xml(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}
