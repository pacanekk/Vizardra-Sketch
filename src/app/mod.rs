use std::path::PathBuf;

use iced::mouse;
use iced::widget::{column, row};
use iced::{Element, Length, Size, Subscription, Task, Theme};

use crate::core::document::Document;
use crate::core::object::{Color, ObjectKind, PathPoint};
use crate::core::project::ProjectFile;
use crate::renderer::WgpuRenderer;
use crate::ui::canvas::{create_object_at, format_zoom, CanvasEvent, CanvasState, CanvasWidget};
use crate::ui::layers::LayersMessage;
use crate::ui::properties::{PropertyData, PropertiesMessage};
use crate::ui::status_bar::StatusBarMessage;
use crate::ui::toolbar::ToolbarMessage;

pub struct AppState {
    pub document: Document,
    pub selected_id: Option<String>,
    pub canvas: CanvasState,
    pub active_tool: String,
    pub status_text: String,
    pub zoom_text: String,
    pub current_file: Option<PathBuf>,
    pub undo_stack: Vec<Document>,
    pub redo_stack: Vec<Document>,
    #[allow(dead_code)]
    pub renderer: WgpuRenderer,
    pub property_data: PropertyData,
    pub doc_size: String,
    pub needs_fit: bool,
    pub editing_layer_id: Option<String>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            document: Document::default_1080p(),
            selected_id: None,
            canvas: CanvasState::default(),
            active_tool: "select".to_string(),
            status_text: "Ready".to_string(),
            zoom_text: "100%".to_string(),
            current_file: None,
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            renderer: WgpuRenderer::new(),
            property_data: PropertyData::default(),
            doc_size: "1920 × 1080".to_string(),
            needs_fit: true,
            editing_layer_id: None,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    Toolbar(ToolbarMessage),
    Canvas(CanvasEvent),
    Layers(LayersMessage),
    Properties(PropertiesMessage),
    StatusBar(StatusBarMessage),
    WindowResized(Size),
}

pub fn run() -> Result<(), iced::Error> {
    iced::application("Vizardra", AppState::update, AppState::view)
        .theme(|_state| Theme::Dark)
        .subscription(AppState::subscription)
        .window_size((1280.0, 800.0))
        .run()
}

fn measure_text(content: &str, font_size: f32) -> (f32, f32) {
    use iced::advanced::text::Paragraph as _;
    
    let text = iced::advanced::Text {
        content,
        bounds: iced::Size::new(f32::INFINITY, f32::INFINITY),
        size: iced::Pixels(font_size),
        line_height: iced::advanced::text::LineHeight::default(),
        font: iced::Font::default(),
        horizontal_alignment: iced::alignment::Horizontal::Left,
        vertical_alignment: iced::alignment::Vertical::Top,
        shaping: iced::advanced::text::Shaping::Basic,
        wrapping: iced::advanced::text::Wrapping::None,
    };
    
    let paragraph = <iced::Renderer as iced::advanced::text::Renderer>::Paragraph::with_text(text);
    let bounds = paragraph.min_bounds();
    (bounds.width.max(1.0), bounds.height.max(1.0))
}

fn is_valid_hex(s: &str) -> bool {
    let clean = s.trim_start_matches('#');
    clean.len() == 6 || clean.len() == 8
}

fn hex_to_rgb6(s: &str) -> String {
    let clean = s.trim_start_matches('#');
    if clean.len() >= 6 {
        format!("#{}", &clean[..6])
    } else {
        s.to_string()
    }
}

impl AppState {
    pub fn new() -> Self {
        Self::default()
    }

    fn push_undo(&mut self) {
        self.undo_stack.push(self.document.clone());
        if self.undo_stack.len() > 50 {
            self.undo_stack.remove(0);
        }
        self.redo_stack.clear();
    }

    fn undo(&mut self) {
        if let Some(prev) = self.undo_stack.pop() {
            self.redo_stack.push(self.document.clone());
            self.document = prev;
            self.selected_id = None;
            self.status_text = "Undo".to_string();
        }
    }

    fn redo(&mut self) {
        if let Some(next) = self.redo_stack.pop() {
            self.undo_stack.push(self.document.clone());
            self.document = next;
            self.selected_id = None;
            self.status_text = "Redo".to_string();
        }
    }

