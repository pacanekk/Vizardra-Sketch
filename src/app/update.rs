use iced::mouse;
use iced::Task;

use super::message::Message;
use super::state::AppState;
use crate::ui::canvas::CanvasEvent;
use crate::ui::layers::LayersMessage;
use crate::ui::properties::PropertiesMessage;
use crate::ui::status_bar::StatusBarMessage;
use crate::ui::toolbar::ToolbarMessage;

impl AppState {
    pub fn update(&mut self, message: Message) -> Task<Message> {
        let result = self.handle_message(message);
        self.refresh_cache();
        result
    }

    fn handle_message(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Toolbar(msg) => {
                match msg {
                    ToolbarMessage::ToolSelected(tool) => self.set_tool(&tool),
                }
                Task::none()
            }
            Message::MenuBar(msg) => {
                match msg {
                    crate::ui::menu_bar::MenuBarMessage::NewProject => {
                        self.new_project();
                    }
                    crate::ui::menu_bar::MenuBarMessage::OpenProject => {
                        return self.open_project();
                    }
                    crate::ui::menu_bar::MenuBarMessage::SaveProject => {
                        return self.save_project();
                    }
                    crate::ui::menu_bar::MenuBarMessage::ExportPng => {
                        self.export_png();
                    }
                    crate::ui::menu_bar::MenuBarMessage::ExportSvg => {
                        self.export_svg();
                    }
                    crate::ui::menu_bar::MenuBarMessage::Undo => {
                        self.undo();
                    }
                    crate::ui::menu_bar::MenuBarMessage::Redo => {
                        self.redo();
                    }
                }
                Task::none()
            }
            Message::Canvas(event) => {
                match event {
                    CanvasEvent::Pressed { pos, button } => {
                        if button == mouse::Button::Middle {
                            self.handle_pan_start(pos.x, pos.y);
                        } else if button == mouse::Button::Left {
                            self.handle_canvas_press(pos.x, pos.y);
                        }
                    }
                    CanvasEvent::Moved(pos) => {
                        if self.canvas.is_panning {
                            self.handle_pan_move(pos.x, pos.y);
                        } else {
                            self.handle_canvas_move(pos.x, pos.y);
                        }
                    }
                    CanvasEvent::Released { pos, button } => {
                        if button == mouse::Button::Middle {
                            self.handle_pan_end();
                        } else if button == mouse::Button::Left {
                            self.handle_canvas_release(pos.x, pos.y);
                        }
                    }
                    CanvasEvent::Scrolled { delta, modifiers, cursor_pos } => {
                        if modifiers.alt {
                            self.handle_scroll(cursor_pos.x, cursor_pos.y, delta.y);
                        } else if modifiers.control {
                            self.handle_pan_delta(delta.y, 0.0);
                        } else if modifiers.shift {
                            self.handle_pan_delta(delta.y, 0.0);
                        } else {
                            self.handle_pan_delta(0.0, delta.y);
                        }
                    }
                }
                Task::none()
            }
            Message::Layers(msg) => {
                match msg {
                    LayersMessage::LayerSelected(id) => self.select_object(&id),
                    LayersMessage::LayerDoubleClicked(id) => {
                        if self.editing_layer_id.as_ref() == Some(&id) {
                            self.editing_layer_id = None;
                        } else {
                            self.select_object(&id);
                            self.editing_layer_id = Some(id);
                        }
                    }
                    LayersMessage::LayerNameChanged(id, name) => self.rename_object(&id, &name),
                    LayersMessage::LayerNameSubmitted => {
                        self.editing_layer_id = None;
                    }
                    LayersMessage::LayerVisibilityToggled(id) => self.toggle_visibility(&id),
                }
                Task::none()
            }
            Message::Properties(msg) => {
                match msg {
                    PropertiesMessage::NameChanged(name) => self.update_object_name(&name),
                    PropertiesMessage::XChanged(v) => {
                        if let Ok(val) = v.parse::<f32>() { self.update_property("x", val); }
                    }
                    PropertiesMessage::YChanged(v) => {
                        if let Ok(val) = v.parse::<f32>() { self.update_property("y", val); }
                    }
                    PropertiesMessage::WidthChanged(v) => {
                        if let Ok(val) = v.parse::<f32>() { self.update_property("width", val); }
                    }
                    PropertiesMessage::HeightChanged(v) => {
                        if let Ok(val) = v.parse::<f32>() { self.update_property("height", val); }
                    }
                    PropertiesMessage::RotationChanged(v) => {
                        if let Ok(val) = v.parse::<f32>() { self.update_property("rotation", val); }
                    }
                    PropertiesMessage::OpacityChanged(v) => {
                        if let Ok(val) = v.parse::<f32>() { self.update_property("opacity", val); }
                    }
                    PropertiesMessage::FillColorInput(hex) => self.handle_fill_color_input(&hex),
                    PropertiesMessage::FillColorSubmit => self.handle_fill_color_submit(),
                    PropertiesMessage::FillOpacityChanged(opacity) => {
                        self.property_data.fill_opacity = opacity;
                        let hex = self.property_data.fill_color_input.clone();
                        if crate::app::utils::is_valid_hex(&hex) {
                            self.update_fill_color_with_opacity(&hex, opacity);
                        }
                    }
                    PropertiesMessage::TextContentChanged(content) => self.update_text_content(&content),
                    PropertiesMessage::FontSizeChanged(v) => {
                        if let Ok(val) = v.parse::<f32>() { self.update_font_size(val); }
                    }
                    PropertiesMessage::TextColorInput(hex) => self.handle_text_color_input(&hex),
                    PropertiesMessage::TextColorSubmit => self.handle_text_color_submit(),
                    PropertiesMessage::TextOpacityChanged(opacity) => {
                        self.property_data.text_opacity = opacity;
                        let hex = self.property_data.text_color_input.clone();
                        if crate::app::utils::is_valid_hex(&hex) {
                            self.update_text_color_with_opacity(&hex, opacity);
                        }
                    }
                }
                Task::none()
            }
            Message::StatusBar(msg) => {
                match msg {
                    StatusBarMessage::ZoomIn => self.zoom_in(),
                    StatusBarMessage::ZoomOut => self.zoom_out(),
                    StatusBarMessage::ZoomReset => self.zoom_reset(),
                }
                Task::none()
            }
            Message::WindowResized(size) => {
                self.window_size = (size.width, size.height);
                self.fit_to_screen(size.width, size.height);
                self.needs_fit = false;
                Task::none()
            }
            Message::EscapePressed => {
                if self.canvas.is_drawing_path {
                    if self.active_tool == "pen" {
                        self.finalize_path();
                    } else {
                        self.canvas.is_drawing_path = false;
                        self.canvas.temp_draw_points.clear();
                    }
                }
                Task::none()
            }
        }
    }
}
