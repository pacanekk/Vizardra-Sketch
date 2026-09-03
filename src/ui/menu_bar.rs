use iced::widget::{button, text};
use iced::{Element, Length};
use iced::alignment;

use iced_aw::menu::{Item, Menu};
use iced_aw::menu_bar;

use crate::ui::theme::Theme;

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
    .width(160.0)
    .offset(0.0)
    .spacing(0.0);

    let edit_menu = Menu::new(vec![
        Item::new(menu_item("Undo", MenuBarMessage::Undo)),
        Item::new(menu_item("Redo", MenuBarMessage::Redo)),
    ])
    .width(160.0)
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
    .close_on_background_click(true);

    mb.into()
}

fn menu_header(label: &str) -> Element<'static, MenuBarMessage> {
    button(
        text(label.to_string()).size(12).color(Theme::text_secondary()),
    )
    .padding([4, 10])
    .style(|_theme, status| {
        let hovered = matches!(status, button::Status::Hovered);
        button::Style {
            background: Some(iced::Background::Color(
                if hovered { Theme::bg_hover() } else { iced::Color::TRANSPARENT }
            )),
            border: iced::Border::default(),
            ..Default::default()
        }
    })
    .into()
}

fn menu_item(label: &str, msg: MenuBarMessage) -> Element<'static, MenuBarMessage> {
    button(
        text(label.to_string())
            .size(12)
            .color(Theme::text_secondary())
            .align_x(alignment::Horizontal::Left)
            .width(Length::Fill),
    )
    .width(Length::Fill)
    .padding([6, 16])
    .style(|_theme, status| {
        let hovered = matches!(status, button::Status::Hovered);
        button::Style {
            background: Some(iced::Background::Color(
                if hovered { Theme::bg_hover() } else { Theme::bg_panel() }
            )),
            border: iced::Border::default(),
            ..Default::default()
        }
    })
    .on_press(msg)
    .into()
}
