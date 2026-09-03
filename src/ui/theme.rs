use iced::Color;

pub struct Theme;

impl Theme {
    pub fn bg_app() -> Color {
        Color::from_rgb(0.10, 0.10, 0.11)
    }

    pub fn bg_panel() -> Color {
        Color::from_rgb(0.125, 0.125, 0.133)
    }

    pub fn bg_panel_alt() -> Color {
        Color::from_rgb(0.149, 0.149, 0.16)
    }

    pub fn bg_elevated() -> Color {
        Color::from_rgb(0.176, 0.176, 0.188)
    }

    pub fn bg_canvas() -> Color {
        Color::from_rgb(0.086, 0.086, 0.094)
    }

    pub fn bg_input() -> Color {
        Color::from_rgb(0.117, 0.117, 0.125)
    }

    pub fn bg_hover() -> Color {
        Color::from_rgb(0.20, 0.20, 0.21)
    }

    pub fn bg_selected() -> Color {
        Color::from_rgb(0.165, 0.165, 0.24)
    }

    pub fn border_subtle() -> Color {
        Color::from_rgb(0.18, 0.18, 0.196)
    }

    pub fn border_default() -> Color {
        Color::from_rgb(0.227, 0.227, 0.243)
    }

    pub fn text_primary() -> Color {
        Color::from_rgb(0.91, 0.91, 0.918)
    }

    pub fn text_secondary() -> Color {
        Color::from_rgb(0.565, 0.565, 0.588)
    }

    pub fn text_tertiary() -> Color {
        Color::from_rgb(0.353, 0.353, 0.376)
    }

    pub fn accent() -> Color {
        Color::from_rgb(0.357, 0.486, 0.98)
    }

    pub fn accent_dim() -> Color {
        Color::from_rgb(0.227, 0.329, 0.549)
    }

    pub fn selection() -> Color {
        Color::from_rgb(0.357, 0.486, 0.98)
    }

    pub fn danger() -> Color {
        Color::from_rgb(0.878, 0.333, 0.333)
    }

    pub fn success() -> Color {
        Color::from_rgb(0.361, 0.722, 0.361)
    }

    pub fn doc_bg() -> Color {
        Color::from_rgb(0.117, 0.117, 0.125)
    }

    pub fn doc_border() -> Color {
        Color::from_rgba(1.0, 1.0, 1.0, 0.13)
    }

    pub fn doc_shadow() -> Color {
        Color::from_rgba(0.0, 0.0, 0.0, 0.38)
    }
}
