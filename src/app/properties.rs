use super::state::AppState;
use crate::core::object::{Color, ObjectKind};
use crate::ui::properties::PropertyData;
use crate::app::utils::{measure_text, hex_to_rgb6, is_valid_hex};

impl AppState {
    pub fn set_tool(&mut self, tool: &str) {
        if tool != "draw" && tool != "pen" && self.canvas.is_drawing_path {
            self.finalize_path();
        }
        self.active_tool = tool.to_string();
        if tool != "select" {
            self.selected_id = None;
        }
    }

    pub fn select_object(&mut self, id: &str) {
        self.selected_id = Some(id.to_string());
    }

    pub fn rename_object(&mut self, id: &str, name: &str) {
        if let Some(obj) = self.document.get_object_mut(id) {
            obj.name = name.to_string();
        }
    }

    pub fn toggle_visibility(&mut self, id: &str) {
        if let Some(obj) = self.document.get_object_mut(id) {
            obj.visible = !obj.visible;
        }
    }

    pub fn update_property(&mut self, key: &str, value: f32) {
        if let Some(ref id) = self.selected_id {
            if let Some(obj) = self.document.get_object_mut(id) {
                match key {
                    "x" => obj.transform.x = value,
                    "y" => obj.transform.y = value,
                    "width" => obj.transform.width = value.max(1.0),
                    "height" => obj.transform.height = value.max(1.0),
                    "rotation" => obj.transform.rotation = value,
                    "opacity" => obj.transform.opacity = value.clamp(0.0, 1.0),
                    _ => {}
                }
            }
        }
    }

    pub fn update_fill_color_with_opacity(&mut self, hex: &str, opacity_pct: f32) {
        if let Some(ref id) = self.selected_id {
            if let Some(obj) = self.document.get_object_mut(id) {
                let mut color = Color::from_hex(hex);
                color.a = ((opacity_pct / 100.0) * 255.0).round() as u8;
                obj.fill_color = color;
            }
        }
    }

    pub fn update_text_color_with_opacity(&mut self, hex: &str, opacity_pct: f32) {
        if let Some(ref id) = self.selected_id {
            if let Some(obj) = self.document.get_object_mut(id) {
                let mut color = Color::from_hex(hex);
                color.a = ((opacity_pct / 100.0) * 255.0).round() as u8;
                obj.text_color = color;
            }
        }
    }

    pub fn update_text_content(&mut self, content: &str) {
        if let Some(ref id) = self.selected_id {
            if let Some(obj) = self.document.get_object_mut(id) {
                obj.text_content = content.to_string();
                if obj.kind == ObjectKind::Text {
                    let (w, h) = measure_text(&obj.text_content, obj.font_size);
                    obj.transform.width = w;
                    obj.transform.height = h;
                }
            }
        }
    }

    pub fn update_font_size(&mut self, size: f32) {
        if let Some(ref id) = self.selected_id {
            if let Some(obj) = self.document.get_object_mut(id) {
                obj.font_size = size.max(1.0);
                if obj.kind == ObjectKind::Text {
                    let (w, h) = measure_text(&obj.text_content, obj.font_size);
                    obj.transform.width = w;
                    obj.transform.height = h;
                }
            }
        }
    }

    pub fn update_object_name(&mut self, name: &str) {
        if let Some(ref id) = self.selected_id {
            if let Some(obj) = self.document.get_object_mut(id) {
                obj.name = name.to_string();
            }
        }
    }

    pub fn update_path_bbox(obj: &mut crate::core::object::ObjectData) {
        if obj.points.is_empty() {
            return;
        }
        let min_x = obj.points.iter().map(|p| p.x).fold(f32::INFINITY, f32::min);
        let min_y = obj.points.iter().map(|p| p.y).fold(f32::INFINITY, f32::min);
        let max_x = obj.points.iter().map(|p| p.x).fold(f32::NEG_INFINITY, f32::max);
        let max_y = obj.points.iter().map(|p| p.y).fold(f32::NEG_INFINITY, f32::max);
        obj.transform.x = min_x;
        obj.transform.y = min_y;
        obj.transform.width = (max_x - min_x).max(1.0);
        obj.transform.height = (max_y - min_y).max(1.0);
    }