    fn can_undo(&self) -> bool {
        !self.undo_stack.is_empty()
    }

    fn can_redo(&self) -> bool {
        !self.redo_stack.is_empty()
    }

    fn new_project(&mut self) {
        self.push_undo();
        self.document = Document::default_1080p();
        self.selected_id = None;
        self.current_file = None;
        self.status_text = "New project".to_string();
    }

    fn save_project(&mut self) -> Task<Message> {
        let path = if let Some(ref p) = self.current_file {
            p.clone()
        } else {
            let file = rfd::FileDialog::new()
                .add_filter("Vizardra", &["vizardra"])
                .set_file_name("untitled.vizardra")
                .save_file();

            match file {
                Some(p) => p,
                None => return Task::none(),
            }
        };

        let project = ProjectFile::from_document(&self.document);
        match project.save_to_file(&path) {
            Ok(()) => {
                self.current_file = Some(path);
                self.status_text = "Project saved".to_string();
            }
            Err(e) => {
                self.status_text = format!("Save error: {}", e);
            }
        }
        Task::none()
    }

    fn open_project(&mut self) -> Task<Message> {
        let file = rfd::FileDialog::new()
            .add_filter("Vizardra", &["vizardra"])
            .pick_file();

        if let Some(path) = file {
            match ProjectFile::load_from_file(&path) {
                Ok(project) => {
                    self.push_undo();
                    self.document = project.document;
                    self.selected_id = None;
                    self.current_file = Some(path);
                    self.status_text = "Project opened".to_string();
                }
                Err(e) => {
                    self.status_text = format!("Open error: {}", e);
                }
            }
        }
        Task::none()
    }

    fn export(&mut self) {
        let file = rfd::FileDialog::new()
            .add_filter("PNG", &["png"])
            .set_file_name("export.png")
            .save_file();

        if let Some(_path) = file {
            self.status_text = "Export not yet implemented".to_string();
        }
    }

    fn set_tool(&mut self, tool: &str) {
        if tool != "pen" && self.canvas.is_drawing_path {
            self.finalize_path();
        }
        self.active_tool = tool.to_string();
        if tool != "select" {
            self.selected_id = None;
        }
    }

    fn select_object(&mut self, id: &str) {
        self.selected_id = Some(id.to_string());
    }

    fn rename_object(&mut self, id: &str, name: &str) {
        if let Some(obj) = self.document.get_object_mut(id) {
            obj.name = name.to_string();
        }
    }

    fn toggle_visibility(&mut self, id: &str) {
        if let Some(obj) = self.document.get_object_mut(id) {
            obj.visible = !obj.visible;
        }
    }

