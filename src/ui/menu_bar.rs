use iced::widget::{button, container, text};
use iced::{Element, Length};
use iced::alignment;
use iced::Border;
use iced::Shadow;

use iced_aw::menu::{Item, Menu};
use iced_aw::menu_bar;
use iced_aw::style::menu_bar::Style;

use crate::ui::theme::Theme as AppTheme;

#[derive(Clone, Debug)]
pub enum MenuBarMessage {
    NewProject,
    OpenProject,
    SaveProject,
    Export,
    Undo,
    Redo,
}

pub fn view(_can_undo: bool, _can_redo: bool) -> Element<'static, MenuBarMessage> {
    let file_menu = Menu::new(vec![
        Item::new(menu_item("New Project", MenuBarMessage::NewProject)),
        Item::new(menu_item("Open Project", MenuBarMessage::OpenProject)),
        Item::new(menu_item("Save Project", MenuBarMessage::SaveProject)),
        Item::new(menu_item("Export PNG", MenuBarMessage::Export)),
    ])
    .width(180.0)
    .offset(0.0)
    .spacing(0.0);

    let edit_menu = Menu::new(vec![
        Item::new(menu_item("Undo", MenuBarMessage::Undo)),
        Item::new(menu_item("Redo", MenuBarMessage::Redo)),
    ])
    .width(180.0)
    .offset(0.0)
    .spacing(0.0);

    let mb = menu_bar!(
        (menu_header("File"), file_menu),
        (menu_header("Edit"), edit_menu),
    )
    .width(Length::Shrink)
    .height(Length::Fixed(28.0))
    .spacing(0.0)
    .close_on_item_click(true)
    .close_on_background_click(true)
    .style(|_theme, _status| Style {
        bar_background: iced::Background::Color(AppTheme::bg_panel()),
        bar_border: Border {
            radius: 0.0.into(),
            ..Default::default()
        },
        bar_shadow: Shadow::default(),
        menu_background: iced::Background::Color(AppTheme::bg_elevated()),
        menu_border: Border {
            color: AppTheme::border_subtle(),
            width: 1.0,
            radius: 6.0.into(),
        },
        menu_shadow: Shadow {
            color: iced::Color::from_rgba(0.0, 0.0, 0.0, 0.4),
            offset: iced::Vector::new(0.0, 4.0),
            blur_radius: 12.0,
        },
        path: iced::Background::Color(AppTheme::bg_hover()),
        path_border: Border {
            radius: 4.0.into(),
            ..Default::default()
        },
    });

    container(mb)
        .padding(iced::Padding::new(0.0).left(8.0))
        .width(Length::Fill)
        .height(Length::Fixed(28.0))
        .style(|_theme| container::Style {
            background: Some(iced::Background::Color(AppTheme::bg_panel())),
            ..Default::default()
        })
        .into()
}

fn menu_header(label: &str) -> Element<'static, MenuBarMessage> {
    button(
        text(label.to_string()).size(12).color(AppTheme::text_secondary()),
    )
    .padding(iced::Padding::new(4.0).horizontal(10.0))
    .style(|_theme, status| {
        let hovered = matches!(status, button::Status::Hovered);
        button::Style {
            background: Some(iced::Background::Color(
                if hovered { AppTheme::bg_hover() } else { iced::Color::TRANSPARENT }
            )),
            border: Border::default(),
            ..Default::default()
        }
    })
    .into()
}

fn menu_item(label: &str, msg: MenuBarMessage) -> Element<'static, MenuBarMessage> {
    button(
        text(label.to_string())
            .size(12)
            .color(AppTheme::text_secondary())
            .align_x(alignment::Horizontal::Left)
            .width(Length::Fill),
    )
    .width(Length::Fill)
    .padding(iced::Padding::new(8.0).horizontal(16.0))
    .style(|_theme, status| {
        let hovered = matches!(status, button::Status::Hovered);
        button::Style {
            background: Some(iced::Background::Color(
                if hovered { AppTheme::bg_selected() } else { iced::Color::TRANSPARENT }
            )),
            border: Border::default(),
            text_color: if hovered { AppTheme::text_primary() } else { AppTheme::text_secondary() },
            ..Default::default()
        }
    })
    .on_press(msg)
    .into()
}
