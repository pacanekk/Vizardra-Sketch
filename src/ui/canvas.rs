use iced::mouse;
use iced::widget::canvas::{self, Canvas, Frame, Geometry, Path, Stroke, Text};
use iced::widget::Action;
use iced::{Color, Element, Point, Rectangle, Size, Vector};

use crate::core::document::Document;
use crate::core::object::{ObjectData, ObjectKind};
use crate::ui::theme::Theme;

#[derive(Clone, Debug)]
pub enum CanvasEvent {
    Pressed { pos: Point, button: mouse::Button },
    Moved(Point),
    Released { pos: Point, button: mouse::Button },
    Scrolled { delta: Vector, modifiers: Modifiers, cursor_pos: Point },
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Modifiers {
    pub alt: bool,
    pub control: bool,
    pub shift: bool,
}

#[derive(Clone, Debug)]
pub struct CanvasState {
    pub zoom: f32,
    pub pan_x: f32,
    pub pan_y: f32,
    pub is_creating: bool,
    pub create_start: (f32, f32),
    pub create_kind: Option<ObjectKind>,
    pub is_dragging: bool,
    pub drag_start: (f32, f32),
    pub drag_object_start: crate::core::object::Transform,
    pub is_panning: bool,
    pub pan_start_mouse: (f32, f32),
    pub pan_start_pan: (f32, f32),
    pub is_drawing_path: bool,
    pub drag_points_start: Vec<crate::core::object::PathPoint>,
}

impl Default for CanvasState {
    fn default() -> Self {
        Self {
            zoom: 1.0,
            pan_x: 100.0,
            pan_y: 100.0,
            is_creating: false,
            create_start: (0.0, 0.0),
            create_kind: None,
            is_dragging: false,
            drag_start: (0.0, 0.0),
            drag_object_start: crate::core::object::Transform::default(),
            is_panning: false,
            pan_start_mouse: (0.0, 0.0),
            pan_start_pan: (0.0, 0.0),
            is_drawing_path: false,
            drag_points_start: Vec::new(),
        }
    }
}

impl CanvasState {
    pub fn zoom_at(&mut self, doc_x: f32, doc_y: f32, delta: f32) {
        let old_zoom = self.zoom;
        let new_zoom = (self.zoom * (1.0 + delta)).clamp(0.05, 8.0);
        if (new_zoom - old_zoom).abs() < 0.001 {
            return;
        }
        let screen_x = doc_x * old_zoom + self.pan_x;
        let screen_y = doc_y * old_zoom + self.pan_y;
        self.zoom = new_zoom;
        self.pan_x = screen_x - doc_x * new_zoom;
        self.pan_y = screen_y - doc_y * new_zoom;
    }

    pub fn screen_to_doc(&self, sx: f32, sy: f32) -> (f32, f32) {
        ((sx - self.pan_x) / self.zoom, (sy - self.pan_y) / self.zoom)
    }
}

pub fn create_object_at(
    doc: &mut Document,
    kind: ObjectKind,
    x: f32,
    y: f32,
    w: f32,
    h: f32,
) -> ObjectData {
    let id = format!("obj_{}", uuid_counter());
    let mut obj = ObjectData::new(id, kind);
    obj.transform.x = x;
    obj.transform.y = y;
    obj.transform.width = w.max(1.0);
    obj.transform.height = h.max(1.0);
    doc.add_object(obj.clone());
    obj
}

fn uuid_counter() -> usize {
    use std::sync::atomic::{AtomicUsize, Ordering};
    static COUNTER: AtomicUsize = AtomicUsize::new(1);
    COUNTER.fetch_add(1, Ordering::SeqCst)
}

pub fn format_zoom(zoom: f32) -> String {
    format!("{:.0}%", zoom * 100.0)
}

pub struct CanvasWidget<'a> {
    pub document: &'a Document,
    pub canvas_state: &'a CanvasState,
    pub selected_id: &'a Option<String>,
    pub active_tool: &'a str,
}

