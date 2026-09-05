use super::state::AppState;
use crate::app::utils::{measure_text, smoothing_to_tolerance, smoothing_to_sample_distance, simplify_path};
use crate::core::object::{ObjectKind, PathPoint};
use crate::ui::canvas::{create_object_at, format_zoom};

impl AppState {
    pub fn finalize_path(&mut self) {
        if !self.canvas.is_drawing_path {
            return;
        }
        self.canvas.is_drawing_path = false;

        if self.active_tool == "draw" {
            let raw_points = std::mem::take(&mut self.canvas.temp_draw_points);

            if raw_points.len() < 2 {
                return;
            }

            let tolerance = smoothing_to_tolerance(self.canvas.smoothing);
            let mut points = raw_points;
            simplify_path(&mut points, tolerance);

            if points.len() < 2 {
                return;
            }

            let mut closed = false;
            if points.len() > 2 {
                let first = &points[0];
                let last = points.last().unwrap();
                let dx = last.x - first.x;
                let dy = last.y - first.y;
                let close_threshold = 15.0 / self.canvas.zoom;
                if (dx * dx + dy * dy).sqrt() < close_threshold {
                    closed = true;
                    points.pop();
                }
            }

            self.push_undo();

            let obj = create_object_at(&mut self.document, ObjectKind::Path, points[0].x, points[0].y, 1.0, 1.0);
            let obj_id = obj.id.clone();
            self.selected_id = Some(obj_id.clone());

            if let Some(obj) = self.document.get_object_mut(&obj_id) {
                obj.points = points;
                obj.closed = closed;
                Self::update_path_bbox(obj);
            }
        } else {
            // Pen tool: points already in document object
            if let Some(ref id) = self.selected_id {
                if let Some(obj) = self.document.get_object_mut(id) {
                    if obj.points.len() < 2 {
                        self.document.remove_object(id);
                        self.selected_id = None;
                    } else {
                        Self::update_path_bbox(obj);
                    }
                }
            }
        }

        self.active_tool = "select".to_string();
    }

    pub fn handle_canvas_press(&mut self, screen_x: f32, screen_y: f32) {
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
            "node" => {
                if let Some(ref id) = self.selected_id {
                    if let Some(obj) = self.document.get_object(id) {
                        if obj.kind == ObjectKind::Path {
                            let hit_radius = 8.0 / self.canvas.zoom;
                            for (i, p) in obj.points.iter().enumerate() {
                                let dx = p.x - doc_x;
                                let dy = p.y - doc_y;
                                if (dx * dx + dy * dy).sqrt() < hit_radius {
                                    self.canvas.dragging_node = Some(i);
                                    self.canvas.drag_points_start = obj.points.clone();
                                    self.push_undo();
                                    return;
                                }
                            }
                        }
                    }
                }
                if let Some(id) = self.document.hit_test(doc_x, doc_y) {
                    self.selected_id = Some(id);
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
            "draw" => {
                if !self.canvas.is_drawing_path {
                    self.canvas.is_drawing_path = true;
                    self.canvas.temp_draw_points.clear();
                }
                self.canvas.temp_draw_points.push(PathPoint { x: doc_x, y: doc_y });
            }
            "pen" => {
                if !self.canvas.is_drawing_path {
                    self.push_undo();
                    let obj = create_object_at(&mut self.document, ObjectKind::Path, doc_x, doc_y, 1.0, 1.0);
                    let obj_id = obj.id.clone();
                    self.selected_id = Some(obj_id.clone());
                    self.canvas.is_drawing_path = true;
                    if let Some(obj) = self.document.get_object_mut(&obj_id) {
                        obj.points.push(PathPoint { x: doc_x, y: doc_y });
                        Self::update_path_bbox(obj);
                    }
                } else if let Some(ref id) = self.selected_id {
                    if let Some(obj) = self.document.get_object_mut(id) {
                        obj.points.push(PathPoint { x: doc_x, y: doc_y });
                        Self::update_path_bbox(obj);
                    }
                }
            }
            _ => {}
        }
    }

    pub fn handle_canvas_move(&mut self, screen_x: f32, screen_y: f32) {
        let (doc_x, doc_y) = self.canvas.screen_to_doc(screen_x, screen_y);

        if let Some(node_idx) = self.canvas.dragging_node {
            if let Some(ref id) = self.selected_id {
                if let Some(obj) = self.document.get_object_mut(id) {
                    if let Some(p) = obj.points.get_mut(node_idx) {
                        p.x = doc_x;
                        p.y = doc_y;
                        Self::update_path_bbox(obj);
                    }
                }
            }
            return;
        }

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
        } else if self.canvas.is_drawing_path && self.active_tool == "draw" {
            let last = self.canvas.temp_draw_points.last();
            let min_dist = smoothing_to_sample_distance(self.canvas.smoothing);
            if last.is_none_or(|p| (p.x - doc_x).abs() > min_dist || (p.y - doc_y).abs() > min_dist) {
                self.canvas.temp_draw_points.push(PathPoint { x: doc_x, y: doc_y });
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

    pub fn handle_canvas_release(&mut self, _screen_x: f32, _screen_y: f32) {
        self.canvas.dragging_node = None;
        if self.canvas.is_drawing_path && self.active_tool == "draw" {
            self.finalize_path();
        } else if self.canvas.is_creating {
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

    pub fn handle_pan_start(&mut self, screen_x: f32, screen_y: f32) {
        self.canvas.is_panning = true;
        self.canvas.pan_start_mouse = (screen_x, screen_y);
        self.canvas.pan_start_pan = (self.canvas.pan_x, self.canvas.pan_y);
    }

    pub fn handle_pan_move(&mut self, screen_x: f32, screen_y: f32) {
        if self.canvas.is_panning {
            let dx = screen_x - self.canvas.pan_start_mouse.0;
            let dy = screen_y - self.canvas.pan_start_mouse.1;
            self.canvas.pan_x = self.canvas.pan_start_pan.0 + dx;
            self.canvas.pan_y = self.canvas.pan_start_pan.1 + dy;
        }
    }

    pub fn handle_pan_end(&mut self) {
        self.canvas.is_panning = false;
    }

    pub fn handle_scroll(&mut self, screen_x: f32, screen_y: f32, delta_y: f32) {
        let (doc_x, doc_y) = self.canvas.screen_to_doc(screen_x, screen_y);
        let zoom_delta = if delta_y > 0.0 { 0.1 } else { -0.1 };
        self.canvas.zoom_at(doc_x, doc_y, zoom_delta);
        self.zoom_text = format_zoom(self.canvas.zoom);
    }

    pub fn handle_pan_delta(&mut self, dx: f32, dy: f32) {
        self.canvas.pan_x += dx;
        self.canvas.pan_y += dy;
    }

    pub fn zoom_in(&mut self) {
        let cx = self.document.width as f32 / 2.0;
        let cy = self.document.height as f32 / 2.0;
        self.canvas.zoom_at(cx, cy, 0.1);
        self.zoom_text = format_zoom(self.canvas.zoom);
    }

    pub fn zoom_out(&mut self) {
        let cx = self.document.width as f32 / 2.0;
        let cy = self.document.height as f32 / 2.0;
        self.canvas.zoom_at(cx, cy, -0.1);
        self.zoom_text = format_zoom(self.canvas.zoom);
    }

    pub fn zoom_reset(&mut self) {
        self.fit_to_screen(self.window_size.0, self.window_size.1);
    }

    pub fn fit_to_screen(&mut self, window_w: f32, window_h: f32) {
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
}
