use serde::{Deserialize, Serialize};

use crate::core::object::{ObjectData, ObjectKind};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Document {
    pub width: u32,
    pub height: u32,
    pub objects: Vec<ObjectData>,
}

#[allow(dead_code)]
impl Document {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            objects: Vec::new(),
        }
    }

    pub fn default_1080p() -> Self {
        Self::new(1920, 1080)
    }

    pub fn add_object(&mut self, object: ObjectData) {
        self.objects.push(object);
    }

    pub fn remove_object(&mut self, id: &str) {
        self.objects.retain(|o| o.id != id);
    }

    pub fn get_object(&self, id: &str) -> Option<&ObjectData> {
        self.objects.iter().find(|o| o.id == id)
    }

    pub fn get_object_mut(&mut self, id: &str) -> Option<&mut ObjectData> {
        self.objects.iter_mut().find(|o| o.id == id)
    }

    pub fn move_object_up(&mut self, id: &str) {
        if let Some(idx) = self.objects.iter().position(|o| o.id == id) {
            if idx + 1 < self.objects.len() {
                self.objects.swap(idx, idx + 1);
            }
        }
    }

    pub fn move_object_down(&mut self, id: &str) {
        if let Some(idx) = self.objects.iter().position(|o| o.id == id) {
            if idx > 0 {
                self.objects.swap(idx, idx - 1);
            }
        }
    }

    pub fn hit_test(&self, x: f32, y: f32) -> Option<String> {
        for obj in self.objects.iter().rev() {
            if obj.visible && obj.contains_point(x, y) {
                return Some(obj.id.clone());
            }
        }
        None
    }

    pub fn next_id(&self) -> String {
        let count = self.objects.len();
        format!("obj_{}", count)
    }

    pub fn object_count_by_kind(&self, kind: &ObjectKind) -> usize {
        self.objects
            .iter()
            .filter(|o| &o.kind == kind)
            .count()
    }
}

impl Default for Document {
    fn default() -> Self {
        Self::default_1080p()
    }
}
