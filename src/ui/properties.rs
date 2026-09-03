use iced::widget::{column, container, row, slider, text, text_input};
use iced::{Color, Element, Length};
use iced::alignment;

use crate::core::object::{ObjectData, ObjectKind};
use crate::ui::theme::Theme;

#[derive(Clone, Debug)]
pub enum PropertiesMessage {
    NameChanged(String),
    XChanged(String),
    YChanged(String),
    WidthChanged(String),
    HeightChanged(String),
    RotationChanged(String),
    OpacityChanged(String),
    FillColorInput(String),
    FillColorSubmit,
    FillOpacityChanged(f32),
    TextContentChanged(String),
    FontSizeChanged(String),
    TextColorInput(String),
    TextColorSubmit,
    TextOpacityChanged(f32),
}

#[derive(Clone, Debug)]
pub struct PropertyData {
    pub has_selection: bool,
    pub object_name: String,
    pub object_kind: String,
    pub x_str: String,
    pub y_str: String,
    pub width_str: String,
    pub height_str: String,
    pub rotation_str: String,
    pub opacity_str: String,
    pub fill_color_input: String,
    pub fill_color_valid: bool,
    pub fill_opacity: f32,
    pub text_content: String,
    pub font_size_str: String,
    pub text_color_input: String,
    pub text_color_valid: bool,
    pub text_opacity: f32,
    pub is_text: bool,
    pub is_rectangle: bool,
}

impl Default for PropertyData {
    fn default() -> Self {
        Self {
            has_selection: false,
            object_name: String::new(),
            object_kind: String::new(),
            x_str: "0".to_string(),
            y_str: "0".to_string(),
            width_str: "0".to_string(),
            height_str: "0".to_string(),
            rotation_str: "0".to_string(),
            opacity_str: "1".to_string(),
            fill_color_input: "#5B7CFA".to_string(),
            fill_color_valid: true,
            fill_opacity: 100.0,
            text_content: String::new(),
            font_size_str: "32".to_string(),
            text_color_input: "#E8E8EA".to_string(),
            text_color_valid: true,
            text_opacity: 100.0,
            is_text: false,
            is_rectangle: false,
        }
    }
}

fn hex_to_rgb6(s: &str) -> String {
    let clean = s.trim_start_matches('#');
    if clean.len() >= 6 {
        format!("#{}", &clean[..6])
    } else {
        s.to_string()
    }
}

impl PropertyData {
    pub fn from_object(obj: &ObjectData) -> Self {
        let fill_opacity = (obj.fill_color.a as f32 / 255.0) * 100.0;
        let text_opacity = (obj.text_color.a as f32 / 255.0) * 100.0;
        Self {
            has_selection: true,
            object_name: obj.name.clone(),
            object_kind: match obj.kind {
                ObjectKind::Rectangle => "rectangle".to_string(),
                ObjectKind::Text => "text".to_string(),
                ObjectKind::Image => "image".to_string(),
                ObjectKind::Path => "path".to_string(),
            },
            x_str: format!("{}", obj.transform.x),
            y_str: format!("{}", obj.transform.y),
            width_str: format!("{}", obj.transform.width),
            height_str: format!("{}", obj.transform.height),
            rotation_str: format!("{}", obj.transform.rotation),
            opacity_str: format!("{}", obj.transform.opacity),
            fill_color_input: hex_to_rgb6(&obj.fill_color.to_hex()),
            fill_color_valid: true,
            fill_opacity,
            text_content: obj.text_content.clone(),
            font_size_str: format!("{}", obj.font_size),
            text_color_input: hex_to_rgb6(&obj.text_color.to_hex()),
            text_color_valid: true,
            text_opacity,
            is_text: obj.kind == ObjectKind::Text,
            is_rectangle: obj.kind == ObjectKind::Rectangle,
        }
    }
}