    fn update_property(&mut self, key: &str, value: f32) {
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

    fn update_fill_color(&mut self, hex: &str) {
        if let Some(ref id) = self.selected_id {
            if let Some(obj) = self.document.get_object_mut(id) {
                obj.fill_color = Color::from_hex(hex);
            }
        }
    }

    fn update_fill_color_with_opacity(&mut self, hex: &str, opacity_pct: f32) {
        if let Some(ref id) = self.selected_id {
            if let Some(obj) = self.document.get_object_mut(id) {
                let mut color = Color::from_hex(hex);
                color.a = ((opacity_pct / 100.0) * 255.0).round() as u8;
                obj.fill_color = color;
            }
        }
    }

    fn update_text_color(&mut self, hex: &str) {
        if let Some(ref id) = self.selected_id {
            if let Some(obj) = self.document.get_object_mut(id) {
                obj.text_color = Color::from_hex(hex);
            }
        }
    }

    fn update_text_color_with_opacity(&mut self, hex: &str, opacity_pct: f32) {
        if let Some(ref id) = self.selected_id {
            if let Some(obj) = self.document.get_object_mut(id) {
                let mut color = Color::from_hex(hex);
                color.a = ((opacity_pct / 100.0) * 255.0).round() as u8;
                obj.text_color = color;
            }
        }
    }

    fn update_text_content(&mut self, content: &str) {
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

    fn update_font_size(&mut self, size: f32) {
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

    fn update_object_name(&mut self, name: &str) {
        if let Some(ref id) = self.selected_id {
            if let Some(obj) = self.document.get_object_mut(id) {
                obj.name = name.to_string();
            }
        }
    }

    fn update_path_bbox(obj: &mut crate::core::object::ObjectData) {
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

    fn finalize_path(&mut self) {
        if self.canvas.is_drawing_path {
            self.canvas.is_drawing_path = false;
            if let Some(ref id) = self.selected_id {
                if let Some(obj) = self.document.get_object(id) {
                    if obj.points.len() < 2 {
                        self.document.remove_object(id);
                        self.selected_id = None;
                    }
                }
            }
            self.active_tool = "select".to_string();
        }
    }

    fn handle_canvas_press(&mut self, screen_x: f32, screen_y: f32) {
        let (doc_x, doc_y) = self.canvas.screen_to_doc(screen_x, screen_y);
        let tool = self.active_tool.clone();
        match tool.as_str() {
            "select" => {
                if let Some(id) = self.document.hit_test(doc_x, doc_y) {
                    self.selected_id = Some(id.clone());
                    if let Some(obj) = self.document.get_object(&id) {
                        self.canvas.is_dragging = true;
                        self.canvas.drag_start = (doc_x, doc_y);
                        self.canvas.drag_object_start = obj.transform.clone();
                        self.canvas.drag_points_start = obj.points.clone();
                    }
                } else {
                    self.selected_id = None;
                }
            }
            "rectangle" | "text" | "image" => {
                let kind = match tool.as_str() {
                    "rectangle" => ObjectKind::Rectangle,
                    "text" => ObjectKind::Text,
                    _ => ObjectKind::Image,
                };
                self.push_undo();
                let obj = create_object_at(&mut self.document, kind.clone(), doc_x, doc_y, 1.0, 1.0);
                self.selected_id = Some(obj.id);
                self.canvas.is_creating = true;
                self.canvas.create_start = (doc_x, doc_y);
                self.canvas.create_kind = Some(kind);
            }
            "pen" => {
                if !self.canvas.is_drawing_path {
                    self.push_undo();
                    let obj = create_object_at(&mut self.document, ObjectKind::Path, doc_x, doc_y, 1.0, 1.0);
                    self.selected_id = Some(obj.id);
                    self.canvas.is_drawing_path = true;
                }
                if let Some(ref id) = self.selected_id {
                    if let Some(obj) = self.document.get_object_mut(id) {
                        obj.points.push(PathPoint { x: doc_x, y: doc_y });
                        Self::update_path_bbox(obj);
                    }
                }
            }
            _ => {}
        }
    }

    fn handle_canvas_move(&mut self, screen_x: f32, screen_y: f32) {
        let (doc_x, doc_y) = self.canvas.screen_to_doc(screen_x, screen_y);
        if self.canvas.is_creating {
            if let Some(ref id) = self.selected_id {
                if let Some(obj) = self.document.get_object_mut(id) {
                    let sx = self.canvas.create_start.0;
                    let sy = self.canvas.create_start.1;
                    obj.transform.x = doc_x.min(sx);
                    obj.transform.y = doc_y.min(sy);
                    obj.transform.width = (doc_x - sx).abs().max(1.0);
                    obj.transform.height = (doc_y - sy).abs().max(1.0);
                }
            }
        } else if self.canvas.is_dragging {
            if let Some(ref id) = self.selected_id {
                let dx = doc_x - self.canvas.drag_start.0;
                let dy = doc_y - self.canvas.drag_start.1;
                if let Some(obj) = self.document.get_object_mut(id) {
                    obj.transform.x = self.canvas.drag_object_start.x + dx;
                    obj.transform.y = self.canvas.drag_object_start.y + dy;
                    if obj.kind == ObjectKind::Path {
                        let pts_start = &self.canvas.drag_points_start;
                        for (i, p) in obj.points.iter_mut().enumerate() {
                            if let Some(orig) = pts_start.get(i) {
                                p.x = orig.x + dx;
                                p.y = orig.y + dy;
                            }
                        }
                    }
                }
            }
        }
    }

    fn handle_canvas_release(&mut self, _screen_x: f32, _screen_y: f32) {
        if self.canvas.is_creating {
            if let Some(ref id) = self.selected_id {
                if let Some(obj) = self.document.get_object(id) {
                    if obj.transform.width < 5.0 || obj.transform.height < 5.0 {
                        if let Some(kind) = self.canvas.create_kind.clone() {
                            if kind == ObjectKind::Text {
                                if let Some(obj_mut) = self.document.get_object_mut(id) {
                                    let (w, h) = measure_text(&obj_mut.text_content, obj_mut.font_size);
                                    obj_mut.transform.width = w;
                                    obj_mut.transform.height = h;
                                }
                            } else {
                                let w = match kind {
                                    ObjectKind::Rectangle => 200.0,
                                    ObjectKind::Image => 240.0,
                                    ObjectKind::Path => 100.0,
                                    _ => 200.0,
                                };
                                let h = match kind {
                                    ObjectKind::Rectangle => 150.0,
                                    ObjectKind::Image => 180.0,
                                    ObjectKind::Path => 100.0,
                                    _ => 150.0,
                                };
                                if let Some(obj_mut) = self.document.get_object_mut(id) {
                                    obj_mut.transform.width = w;
                                    obj_mut.transform.height = h;
                                }
                            }
                        }
                    }
                }
            }
            self.canvas.is_creating = false;
            self.canvas.create_kind = None;
            self.active_tool = "select".to_string();
        }
        self.canvas.is_dragging = false;
    }

    fn handle_pan_start(&mut self, screen_x: f32, screen_y: f32) {
        self.canvas.is_panning = true;
        self.canvas.pan_start_mouse = (screen_x, screen_y);
        self.canvas.pan_start_pan = (self.canvas.pan_x, self.canvas.pan_y);
    }

    fn handle_pan_move(&mut self, screen_x: f32, screen_y: f32) {
        if self.canvas.is_panning {
            let dx = screen_x - self.canvas.pan_start_mouse.0;
            let dy = screen_y - self.canvas.pan_start_mouse.1;
            self.canvas.pan_x = self.canvas.pan_start_pan.0 + dx;
            self.canvas.pan_y = self.canvas.pan_start_pan.1 + dy;
        }
    }

    fn handle_pan_end(&mut self) {
        self.canvas.is_panning = false;
    }

    fn handle_scroll(&mut self, screen_x: f32, screen_y: f32, delta_y: f32) {
        let (doc_x, doc_y) = self.canvas.screen_to_doc(screen_x, screen_y);
        let zoom_delta = if delta_y > 0.0 { 0.1 } else { -0.1 };
        self.canvas.zoom_at(doc_x, doc_y, zoom_delta);
        self.zoom_text = format_zoom(self.canvas.zoom);
    }

    fn handle_pan_delta(&mut self, dx: f32, dy: f32) {
        self.canvas.pan_x += dx;
        self.canvas.pan_y += dy;
    }

    fn zoom_in(&mut self) {
        let cx = self.document.width as f32 / 2.0;
        let cy = self.document.height as f32 / 2.0;
        self.canvas.zoom_at(cx, cy, 0.1);
        self.zoom_text = format_zoom(self.canvas.zoom);
    }

    fn zoom_out(&mut self) {
        let cx = self.document.width as f32 / 2.0;
        let cy = self.document.height as f32 / 2.0;
        self.canvas.zoom_at(cx, cy, -0.1);
        self.zoom_text = format_zoom(self.canvas.zoom);
    }

    fn zoom_reset(&mut self) {
        self.canvas.zoom = 1.0;
        let doc_w = self.document.width as f32;
        let doc_h = self.document.height as f32;
        self.canvas.pan_x = -doc_w / 2.0;
        self.canvas.pan_y = -doc_h / 2.0;
        self.zoom_text = format_zoom(self.canvas.zoom);
    }

    fn fit_to_screen(&mut self, window_w: f32, window_h: f32) {
        let canvas_w = (window_w - 220.0 - 260.0).max(1.0);
        let canvas_h = (window_h - 44.0 - 28.0).max(1.0);
        let doc_w = self.document.width as f32;
        let doc_h = self.document.height as f32;
        let margin = 20.0;
        let avail_w = (canvas_w - margin * 2.0).max(1.0);
        let avail_h = (canvas_h - margin * 2.0).max(1.0);
        let zoom_x = avail_w / doc_w;
        let zoom_y = avail_h / doc_h;
        self.canvas.zoom = zoom_x.min(zoom_y);
        let rendered_w = doc_w * self.canvas.zoom;
        let rendered_h = doc_h * self.canvas.zoom;
        self.canvas.pan_x = (canvas_w - rendered_w) / 2.0;
        self.canvas.pan_y = (canvas_h - rendered_h) / 2.0;
        self.zoom_text = format_zoom(self.canvas.zoom);
    }

    fn get_property_data(&self) -> PropertyData {
        if let Some(ref id) = self.selected_id {
            if let Some(obj) = self.document.get_object(id) {
                return PropertyData::from_object(obj);
            }
        }
        PropertyData::default()
    }

    fn refresh_cache(&mut self) {
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

    pub fn subscription(&self) -> Subscription<Message> {
        iced::window::resize_events().map(|(_id, size)| Message::WindowResized(size))
    }

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
                    ToolbarMessage::Undo => self.undo(),
                    ToolbarMessage::Redo => self.redo(),
                    ToolbarMessage::NewProject => self.new_project(),
                    ToolbarMessage::OpenProject => {
                        let task = self.open_project();
                        return task;
                    }
                    ToolbarMessage::SaveProject => {
                        let task = self.save_project();
                        return task;
                    }
                    ToolbarMessage::Export => self.export(),
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
                    PropertiesMessage::FillColorInput(hex) => {
                        self.property_data.fill_color_input = hex.clone();
                        self.property_data.fill_color_valid = is_valid_hex(&hex);
                        if self.property_data.fill_color_valid {
                            let clean = hex.trim_start_matches('#');
                            if clean.len() == 8 {
                                self.property_data.fill_color_input = format!("#{}", &clean[..6]);
                                self.property_data.fill_opacity = (u8::from_str_radix(&clean[6..8], 16).unwrap_or(255) as f32 / 255.0) * 100.0;
                            }
                            self.update_fill_color_with_opacity(&hex, self.property_data.fill_opacity);
                        }
                    }
                    PropertiesMessage::FillColorSubmit => {
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
                    PropertiesMessage::FillOpacityChanged(opacity) => {
                        self.property_data.fill_opacity = opacity;
                        let hex = self.property_data.fill_color_input.clone();
                        if is_valid_hex(&hex) {
                            self.update_fill_color_with_opacity(&hex, opacity);
                        }
                    }
                    PropertiesMessage::TextContentChanged(content) => self.update_text_content(&content),
                    PropertiesMessage::FontSizeChanged(v) => {
                        if let Ok(val) = v.parse::<f32>() { self.update_font_size(val); }
                    }
                    PropertiesMessage::TextColorInput(hex) => {
                        self.property_data.text_color_input = hex.clone();
                        self.property_data.text_color_valid = is_valid_hex(&hex);
                        if self.property_data.text_color_valid {
                            let clean = hex.trim_start_matches('#');
                            if clean.len() == 8 {
                                self.property_data.text_color_input = format!("#{}", &clean[..6]);
                                self.property_data.text_opacity = (u8::from_str_radix(&clean[6..8], 16).unwrap_or(255) as f32 / 255.0) * 100.0;
                            }
                            self.update_text_color_with_opacity(&hex, self.property_data.text_opacity);
                        }
                    }
                    PropertiesMessage::TextColorSubmit => {
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
                    PropertiesMessage::TextOpacityChanged(opacity) => {
                        self.property_data.text_opacity = opacity;
                        let hex = self.property_data.text_color_input.clone();
                        if is_valid_hex(&hex) {
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
                if self.needs_fit {
                    self.fit_to_screen(size.width, size.height);
                    self.needs_fit = false;
                }
                Task::none()
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
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

        let status_bar = crate::ui::status_bar::view(&self.status_text, &self.doc_size, &self.zoom_text)
            .map(Message::StatusBar);

        let main = row![layers, canvas, properties]
            .width(Length::Fill)
            .height(Length::Fill);

        column![toolbar, main, status_bar]
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}
