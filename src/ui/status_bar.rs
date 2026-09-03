use iced::widget::{button, container, row, slider, svg, text};
use iced::{Element, Length};
use iced::alignment;

use crate::ui::theme::Theme;

#[derive(Clone, Debug)]
pub enum StatusBarMessage {
    ZoomIn,
    ZoomOut,
    ZoomReset,
    SmoothingChanged(f32),
}

pub fn view(status_text: &str, doc_size: &str, zoom_text: &str, smoothing: f32) -> Element<'static, StatusBarMessage> {
    let reset_handle = svg::Handle::from_path("assets/icons/zoom-reset.svg");
    let reset_icon = svg(reset_handle)
        .width(Length::Fixed(12.0))
        .height(Length::Fixed(12.0))
        .style(move |_theme, _status| svg::Style {
            color: Some(Theme::text_secondary()),
        });

    let smoothing_label = text("Smoothing").size(10).color(Theme::text_tertiary());
    let smoothing_slider = slider(0.0..=1.0, smoothing, StatusBarMessage::SmoothingChanged)
        .step(0.05)
        .width(Length::Fixed(80.0));
    let smoothing_pct = text(format!("{:.0}%", smoothing * 100.0)).size(10).color(Theme::text_secondary()).width(32);

    let content = row![
        text(status_text.to_string()).size(11).color(Theme::text_secondary()),
        container(
            text(doc_size.to_string()).size(11).color(Theme::text_tertiary())
        ).width(Length::Fill).align_x(alignment::Horizontal::Center),
        row![
            smoothing_label,
            smoothing_slider,
            smoothing_pct,
            text("  ").width(8),
            zoom_btn("−", StatusBarMessage::ZoomOut),
            text(zoom_text.to_string()).size(11).color(Theme::text_secondary()).width(48).align_x(alignment::Horizontal::Center),
            zoom_btn("+", StatusBarMessage::ZoomIn),
            button(reset_icon)
                .padding([4, 6])
                .style(|_theme, _status| button::Style {
                    background: Some(iced::Background::Color(iced::Color::TRANSPARENT)),
                    ..Default::default()
                })
                .on_press(StatusBarMessage::ZoomReset),
        ]
        .spacing(4)
        .align_y(alignment::Vertical::Center),
    ]
    .align_y(alignment::Vertical::Center)
    .padding([0, 12]);

    container(content)
        .height(28)
        .width(Length::Fill)
        .style(|_theme| container::Style {
            background: Some(iced::Background::Color(Theme::bg_panel())),
            ..Default::default()
        })
        .align_y(alignment::Vertical::Center)
        .into()
}

fn zoom_btn(icon: &str, on_press: StatusBarMessage) -> button::Button<'static, StatusBarMessage> {
    button(text(icon.to_string()).size(12).color(Theme::text_secondary()))
        .padding([2, 6])
        .style(|_theme, _status| button::Style {
            background: Some(iced::Background::Color(iced::Color::TRANSPARENT)),
            ..Default::default()
        })
        .on_press(on_press)
}
