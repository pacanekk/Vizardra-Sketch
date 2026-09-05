use iced::widget::{column, row};
use iced::{Element, Length};

use super::message::Message;
use super::state::AppState;
use crate::ui::canvas::CanvasWidget;

impl AppState {
    pub fn view(&self) -> Element<'_, Message> {
        let menu_bar = crate::ui::menu_bar::view(self.can_undo(), self.can_redo())
            .map(Message::MenuBar);

        let toolbar = crate::ui::toolbar::view(&self.active_tool, self.can_undo(), self.can_redo())
            .map(Message::Toolbar);

        let layers = crate::ui::layers::view(&self.document, &self.selected_id, &self.editing_layer_id)
            .map(Message::Layers);

        let canvas = CanvasWidget {
            document: &self.document,
            canvas_state: &self.canvas,
            selected_id: &self.selected_id,
            active_tool: &self.active_tool,
        }
        .view()
        .map(Message::Canvas);

        let properties = crate::ui::properties::view(&self.property_data)
            .map(Message::Properties);

        let status_bar = crate::ui::status_bar::view(
            &self.status_text,
            &self.doc_size,
            &self.zoom_text,
            self.canvas.smoothing,
        )
        .map(Message::StatusBar);

        let main = row![layers, canvas, properties]
            .width(Length::Fill)
            .height(Length::Fill);

        column![menu_bar, toolbar, main, status_bar]
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}