impl<'a> CanvasWidget<'a> {
    pub fn view(&self) -> Element<'a, CanvasEvent> {
        Canvas::new(CanvasProgram {
            document: self.document,
            canvas_state: self.canvas_state,
            selected_id: self.selected_id,
            active_tool: self.active_tool,
        })
        .width(iced::Fill)
        .height(iced::Fill)
        .into()
    }
}

struct CanvasProgram<'a> {
    document: &'a Document,
    canvas_state: &'a CanvasState,
    selected_id: &'a Option<String>,
    active_tool: &'a str,
}

impl<'a> canvas::Program<CanvasEvent> for CanvasProgram<'a> {
    type State = InteractionState;

    fn update(
        &self,
        state: &mut InteractionState,
        event: &canvas::Event,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> Option<Action<CanvasEvent>> {
        let cursor_pos = cursor.position_in(bounds);
        let modifiers = state.last_modifiers;

        match event {
            canvas::Event::Mouse(mouse::Event::ButtonPressed(button)) => {
                if let Some(pos) = cursor_pos {
                    return Some(
                        Action::publish(CanvasEvent::Pressed { pos, button: *button })
                            .and_capture(),
                    );
                }
            }
            canvas::Event::Mouse(mouse::Event::ButtonReleased(button)) => {
                if let Some(pos) = cursor_pos {
                    return Some(
                        Action::publish(CanvasEvent::Released { pos, button: *button })
                            .and_capture(),
                    );
                }
            }
            canvas::Event::Mouse(mouse::Event::CursorMoved { .. }) => {
                if let Some(pos) = cursor_pos {
                    return Some(Action::publish(CanvasEvent::Moved(pos)));
                }
            }
            canvas::Event::Mouse(mouse::Event::WheelScrolled { delta }) => {
                if let Some(pos) = cursor_pos {
                    let scroll_delta = match delta {
                        mouse::ScrollDelta::Pixels { x, y } => Vector::new(*x, *y),
                        mouse::ScrollDelta::Lines { x, y } => {
                            Vector::new(*x * 20.0, *y * 20.0)
                        }
                    };
                    return Some(
                        Action::publish(CanvasEvent::Scrolled {
                            delta: scroll_delta,
                            modifiers: Modifiers {
                                alt: modifiers.alt,
                                control: modifiers.control,
                                shift: modifiers.shift,
                            },
                            cursor_pos: pos,
                        })
                        .and_capture(),
                    );
                }
            }
            canvas::Event::Keyboard(iced::keyboard::Event::ModifiersChanged(mods)) => {
                state.last_modifiers = Modifiers {
                    alt: mods.alt(),
                    control: mods.control(),
                    shift: mods.shift(),
                };
            }
            _ => {}
        }

        None
    }

    fn draw(
        &self,
        _state: &InteractionState,
        renderer: &iced::Renderer,
        _theme: &iced::Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<Geometry> {
        let mut frame = Frame::new(renderer, bounds.size());

        // Background
        frame.fill_rectangle(Point::ORIGIN, bounds.size(), Theme::bg_canvas());

        let cs = self.canvas_state;
        let doc = self.document;

        let doc_w = doc.width as f32 * cs.zoom;
        let doc_h = doc.height as f32 * cs.zoom;
        let doc_x = cs.pan_x;
        let doc_y = cs.pan_y;

        // Shadow
        frame.fill_rectangle(
            Point::new(doc_x + 3.0, doc_y + 3.0),
            Size::new(doc_w, doc_h),
            Theme::doc_shadow(),
        );

        // Document background
        frame.fill_rectangle(
            Point::new(doc_x, doc_y),
            Size::new(doc_w, doc_h),
            Theme::doc_bg(),
        );

        // Document border
        frame.stroke(
            &Path::rectangle(Point::new(doc_x, doc_y), Size::new(doc_w, doc_h)),
            Stroke::default().with_width(1.0).with_color(Theme::doc_border()),
        );

        // Draw objects (clipped to document area)
        let doc_bounds = Rectangle::new(Point::new(doc_x, doc_y), Size::new(doc_w, doc_h));
        let cs_zoom = cs.zoom;
        let doc_x_clip = doc_x;
        let doc_y_clip = doc_y;
        frame.with_clip(doc_bounds, |frame| {
            for obj in &doc.objects {
                if !obj.visible {
                    continue;
                }
                draw_object(frame, obj, cs_zoom, doc_x_clip, doc_y_clip);
            }
        });

        // Selection highlight
        if let Some(sel_id) = self.selected_id {
            if let Some(obj) = doc.get_object(sel_id) {
                if obj.visible {
                    let sx = obj.transform.x * cs.zoom + doc_x;
                    let sy = obj.transform.y * cs.zoom + doc_y;
                    let sw = obj.transform.width * cs.zoom;
                    let sh = obj.transform.height * cs.zoom;
                    frame.stroke(
                        &Path::rectangle(Point::new(sx - 1.0, sy - 1.0), Size::new(sw + 2.0, sh + 2.0)),
                        Stroke::default().with_width(1.5).with_color(Theme::selection()),
                    );
                }
            }
        }

        vec![frame.into_geometry()]
    }

    fn mouse_interaction(
        &self,
        _state: &InteractionState,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> mouse::Interaction {
        if self.canvas_state.is_panning {
            return mouse::Interaction::Grabbing;
        }
        match self.active_tool {
            "select" => {
                if cursor.position_in(bounds).is_some() {
                    mouse::Interaction::Pointer
                } else {
                    mouse::Interaction::default()
                }
            }
            _ => mouse::Interaction::Crosshair,
        }
    }
}

#[derive(Default)]
pub struct InteractionState {
    pub last_modifiers: Modifiers,
}

fn draw_object(frame: &mut Frame, obj: &ObjectData, zoom: f32, pan_x: f32, pan_y: f32) {
    let x = obj.transform.x * zoom + pan_x;
    let y = obj.transform.y * zoom + pan_y;
    let w = obj.transform.width * zoom;
    let h = obj.transform.height * zoom;

    match obj.kind {
        ObjectKind::Rectangle => {
            frame.fill_rectangle(
                Point::new(x, y),
                Size::new(w, h),
                color_from_core(&obj.fill_color),
            );
        }
        ObjectKind::Text => {
            let text = Text {
                content: obj.text_content.clone(),
                position: Point::new(x, y),
                color: color_from_core(&obj.text_color),
                size: iced::Pixels(obj.font_size * zoom),
                ..Default::default()
            };
            frame.fill_text(text);
        }
        ObjectKind::Image => {
            frame.fill_rectangle(
                Point::new(x, y),
                Size::new(w, h),
                Color::from_rgb(0.165, 0.165, 0.18),
            );
            frame.stroke(
                &Path::rectangle(Point::new(x, y), Size::new(w, h)),
                Stroke::default().with_width(1.0).with_color(Color::from_rgb(0.227, 0.227, 0.243)),
            );
            let text = Text {
                content: "Image".to_string(),
                position: Point::new(x + w / 2.0, y + h / 2.0),
                color: Color::from_rgb(0.353, 0.353, 0.376),
                size: iced::Pixels(12.0),
                align_x: iced::alignment::Horizontal::Center.into(),
                align_y: iced::alignment::Vertical::Center.into(),
                ..Default::default()
            };
            frame.fill_text(text);
        }
        ObjectKind::Path => {
            if obj.points.len() >= 2 {
                let path = build_path(&obj.points, zoom, pan_x, pan_y);
                frame.fill(&path, color_from_core(&obj.fill_color));
                frame.stroke(&path, Stroke::default().with_width(2.0 * zoom).with_color(Theme::accent()));
            }
        }
    }
}

fn build_path(points: &[crate::core::object::PathPoint], zoom: f32, pan_x: f32, pan_y: f32) -> Path {
    let mut builder = canvas::path::Builder::new();
    if let Some(first) = points.first() {
        builder.move_to(Point::new(first.x * zoom + pan_x, first.y * zoom + pan_y));
        for p in points.iter().skip(1) {
            builder.line_to(Point::new(p.x * zoom + pan_x, p.y * zoom + pan_y));
        }
        if points.len() > 2 {
            builder.close();
        }
    }
    builder.build()
}

fn color_from_core(c: &crate::core::object::Color) -> Color {
    Color::from_rgba8(c.r, c.g, c.b, c.a as f32 / 255.0)
}
