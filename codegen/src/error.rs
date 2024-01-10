use std::path::Path;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("deserialize error: {0}")]
    Deserialize(#[from] serde_json::Error),

    #[error("render error: {0}")]
    Render(#[from] handlebars::RenderError),

    #[error("path conversion error: {0}")]
    PathConversion(&'static Path),
}
