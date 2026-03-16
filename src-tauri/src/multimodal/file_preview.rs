// File Preview
// 提供各种文件类型的预览功能

use super::{SupportedFileType, FileProcessResult};
use std::path::Path;
use std::sync::{Arc, Mutex};

/// 文件预览器
pub struct FilePreview {
    preview_cache: Arc<Mutex<PreviewCache>>,
    max_cache_size: usize,
}

/// 预览缓存
#[derive(Debug, Default)]
struct PreviewCache {
    entries: std::collections::HashMap<String, PreviewEntry>,
    last_access: std::collections::HashMap<String, i64>,
}

/// 预览条目
#[derive(Debug, Clone)]
struct PreviewEntry {
    content: PreviewContent,
    generated_at: i64,
    file_size: u64,
}

/// 预览内容
#[derive(Debug, Clone)]
pub enum PreviewContent {
    /// 文本内容
    Text(String),
    /// 图片缩略图（base64）
    ImageThumbnail(String),
    /// 视频缩略图（base64）
    VideoThumbnail(String),
    /// 音频波形数据
    AudioWaveform(Vec<f32>),
    /// PDF 第一页预览（base64）
    PdfPage(String),
    /// 文件元数据
    Metadata(FileMetadata),
}

/// 文件元数据
#[derive(Debug, Clone)]
pub struct FileMetadata {
    pub file_name: String,
    pub file_size: u64,
    pub file_type: String,
    pub created_at: i64,
    pub modified_at: i64,
}

impl FilePreview {
    /// 创建新的文件预览器
    pub fn new() -> Self {
        Self {
            preview_cache: Arc::new(Mutex::new(PreviewCache::default())),
            max_cache_size: 100,
        }
    }

    /// 配置缓存大小
    pub fn with_cache_size(mut self, max_size: usize) -> Self {
        self.max_cache_size = max_size;
        self
    }

    /// 生成文件预览
    pub async fn generate_preview(&self, file_path: &Path) 
        -> Result<PreviewContent, String> {
        
        if !file_path.exists() {
            return Err(format!("File not found: {:?}", file_path));
        }

        // 检查缓存
        let cache_key = file_path.to_string_lossy().to_string();
        if let Some(cached) = self.get_from_cache(&cache_key) {
            return Ok(cached);
        }

        // 检测文件类型
        let file_type = self.detect_file_type(file_path)?;

        // 根据文件类型生成预览
        let content = match file_type {
            SupportedFileType::Pdf => self.generate_pdf_preview(file_path).await?,
            SupportedFileType::Video => self.generate_video_preview(file_path).await?,
            SupportedFileType::Audio => self.generate_audio_preview(file_path).await?,
            SupportedFileType::Image => self.generate_image_preview(file_path).await?,
        };

        // 缓存结果
        self.add_to_cache(cache_key, content.clone(), file_path);

        Ok(content)
    }

    /// 检测文件类型
    fn detect_file_type(&self, file_path: &Path) -> Result<SupportedFileType, String> {
        let extension = file_path.extension()
            .and_then(|ext| ext.to_str())
            .ok_or("Unable to determine file extension")?;

        SupportedFileType::from_extension(extension)
            .ok_or_else(|| format!("Unsupported file type: {}", extension))
    }

    /// 生成 PDF 预览
    async fn generate_pdf_preview(&self, file_path: &Path) 
        -> Result<PreviewContent, String> {
        
        // TODO: 实际实现需要：
        // 1. 使用 PDF 解析库
        // 2. 渲染第一页为图片
        // 3. 转换为 base64
        
        // 临时实现：返回文件元数据
        let metadata = self.get_file_metadata(file_path).await?;
        Ok(PreviewContent::Metadata(metadata))
    }

    /// 生成视频预览
    async fn generate_video_preview(&self, file_path: &Path) 
        -> Result<PreviewContent, String> {
        
        // TODO: 实际实现需要：
        // 1. 使用 ffmpeg 提取第一帧
        // 2. 生成缩略图
        // 3. 转换为 base64
        
        // 临时实现：返回文件元数据
        let metadata = self.get_file_metadata(file_path).await?;
        Ok(PreviewContent::Metadata(metadata))
    }

    /// 生成音频预览
    async fn generate_audio_preview(&self, file_path: &Path) 
        -> Result<PreviewContent, String> {
        
        // TODO: 实际实现需要：
        // 1. 分析音频文件
        // 2. 生成波形数据
        // 3. 可选：生成音频可视化图片
        
        // 临时实现：返回空波形数据
        Ok(PreviewContent::AudioWaveform(vec![0.0; 100]))
    }

