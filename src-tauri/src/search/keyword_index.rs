// 关键词索引
// 使用 SQLite 实现关键词搜索索引

use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use rusqlite::{Connection, params, Row};
use serde::{Deserialize, Serialize};

/// 索引对象
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexedDocument {
    pub id: String,
    pub content: String,
    pub file_path: String,
    pub content_length: usize,
    pub language: String,
    pub tags: Vec<String>,
    pub created_at: i64,
    pub updated_at: i64,
}

/// 关键词索引
pub struct KeywordIndex {
    db_path: PathBuf,
    conn: Arc<Mutex<Option<Connection>>>,
}

impl KeywordIndex {
    /// 创建新的关键词索引
    pub fn new(db_path: PathBuf) -> Result<Self, String> {
        // 创建数据库目录
        if let Some(parent) = db_path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| format!("Failed to create database directory: {}", e))?;
        }

        let conn = Arc::new(Mutex::new(None));
        let index = Self {
            db_path: db_path.clone(),
            conn,
        };

        // 初始化数据库
        index.initialize()?;

        Ok(index)
    }

    /// 初始化数据库
    fn initialize(&self) -> Result<(), String> {
        let mut conn_opt = self.conn.lock().unwrap();

        // 如果连接不存在，创建新连接
        if conn_opt.is_none() {
            let conn = Connection::open(&self.db_path)
                .map_err(|e| format!("Failed to open database: {}", e))?;
            *conn_opt = Some(conn);
        }

        let conn = conn_opt.as_mut().ok_or("Connection not initialized")?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS documents (
                id TEXT PRIMARY KEY,
                content TEXT NOT NULL,
                file_path TEXT NOT NULL,
                content_length INTEGER,
                language TEXT,
                tags TEXT,
                created_at INTEGER,
                updated_at INTEGER
            );",
            [],
        ).map_err(|e| format!("Failed to create documents table: {}", e))?;

        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_file_path ON documents(file_path);",
            [],
        ).map_err(|e| format!("Failed to create index: {}", e))?;

        Ok(())
    }

    /// 添加文档到索引
    pub fn add_document(&self, document: &IndexedDocument) -> Result<(), String> {
        let mut conn_opt = self.conn.lock().unwrap();
        let conn = conn_opt.as_mut().ok_or("Connection not initialized")?;

        let tags_json = serde_json::to_string(&document.tags)
            .map_err(|e| format!("Failed to serialize tags: {}", e))?;

        conn.execute(
        "INSERT INTO documents (id, content, file_path, content_length, language, tags, created_at, updated_at)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8);",
            params![
                &document.id,
                &document.content,
                &document.file_path,
                document.content_length as i64,
                &document.language,
                &tags_json,
                document.created_at,
                document.updated_at,
            ],
        ).map_err(|e| format!("Failed to insert document: {}", e))?;

        Ok(())
    }

    /// 批量添加文档
    pub fn add_documents(&self, documents: &[IndexedDocument]) -> Result<(), String> {
        let mut conn_opt = self.conn.lock().unwrap();
        let conn = conn_opt.as_mut().ok_or("Connection not initialized")?;

        let tx = conn.unchecked_transaction()
            .map_err(|e| format!("Failed to begin transaction: {}", e))?;

        for doc in documents {
            let tags_json = serde_json::to_string(&doc.tags)
                .map_err(|e| format!("Failed to serialize tags: {}", e))?;

            tx.execute(
                "INSERT INTO documents (id, content, file_path, content_length, language, tags, created_at, updated_at)
                VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8);",
                params![
                    &doc.id,
                    &doc.content,
                    &doc.file_path,
                    doc.content_length as i64,
                    &doc.language,
                    &tags_json,
                    doc.created_at,
                    doc.updated_at,
                ],
            ).map_err(|e| format!("Failed to insert: {}", e))?;
        }

        tx.commit().map_err(|e| format!("Failed to commit: {}", e))?;

        Ok(())
    }

    /// 从Row构建IndexedDocument
    fn document_from_row(row: &Row) -> IndexedDocument {
        let tags_str: String = row.get("tags").unwrap_or_default();
        let tags: Vec<String> = serde_json::from_str(&tags_str).unwrap_or_default();

        IndexedDocument {
            id: row.get("id").unwrap_or_default(),
            content: row.get("content").unwrap_or_default(),
            file_path: row.get("file_path").unwrap_or_default(),
            content_length: row.get("content_length").unwrap_or(0) as usize,
            language: row.get("language").unwrap_or_default(),
            tags,
            created_at: row.get("created_at").unwrap_or(0),
            updated_at: row.get("updated_at").unwrap_or(0),
        }
    }

    /// 搜索文档
    pub fn search(&self, query: &str, max_results: usize) -> Result<Vec<IndexedDocument>, String> {
        let mut conn_opt = self.conn.lock().unwrap();
        let conn = conn_opt.as_mut().ok_or("Connection not initialized")?;

        let mut stmt = conn.prepare(
            "SELECT id, content, file_path, content_length, language, tags, created_at, updated_at
             FROM documents
             WHERE content LIKE '%' || ?1 || '%'
             ORDER BY content_length DESC
             LIMIT ?2;"
        ).map_err(|e| format!("Failed to prepare: {}", e))?;

        let mut results = Vec::new();
        let mut rows = stmt.query(params![query, max_results as i64])
            .map_err(|e| format!("Failed to query: {}", e))?;

        while let Ok(Some(row)) = rows.next() {
            results.push(Self::document_from_row(row));
        }

        Ok(results)
    }

    /// 精确搜索（使用 FTS）
    pub fn fuzzy_search(&self, _query: &str, _max_results: usize, _threshold: f64)
        -> Result<Vec<(IndexedDocument, f64)>, String> {

        let mut conn_opt = self.conn.lock().unwrap();
        let conn = conn_opt.as_mut().ok_or("Connection not initialized")?;

        let mut results = Vec::new();

        let mut stmt = conn.prepare(
            "SELECT id, content, file_path, content_length, language, tags, created_at, updated_at
             FROM documents
             WHERE content LIKE '%' || ?1 || '%'
             ORDER BY content_length DESC
             LIMIT ?2;"
        ).map_err(|e| format!("Failed to prepare: {}", e))?;

        let mut rows = stmt.query(params![_query, _max_results as i64])
            .map_err(|e| format!("Failed to query: {}", e))?;

        while let Ok(Some(row)) = rows.next() {
            let score = 0.5;
            results.push((Self::document_from_row(row), score));
        }

        Ok(results)
    }

    /// 按文件路径搜索
    pub fn search_by_file(&self, file_path: &str, max_results: usize)
        -> Result<Vec<IndexedDocument>, String> {

        let mut conn_opt = self.conn.lock().unwrap();
        let conn = conn_opt.as_mut().ok_or("Connection not initialized")?;

        let mut stmt = conn.prepare(
            "SELECT id, content, file_path, content_length, language, tags, created_at, updated_at
             FROM documents
             WHERE file_path LIKE '%' || ?1 || '%'
             ORDER BY created_at DESC
             LIMIT ?2;"
        ).map_err(|e| format!("Failed to prepare: {}", e))?;

        let mut results = Vec::new();
        let mut rows = stmt.query(params![file_path, max_results as i64])
            .map_err(|e| format!("Failed to query: {}", e))?;

        while let Ok(Some(row)) = rows.next() {
            results.push(Self::document_from_row(row));
        }

        Ok(results)
    }

    /// 按标签搜索
    pub fn search_by_tag(&self, tag: &str, max_results: usize)
        -> Result<Vec<IndexedDocument>, String> {

        let mut conn_opt = self.conn.lock().unwrap();
        let conn = conn_opt.as_mut().ok_or("Connection not initialized")?;

        let mut stmt = conn.prepare(
            "SELECT id, content, file_path, content_length, language, tags, created_at, updated_at
             FROM documents
             WHERE tags LIKE '%' || ?1 || '%'
             ORDER BY created_at DESC
             LIMIT ?2;"
        ).map_err(|e| format!("Failed to prepare: {}", e))?;

        let mut results = Vec::new();
        let mut rows = stmt.query(params![tag, max_results as i64])
            .map_err(|e| format!("Failed to query: {}", e))?;

        while let Ok(Some(row)) = rows.next() {
            results.push(Self::document_from_row(row));
        }

        Ok(results)
    }

    /// 删除文档
    pub fn delete_document(&self, id: &str) -> Result<(), String> {
        let mut conn_opt = self.conn.lock().unwrap();
        let conn = conn_opt.as_mut().ok_or("Connection not initialized")?;

        conn.execute("DELETE FROM documents WHERE id = ?1;", params![id])
            .map_err(|e| format!("Failed to delete: {}", e))?;

        Ok(())
    }

    /// 清空索引
    pub fn clear(&self) -> Result<(), ()> {
        let mut conn_opt = self.conn.lock().unwrap();
        let conn = conn_opt.as_mut().ok_or(())?;

        conn.execute("DELETE FROM documents;", [])
            .map_err(|_| ())?;

        Ok(())
    }

    /// 获取文档数量
    pub fn count(&self) -> Result<usize, String> {
        let mut conn_opt = self.conn.lock().unwrap();
        let conn = conn_opt.as_mut().ok_or("Connection not initialized")?;

        let mut stmt = conn.prepare("SELECT COUNT(*) as count FROM documents;")
            .map_err(|e| format!("Failed to prepare: {}", e))?;

        let mut rows = stmt.query([])
            .map_err(|e| format!("Failed to query: {}", e))?;

        while let Ok(Some(row)) = rows.next() {
            let count: i64 = row.get("count").unwrap_or(0);
            return Ok(count as usize);
        }

        Ok(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keyword_index() {
        // 测试需要临时数据库
        // TODO: 使用内存 SQLite 进行测试
    }
}
