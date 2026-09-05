use iced::Size;

use crate::ui::canvas::CanvasEvent;
use crate::ui::layers::LayersMessage;
use crate::ui::menu_bar::MenuBarMessage;
use crate::ui::properties::PropertiesMessage;
use crate::ui::status_bar::StatusBarMessage;
use crate::ui::toolbar::ToolbarMessage;

#[derive(Debug, Clone)]
pub enum Message {
    Toolbar(ToolbarMessage),
    MenuBar(MenuBarMessage),
    Canvas(CanvasEvent),
    Layers(LayersMessage),
    Properties(PropertiesMessage),
    StatusBar(StatusBarMessage),
    WindowResized(Size),
    EscapePressed,
}
