use crate::core::object::ObjectData;

#[allow(dead_code)]
pub struct LayerEntry {
    pub object: ObjectData,
}

impl LayerEntry {
    #[allow(dead_code)]
    pub fn new(object: ObjectData) -> Self {
        Self { object }
    }
}
