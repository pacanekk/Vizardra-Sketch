use iced::Task;

use super::message::Message;
use super::state::AppState;
use crate::core::document::Document;
use crate::core::project::ProjectFile;

impl AppState {
    pub fn new_project(&mut self) {
        self.push_undo();
        self.document = Document::default_1080p();
        self.selected_id = None;
        self.current_file = None;
        self.status_text = "New project".to_string();
    }

    pub fn save_project(&mut self) -> Task<Message> {
        let path = if let Some(ref p) = self.current_file {
            p.clone()
        } else {
            let file = rfd::FileDialog::new()
                .add_filter("Vizardra", &["vzd"])
                .set_file_name("untitled.vzd")
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

    pub fn open_project(&mut self) -> Task<Message> {
        let file = rfd::FileDialog::new()
            .add_filter("Vizardra", &["vzd"])
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

    pub fn export_png(&mut self) {
        let file = rfd::FileDialog::new()
            .add_filter("PNG", &["png"])
            .set_file_name("export.png")
            .save_file();

        if let Some(_path) = file {
            self.status_text = "PNG export not yet implemented".to_string();
        }
    }

    pub fn export_svg(&mut self) {
        let file = rfd::FileDialog::new()
            .add_filter("SVG", &["svg"])
            .set_file_name("export.svg")
            .save_file();

        if let Some(path) = file {
            let svg = crate::core::svg_export::document_to_svg(&self.document);
            match std::fs::write(&path, svg) {
                Ok(()) => {
                    self.status_text = "SVG exported".to_string();
                }
                Err(e) => {
                    self.status_text = format!("Export error: {}", e);
                }
            }
        }
    }
}
