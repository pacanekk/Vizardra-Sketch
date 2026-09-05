use std::path::PathBuf;

use crate::core::document::Document;
use crate::renderer::WgpuRenderer;
use crate::ui::canvas::CanvasState;
use crate::ui::properties::PropertyData;

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
    pub window_size: (f32, f32),
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
            window_size: (1280.0, 800.0),
        }
    }
}
