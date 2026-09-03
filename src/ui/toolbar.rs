use iced::widget::{button, container, row, svg, text};
use iced::{Color, Element, Length};
use iced::alignment;

use crate::ui::theme::Theme;

#[derive(Clone, Debug)]
pub enum ToolbarMessage {
    ToolSelected(String),
    Undo,
    Redo,
    NewProject,
    OpenProject,
    SaveProject,
    Export,
}

pub fn view(active_tool: &str, _can_undo: bool, _can_redo: bool) -> Element<'static, ToolbarMessage> {
    let tools = row![
        tool_button_svg("assets/icons/select.svg", "Select", active_tool == "select", ToolbarMessage::ToolSelected("select".into())),
        tool_button("R", "Rectangle", active_tool == "rectangle", ToolbarMessage::ToolSelected("rectangle".into())),
        tool_button("T", "Text", active_tool == "text", ToolbarMessage::ToolSelected("text".into())),
        tool_button("I", "Image", active_tool == "image", ToolbarMessage::ToolSelected("image".into())),
        tool_button("P", "Pen", active_tool == "pen", ToolbarMessage::ToolSelected("pen".into())),
    ]
    .spacing(2)
    .padding([0, 12]);

    let content = row![
        tools,
        container(row![].spacing(16))
            .width(Length::Fill)
            .align_x(alignment::Horizontal::Center)
    ]
    .align_y(alignment::Vertical::Center);

    container(content)
        .height(44)
        .width(Length::Fill)
        .style(|_theme| container::Style {
            background: Some(iced::Background::Color(Theme::bg_panel())),
            ..Default::default()
        })
        .align_y(alignment::Vertical::Center)
        .into()
}

fn tool_button(icon: &str, label: &str, active: bool, on_press: ToolbarMessage) -> Element<'static, ToolbarMessage> {
    let bg = if active { Theme::bg_selected() } else { Theme::bg_panel() };
    let border_color = if active { Theme::accent_dim() } else { Color::TRANSPARENT };
    let icon_color = if active { Theme::accent() } else { Theme::text_secondary() };
    let label_color = if active { Theme::text_primary() } else { Theme::text_secondary() };

    button(
        iced::widget::column![
            text(icon.to_string()).size(18).color(icon_color).align_x(alignment::Horizontal::Center),
            text(label.to_string()).size(10).color(label_color).align_x(alignment::Horizontal::Center),
        ]
        .spacing(4)
        .align_x(alignment::Horizontal::Center),
    )
    .padding([6, 8])
    .style(move |_theme, _status| button::Style {
        background: Some(iced::Background::Color(bg)),
        border: iced::Border {
            color: border_color,
            width: if active { 1.0 } else { 0.0 },
            radius: 5.0.into(),
        },
        ..Default::default()
    })
    .on_press(on_press)
    .into()
}

fn tool_button_svg(path: &str, label: &str, active: bool, on_press: ToolbarMessage) -> Element<'static, ToolbarMessage> {
    let bg = if active { Theme::bg_selected() } else { Theme::bg_panel() };
    let border_color = if active { Theme::accent_dim() } else { Color::TRANSPARENT };
    let label_color = if active { Theme::text_primary() } else { Theme::text_secondary() };
    let icon_color = if active { Theme::accent() } else { Theme::text_secondary() };

    let handle = svg::Handle::from_path(path);
    let svg_widget = svg(handle)
        .width(Length::Fixed(18.0))
        .height(Length::Fixed(18.0))
        .style(move |_theme, _status| svg::Style {
            color: Some(icon_color),
        });

    button(
        iced::widget::column![
            svg_widget,
            text(label.to_string()).size(10).color(label_color).align_x(alignment::Horizontal::Center),
        ]
        .spacing(4)
        .align_x(alignment::Horizontal::Center),
    )
    .padding([6, 8])
    .style(move |_theme, _status| button::Style {
        background: Some(iced::Background::Color(bg)),
        border: iced::Border {
            color: border_color,
            width: if active { 1.0 } else { 0.0 },
            radius: 5.0.into(),
        },
        ..Default::default()
    })
    .on_press(on_press)
    .into()
}