pub fn view(prop: &PropertyData) -> Element<'_, PropertiesMessage> {
    if !prop.has_selection {
        let content = column![
            text("Properties").size(13).color(Theme::text_primary()),
            text("No object selected").size(11).color(Theme::text_tertiary()),
        ]
        .spacing(12)
        .padding(16);

        return container(content)
            .width(260)
            .height(Length::Fill)
            .style(|_theme| container::Style {
                background: Some(iced::Background::Color(Theme::bg_panel())),
                ..Default::default()
            })
            .into();
    }

    let mut sections = column![
        text("Properties").size(13).color(Theme::text_primary()),
    ]
    .spacing(16)
    .padding(16);

    // Name
    sections = sections.push(
        column![
            text("Name").size(10).color(Theme::text_tertiary()),
            text_input("Name", &prop.object_name)
                .size(12)
                .on_input(PropertiesMessage::NameChanged)
                .style(input_style),
        ]
        .spacing(4)
    );

    // Transform
    sections = sections.push(
        column![
            text("Transform").size(10).color(Theme::text_tertiary()),
            row![
                field_input("X", &prop.x_str, PropertiesMessage::XChanged),
                field_input("Y", &prop.y_str, PropertiesMessage::YChanged),
            ].spacing(8),
            row![
                field_input("W", &prop.width_str, PropertiesMessage::WidthChanged),
                field_input("H", &prop.height_str, PropertiesMessage::HeightChanged),
            ].spacing(8),
            row![
                field_input("R°", &prop.rotation_str, PropertiesMessage::RotationChanged),
                field_input("O", &prop.opacity_str, PropertiesMessage::OpacityChanged),
            ].spacing(8),
        ]
        .spacing(4)
    );

    // Appearance (for rectangle/path)
    if prop.is_rectangle || prop.object_kind == "path" {
        sections = sections.push(
            column![
                text("Appearance").size(10).color(Theme::text_tertiary()),
                row![
                    text("Fill").size(11).color(Theme::text_secondary()),
                    text_input("#HEX", &prop.fill_color_input)
                        .size(12)
                        .on_input(PropertiesMessage::FillColorInput)
                        .on_submit(PropertiesMessage::FillColorSubmit)
                        .style(if prop.fill_color_valid { input_style } else { invalid_input_style }),
                ].spacing(8),
                row![
                    text(format!("{:.0}%", prop.fill_opacity)).size(10).color(Theme::text_tertiary()).width(40),
                    slider(0.0..=100.0, prop.fill_opacity, PropertiesMessage::FillOpacityChanged)
                        .step(1.0),
                ].spacing(8),
            ]
            .spacing(4)
        );
    }

    // Text properties
    if prop.is_text {
        sections = sections.push(
            column![
                text("Text").size(10).color(Theme::text_tertiary()),
                text_input("Content", &prop.text_content)
                    .size(12)
                    .on_input(PropertiesMessage::TextContentChanged)
                    .style(input_style),
                row![
                    text("Size").size(11).color(Theme::text_secondary()),
                    text_input("32", &prop.font_size_str)
                        .size(12)
                        .on_input(PropertiesMessage::FontSizeChanged)
                        .style(input_style),
                ].spacing(8),
                row![
                    text("Color").size(11).color(Theme::text_secondary()),
                    text_input("#HEX", &prop.text_color_input)
                        .size(12)
                        .on_input(PropertiesMessage::TextColorInput)
                        .on_submit(PropertiesMessage::TextColorSubmit)
                        .style(if prop.text_color_valid { input_style } else { invalid_input_style }),
                ].spacing(8),
                row![
                    text(format!("{:.0}%", prop.text_opacity)).size(10).color(Theme::text_tertiary()).width(40),
                    slider(0.0..=100.0, prop.text_opacity, PropertiesMessage::TextOpacityChanged)
                        .step(1.0),
                ].spacing(8),
            ]
            .spacing(4)
        );
    }

    container(sections)
        .width(260)
        .height(Length::Fill)
        .style(|_theme| container::Style {
            background: Some(iced::Background::Color(Theme::bg_panel())),
            ..Default::default()
        })
        .into()
}

fn field_input<'a>(
    label: &'a str,
    value: &'a str,
    on_input: impl Fn(String) -> PropertiesMessage + 'a,
) -> Element<'a, PropertiesMessage> {
    row![
        text(label.to_string()).size(10).color(Theme::text_tertiary()).width(24),
        text_input("", value)
            .size(11)
            .on_input(on_input)
            .style(input_style),
    ]
    .spacing(4)
    .align_y(alignment::Vertical::Center)
    .into()
}

fn input_style(_theme: &iced::Theme, _status: text_input::Status) -> text_input::Style {
    text_input::Style {
        background: iced::Background::Color(Theme::bg_input()),
        border: iced::Border {
            color: Theme::border_subtle(),
            width: 1.0,
            radius: 3.0.into(),
        },
        value: Theme::text_primary(),
        placeholder: Theme::text_tertiary(),
        selection: Theme::accent_dim(),
        icon: Theme::text_secondary(),
    }
}

fn invalid_input_style(_theme: &iced::Theme, _status: text_input::Status) -> text_input::Style {
    text_input::Style {
        background: iced::Background::Color(Theme::bg_input()),
        border: iced::Border {
            color: Color::from_rgb(0.8, 0.2, 0.2),
            width: 1.0,
            radius: 3.0.into(),
        },
        value: Theme::text_primary(),
        placeholder: Theme::text_tertiary(),
        selection: Theme::accent_dim(),
        icon: Theme::text_secondary(),
    }
}