    pub fn get_property_data(&self) -> PropertyData {
        if let Some(ref id) = self.selected_id {
            if let Some(obj) = self.document.get_object(id) {
                return PropertyData::from_object(obj);
            }
        }
        PropertyData::default()
    }

    pub fn refresh_cache(&mut self) {
        let prev_fill_input = self.property_data.fill_color_input.clone();
        let prev_fill_valid = self.property_data.fill_color_valid;
        let prev_fill_opacity = self.property_data.fill_opacity;
        let prev_text_input = self.property_data.text_color_input.clone();
        let prev_text_valid = self.property_data.text_color_valid;
        let prev_text_opacity = self.property_data.text_opacity;
        let prev_sel = self.property_data.has_selection;
        let prev_id = self.selected_id.clone();
        let selection_changed = prev_sel != prev_id.is_some()
            || self.property_data.object_name.is_empty() && prev_id.is_some();

        self.property_data = self.get_property_data();

        if !selection_changed {
            self.property_data.fill_color_input = prev_fill_input;
            self.property_data.fill_color_valid = prev_fill_valid;
            self.property_data.fill_opacity = prev_fill_opacity;
            self.property_data.text_color_input = prev_text_input;
            self.property_data.text_color_valid = prev_text_valid;
            self.property_data.text_opacity = prev_text_opacity;
        }

        self.doc_size = format!("{} × {}", self.document.width, self.document.height);
    }

    pub fn handle_fill_color_input(&mut self, hex: &str) {
        self.property_data.fill_color_input = hex.to_string();
        self.property_data.fill_color_valid = is_valid_hex(hex);
        if self.property_data.fill_color_valid {
            let clean = hex.trim_start_matches('#');
            if clean.len() == 8 {
                self.property_data.fill_color_input = format!("#{}", &clean[..6]);
                self.property_data.fill_opacity =
                    (u8::from_str_radix(&clean[6..8], 16).unwrap_or(255) as f32 / 255.0) * 100.0;
            }
            self.update_fill_color_with_opacity(hex, self.property_data.fill_opacity);
        }
    }

    pub fn handle_fill_color_submit(&mut self) {
        if self.property_data.fill_color_valid {
            if let Some(ref id) = self.selected_id {
                if let Some(obj) = self.document.get_object(id) {
                    self.property_data.fill_color_input = hex_to_rgb6(&obj.fill_color.to_hex());
                }
            }
        } else if let Some(ref id) = self.selected_id {
            if let Some(obj) = self.document.get_object(id) {
                self.property_data.fill_color_input = hex_to_rgb6(&obj.fill_color.to_hex());
                self.property_data.fill_color_valid = true;
            }
        }
    }

    pub fn handle_text_color_input(&mut self, hex: &str) {
        self.property_data.text_color_input = hex.to_string();
        self.property_data.text_color_valid = is_valid_hex(hex);
        if self.property_data.text_color_valid {
            let clean = hex.trim_start_matches('#');
            if clean.len() == 8 {
                self.property_data.text_color_input = format!("#{}", &clean[..6]);
                self.property_data.text_opacity =
                    (u8::from_str_radix(&clean[6..8], 16).unwrap_or(255) as f32 / 255.0) * 100.0;
            }
            self.update_text_color_with_opacity(hex, self.property_data.text_opacity);
        }
    }

    pub fn handle_text_color_submit(&mut self) {
        if self.property_data.text_color_valid {
            if let Some(ref id) = self.selected_id {
                if let Some(obj) = self.document.get_object(id) {
                    self.property_data.text_color_input = hex_to_rgb6(&obj.text_color.to_hex());
                }
            }
        } else if let Some(ref id) = self.selected_id {
            if let Some(obj) = self.document.get_object(id) {
                self.property_data.text_color_input = hex_to_rgb6(&obj.text_color.to_hex());
                self.property_data.text_color_valid = true;
            }
        }
    }
}
