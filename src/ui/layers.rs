use iced::widget::{button, column, container, row, scrollable, svg, text, text_input};
use iced::{Color, Element, Length};
use iced::alignment;

use crate::core::document::Document;
use crate::ui::theme::Theme;

#[derive(Clone, Debug)]
pub enum LayersMessage {
    LayerSelected(String),
    LayerDoubleClicked(String),
    LayerNameChanged(String, String),
    LayerNameSubmitted,
    LayerVisibilityToggled(String),
}

pub fn view<'a>(
    doc: &'a Document,
    selected_id: &'a Option<String>,
    editing_layer_id: &'a Option<String>,
) -> Element<'a, LayersMessage> {
    let title = text("Layers")
        .size(13)
        .color(Theme::text_primary())
        .align_x(alignment::Horizontal::Left);

    let mut layers = column![].spacing(2).padding([8, 0]);

    for obj in doc.objects.iter().rev() {
        let is_selected = selected_id.as_ref() == Some(&obj.id);
        let is_editing = editing_layer_id.as_ref() == Some(&obj.id);
        let id = obj.id.clone();
        let name = obj.name.clone();
        let visible = obj.visible;

        let icon = match obj.kind {
            crate::core::object::ObjectKind::Rectangle => "R",
            crate::core::object::ObjectKind::Text => "T",
            crate::core::object::ObjectKind::Image => "I",
            crate::core::object::ObjectKind::Path => "P",
        };

        let vis_icon = if visible { "assets/icons/eye.svg" } else { "assets/icons/eye-slash.svg" };
        let vis_handle = svg::Handle::from_path(vis_icon);
        let vis_btn = button(
                svg(vis_handle)
                    .width(Length::Fixed(14.0))
                    .height(Length::Fixed(14.0))
                    .style(move |_theme, _status| svg::Style {
                        color: Some(Theme::text_tertiary()),
                    })
            )
            .padding([2, 4])
            .style(|_theme, _status| button::Style {
                background: Some(iced::Background::Color(Color::TRANSPARENT)),
                ..Default::default()
            })
            .on_press(LayersMessage::LayerVisibilityToggled(id.clone()));

        let bg = if is_selected { Theme::bg_selected() } else { Color::TRANSPARENT };

        let layer_btn = if is_editing {
            let name_for_input = name.clone();
            let id_for_input = id.clone();
            button(
                row![
                    text(icon.to_string()).size(12).color(Theme::accent()),
                    text_input("", &name_for_input)
                        .size(12)
                        .on_input(move |s| LayersMessage::LayerNameChanged(id_for_input.clone(), s))
                        .on_submit(LayersMessage::LayerNameSubmitted)
                        .style(input_style),
                    vis_btn,
                ]
                .spacing(8)
                .align_y(alignment::Vertical::Center),
            )
            .padding([6, 8])
            .width(Length::Fill)
            .style(move |_theme, _status| button::Style {
                background: Some(iced::Background::Color(bg)),
                border: iced::Border {
                    color: if is_selected { Theme::accent_dim() } else { Color::TRANSPARENT },
                    width: if is_selected { 1.0 } else { 0.0 },
                    radius: 3.0.into(),
                },
                ..Default::default()
            })
            .on_press(LayersMessage::LayerNameSubmitted)
        } else {
            let id_for_double = id.clone();
            button(
                row![
                    text(icon.to_string()).size(12).color(Theme::accent()),
                    text(name).size(12).color(if is_selected { Theme::text_primary() } else { Theme::text_secondary() }),
                    vis_btn,
                ]
                .spacing(8)
                .align_y(alignment::Vertical::Center),
            )
            .padding([6, 8])
            .width(Length::Fill)
            .style(move |_theme, _status| button::Style {
                background: Some(iced::Background::Color(bg)),
                border: iced::Border {
                    color: if is_selected { Theme::accent_dim() } else { Color::TRANSPARENT },
                    width: if is_selected { 1.0 } else { 0.0 },
                    radius: 3.0.into(),
                },
                ..Default::default()
            })
            .on_press(LayersMessage::LayerDoubleClicked(id_for_double))
        };

        layers = layers.push(layer_btn);
    }

    let content = column![title, scrollable(layers).height(Length::Fill)]
        .spacing(12)
        .padding(16);

    container(content)
        .width(220)
        .height(Length::Fill)
        .style(|_theme| container::Style {
            background: Some(iced::Background::Color(Theme::bg_panel())),
            ..Default::default()
        })
        .into()
}

fn input_style(_theme: &iced::Theme, _status: text_input::Status) -> text_input::Style {
    text_input::Style {
        background: iced::Background::Color(Theme::bg_input()),
        border: iced::Border {
            color: Theme::accent_dim(),
            width: 1.0,
            radius: 3.0.into(),
        },
        value: Theme::text_primary(),
        placeholder: Theme::text_tertiary(),
        selection: Theme::accent_dim(),
        icon: Theme::text_secondary(),
    }
}