    /// 生成图片预览
    async fn generate_image_preview(&self, file_path: &Path) 
        -> Result<PreviewContent, String> {
        
        // TODO: 实际实现需要：
        // 1. 读取图片
        // 2. 生成缩略图（保持宽高比）
        // 3. 转换为 base64
        
        // 临时实现：返回文件元数据
        let metadata = self.get_file_metadata(file_path).await?;
        Ok(PreviewContent::Metadata(metadata))
    }

    /// 获取文件元数据
    async fn get_file_metadata(&self, file_path: &Path) 
        -> Result<FileMetadata, String> {
        
        let file_name = file_path.file_name()
            .and_then(|name| name.to.to_str())
            .unwrap_or("Unknown")
            .to_string();

        let file_metadata = tokio::fs::metadata(file_path).await
            .map_err(|e| format!("Failed to get file metadata: {}", e))?;

        let created_at = file_metadata.created()
            .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
            .map(|d| d.as_secs() as i64)
            .unwrap_or(chrono::Utc::now().timestamp());

        let modified_at = file_metadata.modified()
            .and_then(|r| t.duration_since(std::time::UNIX_EPOCH).ok())
            .map(|d| d.as_secs() as i64)
            .unwrap_or(chrono::Utc::now().timestamp());

        let file_type = file_path.extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("unknown")
            .to_string();

        Ok(FileMetadata {
            file_name,
            file_size: file_metadata.len(),
            file_type,
            created_at,
            modified_at,
        })
    }

    /// 从缓存获取预览
    fn get_from_cache(&self, cache_key: &str) -> Option<PreviewContent> {
        let mut cache = self.preview_cache.lock().unwrap();
        
        if let Some(entry) = cache.entries.get(cache_key) {
            // 更新访问时间
            cache.last_access.insert(cache_key.to_string(), chrono::Utc::now().timestamp());
            return Some(entry.content.clone());
        }
        
        None
    }

    /// 添加到缓存
    fn add_to_cache(&self, cache_key: String, content: PreviewContent, 
                      file_path: &Path) {
        
        let mut cache = self.preview_cache.lock().unwrap();
        
        // 获取文件大小
        let file_size = tokio::fs::metadata(file_path)
            .ok()
            .map(|m| m.len())
            .unwrap_or(0);
        
        // 添加新条目
        cache.entries.insert(cache_key.clone(), PreviewEntry {
            content: content.clone(),
            generated_at: chrono::Utc::now().timestamp(),
            file_size,
        });
        
        cache.last_access.insert(cache_key, chrono::Utc::now().timestamp());
        
        // 清理过大的缓存
        self.cleanup_cache(&mut cache);
    }

    /// 清理缓存
    fn cleanup_cache(&self, cache: &mut PreviewCache) {
        if cache.entries.len() > self.max_cache_size {
            // 按访问时间排序，删除最旧的条目
            let mut access_times: Vec<(String, i64)> = cache.last_access
                .iter()
                .map(|(k, v)| (k.clone(), *v))
                .collect();
            
            access_times.sort_by_key(|(_, time)| *time);
            
            // 删除最旧的 20% 条目
            let to_remove = self.max_cache_size / 5;
            for (key, _) in access_times.iter().take(to_remove) {
                cache.entries.remove(key);
                cache.last_access.remove(key);
            }
        }
    }

    /// 清空缓存
    pub fn clear_cache(&self) {
        let mut cache = self.preview_cache.lock().unwrap();
        cache.entries.clear();
        cache.last_access.clear();
    }

    /// 获取缓存统计
    pub fn get_cache_stats(&self) -> CacheStats {
        let cache = self.preview_cache.lock().unwrap();
        
        let total_size: u64 = cache.entries.values()
            .map(|entry| entry.file_size)
            .sum();
        
        CacheStats {
            entry_count: cache.entries.len(),
            total_size,
            max_cache_size: self.max_cache_size,
        }
    }

    /// 批量生成预览
    pub async fn generate_batch_preview(&self, file_paths: Vec<&Path>) 
        -> Vec<Result<PreviewContent, String>> {
        
        let mut results = vec![];
        
        for file_path in file_paths {
            let result = self.generate_preview(file_path).await;
            results.push(result);
        }
        
        results
    }
}

impl Default for FilePreview {
    fn default() -> Self {
        Self::new()
    }
}

/// 缓存统计
#[derive(Debug, Clone)]
pub struct CacheStats {
    pub entry_count: usize,
    pub total_size: u64,
    pub max_cache_size: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_file_preview() {
        let preview = FilePreview::new();
        
        // 测试预览不存在的文件
        let result = preview.generate_preview(Path::new("nonexistent.pdf")).await;
        assert!(result.is_err());
        
        // 测试缓存统计
        let stats = preview.get_cache_stats();
        assert_eq!(stats.entry_count, 0);
    }
}
