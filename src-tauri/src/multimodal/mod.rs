pub mod pdf_processor;
pub mod video_processor;
pub mod audio_processor;
pub mod image_ocr;
pub mod file_preview;

pub use pdf_processor::PdfProcessor;
pub use video_processor::VideoProcessor;
pub use audio_processor::AudioProcessor;
pub use image_ocr::ImageOCR;
pub use file_preview::FilePreview;

use serde::{Deserialize, Serialize};

/// 文件处理结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileProcessResult {
    pub file_path: String,
    pub file_type: String,
    pub extracted_text: String,
    pub metadata: FileMetadata,
    pub processed_at: i64,
}

/// 文件元数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileMetadata {
    pub size: u64,
    pub created_at: i64,
    pub modified_at: i64,
    pub mime_type: Option<String>,
    pub additional_info: serde_json::Value,
}
