use thiserror::Error;

#[derive(Error, Debug)]
pub enum ProcessError {
    #[error("Failed to load image from {path}: {source}")]
    ImageLoad {
        path: String,
        #[source]
        source: Box<dyn std::error::Error + Send + Sync>,
    },

    #[error("Failed to write file to {path}: {source}")]
    FileWrite {
        path: String,
        #[source]
        source: Box<dyn std::error::Error + Send + Sync>,
    },

    #[error("{operation} operation out of bounds: {details}")]
    OutOfBounds { operation: String, details: String },

    #[error("JPEG encoding failed: {0}")]
    JpegEncoding(#[source] Box<dyn std::error::Error + Send + Sync>),

    #[error(
        "Target size {target_kb} KB ({target_bytes} bytes) is too small. Minimum achievable size is {min_kb} KB ({min_bytes} bytes) at quality 10"
    )]
    CompressionTargetTooSmall {
        target_kb: usize,
        target_bytes: usize,
        min_kb: usize,
        min_bytes: usize,
    },
}
