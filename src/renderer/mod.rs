use crate::core::document::Document;

#[allow(dead_code)]
#[derive(Debug)]
pub enum RenderError {
    NotImplemented,
    InvalidDimensions,
}

impl std::fmt::Display for RenderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RenderError::NotImplemented => write!(f, "Rendering not yet implemented"),
            RenderError::InvalidDimensions => write!(f, "Invalid render dimensions"),
        }
    }
}

impl std::error::Error for RenderError {}

#[allow(dead_code)]
pub trait DocumentRenderer {
    fn render_to_image(
        &self,
        document: &Document,
        width: u32,
        height: u32,
    ) -> Result<Vec<u8>, RenderError>;
}

pub struct WgpuRenderer {
    _initialized: bool,
}

impl WgpuRenderer {
    pub fn new() -> Self {
        Self { _initialized: false }
    }
}

impl Default for WgpuRenderer {
    fn default() -> Self {
        Self::new()
    }
}

impl DocumentRenderer for WgpuRenderer {
    fn render_to_image(
        &self,
        _document: &Document,
        _width: u32,
        _height: u32,
    ) -> Result<Vec<u8>, RenderError> {
        Err(RenderError::NotImplemented)
    }
}
