use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum ObjectKind {
    Rectangle,
    Text,
    Image,
    Path,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PathPoint {
    pub x: f32,
    pub y: f32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }

    pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    pub fn from_hex(hex: &str) -> Self {
        let hex = hex.trim_start_matches('#');
        match hex.len() {
            6 => {
                let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0);
                let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0);
                let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0);
                Self::rgb(r, g, b)
            }
            8 => {
                let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0);
                let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0);
                let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0);
                let a = u8::from_str_radix(&hex[6..8], 16).unwrap_or(255);
                Self::rgba(r, g, b, a)
            }
            _ => Self::rgb(0, 0, 0),
        }
    }

    pub fn to_hex(&self) -> String {
        format!("#{:02X}{:02X}{:02X}", self.r, self.g, self.b)
    }
}

impl Default for Color {
    fn default() -> Self {
        Self::rgb(91, 124, 250)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Transform {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub rotation: f32,
    pub opacity: f32,
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            width: 200.0,
            height: 150.0,
            rotation: 0.0,
            opacity: 1.0,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ObjectData {
    pub id: String,
    pub kind: ObjectKind,
    pub name: String,
    pub visible: bool,
    pub transform: Transform,

    pub fill_color: Color,
    pub text_content: String,
    pub font_size: f32,
    pub text_color: Color,
    pub points: Vec<PathPoint>,
    pub closed: bool,
}

impl ObjectData {
    pub fn new(id: String, kind: ObjectKind) -> Self {
        let name = match kind {
            ObjectKind::Rectangle => "Rectangle".to_string(),
            ObjectKind::Text => "Text".to_string(),
            ObjectKind::Image => "Image".to_string(),
            ObjectKind::Path => "Path".to_string(),
        };

        let (fill_color, text_content, font_size, text_color, width, height) = match kind {
            ObjectKind::Rectangle => (
                Color::rgb(91, 124, 250),
                String::new(),
                32.0,
                Color::rgb(232, 232, 234),
                200.0,
                150.0,
            ),
            ObjectKind::Text => (
                Color::rgb(91, 124, 250),
                "Text".to_string(),
                48.0,
                Color::rgb(232, 232, 234),
                200.0,
                60.0,
            ),
            ObjectKind::Image => (
                Color::rgb(42, 42, 46),
                String::new(),
                32.0,
                Color::rgb(144, 144, 150),
                240.0,
                180.0,
            ),
            ObjectKind::Path => (
                Color::rgb(91, 124, 250),
                String::new(),
                32.0,
                Color::rgb(232, 232, 234),
                1.0,
                1.0,
            ),
        };

        Self {
            id,
            kind,
            name,
            visible: true,
            transform: Transform {
                width,
                height,
                ..Default::default()
            },
            fill_color,
            text_content,
            font_size,
            text_color,
            points: Vec::new(),
            closed: false,
        }
    }

    pub fn contains_point(&self, px: f32, py: f32) -> bool {
        let t = &self.transform;
        let cx = t.x + t.width / 2.0;
        let cy = t.y + t.height / 2.0;
        let angle = -t.rotation.to_radians();
        let cos = angle.cos();
        let sin = angle.sin();
        let dx = px - cx;
        let dy = py - cy;
        let rx = dx * cos - dy * sin;
        let ry = dx * sin + dy * cos;
        rx >= -t.width / 2.0
            && rx <= t.width / 2.0
            && ry >= -t.height / 2.0
            && ry <= t.height / 2.0
    }
}
