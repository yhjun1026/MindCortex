// 任务管理模块
// 使用 SQLite 和 Markdown 文件系统管理任务

use rusqlite::{Connection, Result as SqlResult};
use serde::{Deserialize, Serialize};
use chrono::{Utc};
use uuid::Uuid;
use std::fs;
use std::path::PathBuf;
use std::io;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: String,
    pub title: String,
    pub description: String,
    pub status: String, // "todo", "in-progress", "completed"
    pub priority: String, // "low", "medium", "high"
    pub due_date: Option<i64>,
    pub created_at: i64,
    pub updated_at: i64,
    pub tags: Vec<String>,
    pub markdown_file: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskBackup {
    pub version: String,
    pub exported_at: i64,
    pub tasks: Vec<Task>,
}

pub struct TaskManager {
    conn: Connection,
    base_dir: PathBuf,
}

impl TaskManager {
    pub fn new(db_path: &str, base_dir: &Path) -> SqlResult<Self> {
        let conn = Connection::open(db_path)?;
        let manager = TaskManager {
            conn,
            base_dir: base_dir.to_path_buf(),
        };
        manager.init_tables()?;
        Ok(manager)
    }

    fn init_tables(&self) -> SqlResult<()> {
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS tasks (
                id TEXT PRIMARY KEY,
                title TEXT NOT NULL,
                description TEXT DEFAULT '',
                status TEXT DEFAULT 'todo',
                priority TEXT DEFAULT 'medium',
                due_date INTEGER,
                created_at INTEGER NOT NULL,
                updated_at INTEGER NOT NULL,
                tags TEXT DEFAULT '[]',
                markdown_file TEXT
            )",
            [],
        )?;

        self.conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_tasks_status ON tasks(status)",
            [],
        )?;

        self.conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_tasks_priority ON tasks(priority)",
            [],
        )?;

        self.conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_tasks_updated ON tasks(updated_at)",
            [],
        )?;

        Ok(())
    }

    pub fn create_task(&self, title: &str) -> SqlResult<Task> {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now().timestamp();
        let task_dir = self.base_dir.join(&id);

        // 创建任务目录
        if !task_dir.exists() {
            fs::create_dir_all(&task_dir)
                .map_err(|e| rusqlite::Error::ToSqlConversion(e.to_string()))?;
        }

        // 创建 Markdown 文件
        let markdown_file = task_dir.join("task.md");
        let markdown_path = markdown_file.to_string_lossy().to_string();

        let content = format!(
            "# {}\n\n\n---\n\n**创建时间:** {}\n**状态:** 待办\n**优先级:** 中\n\n## 标签\n\n\n## 子任务\n\n- [ ] \n\n## 备注\n\n",
            title,
            Utc::now().format("%Y-%m-%d %H:%M:%S")
        );

        fs::write(&markdown_file, content)
            .map_err(|e| rusqlite::Error::ToSqlConversion(e.to_string()))?;

        // 保存到数据库
        self.conn.execute(
            "INSERT INTO tasks (id, title, description, status, priority, created_at, updated_at, tags, markdown_file) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            [
                &id,
                title,
                "",
                "todo",
                "medium",
                &now.to_string(),
                &now.to_string(),
                "[]",
                &markdown_path,
            ],
        )?;

        Ok(Task {
            id,
            title: title.to_string(),
            description: String::new(),
            status: "todo".to_string(),
            priority: "medium".to_string(),
            due_date: None,
            created_at: now,
            updated_at: now,
            tags: Vec::new(),
            markdown_file: Some(markdown_path),
        })
    }

    pub fn update_task(&self, task: &Task) -> SqlResult<Task> {
        let now = Utc::now().timestamp();
        let tags_json = serde_json::to_string(&task.tags)
            .map_err(|e| rusqlite::Error::ToSqlConversion(e.to_string()))?;

        self.conn.execute(
            "UPDATE tasks SET title = ?1, description = ?2, status = ?3, priority = ?4, due_date = ?5, updated_at = ?6, tags = ?7 WHERE id = ?8",
            [
                &task.title,
                &task.description,
                &task.status,
                &task.priority,
                &task.due_date.map(|d| d.to_string()).unwrap_or_else(|| "NULL".to_string()),
                &now.to_string(),
                &tags_json,
                &task.id,
            ],
        )?;

        // 同步到 Markdown 文件
        if let Some(markdown_path) = &task.markdown_file {
            self.sync_to_markdown(task, markdown_path)?;
        }

        Ok(Task {
            ..task.clone()
        })
    }

    fn sync_to_markdown(&self, task: &Task, markdown_path: &str) -> SqlResult<()> {
        let content = format!(
            "# {}\n\n{}\n\n---\n\n**创建时间:** {}\n**更新时间:** {}\n**状态:** {}\n**优先级:** {}\n\n## 标签\n\n{}\n\n## 子任务\n\n- [ ] \n\n## 备注\n\n",
            task.title,
            task.description,
            Utc::timestamp_millis_opt(task.created_at).unwrap().format("%Y-%m-%d %H:%M:%S"),
            Utc::timestamp_millis_opt(task.updated_at).unwrap().format("%Y-%m-%d %H:%M:%S"),
            task.status,
            task.priority,
            task.tags.iter()
                .map(|t| format!("- #{}", t))
                .collect::<Vec<_>>()
                .join("\n")
        );

        fs::write(&markdown_path, content)
            .map_err(|e| rusqlite::Error::ToSqlConversion(e.to_string()))?;

        Ok(())
    }

    pub fn get_task(&self, id: &str) -> SqlResult<Option<Task>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, title, description, status, priority, due_date, created_at, updated_at, tags, markdown_file FROM tasks WHERE id = ?1"
        )?;

        let task_iter = stmt.query_map([id], |row| {
            Ok(Task {
                id: row.get(0)?,
                title: row.get(1)?,
                description: row.get(2)?,
                status: row.get(3)?,
                priority: row.get(4)?,
                due_date: row.get(5)?,
                created_at: row.get(6)?,
                updated_at: row.get(7)?,
                tags: serde_json::from_str(row.get::<_, String>(8)?.as_str())
                    .unwrap_or_else(|_| Vec::new()),
                markdown_file: row.get(9)?,
            })
        })?;

        for task in task_iter {
            return Ok(Some(task?));
        }
        Ok(None)
    }

    pub fn list_tasks(&self) -> SqlResult<Vec<Task>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, title, description, status, priority, due_date, created_at, updated_at, tags, markdown_file FROM tasks ORDER BY updated_at DESC"
        )?;

        let task_iter = stmt.query_map([], |row| {
            Ok(Task {
                id: row.get(0)?,
                title: row.get(1)?,
                description: row.get(2)?,
                status: row.get(3)?,
                priority: row.get(4)?,
                due_date: row.get(5)?,
                created_at: row.get(6)?,
                updated_at: row.get(7)?,
                tags: serde_json::from_str(row.get::<_, String>(8)?.as_str())
                    .unwrap_or_else(|_| Vec::new()),
                markdown_file: row.get(9)?,
            })
        })?;

        let mut tasks = Vec::new();
        for task in task_iter {
            tasks.push(task?);
        }
        Ok(tasks)
    }

    pub fn delete_task(&self, id: &str) -> SqlResult<()> {
        // 先获取任务以获取 markdown 文件路径
        if let Some(task) = self.get_task(id)? {
            // 删除 Markdown 文件和目录
            if let Some(markdown_path) = task.markdown_file {
                let path = Path::new(&markdown_path);
                if let Some(parent) = path.parent() {
                    if parent.exists() {
                        fs::remove_dir_all(parent)
                            .map_err(|e| rusqlite::Error::ToSqlConversion(e.to_string()))?;
                    }
                }
            }
        }

        // 从数据库删除
        self.conn.execute("DELETE FROM tasks WHERE id = ?1", [id])?;
        Ok(())
    }

    pub fn get_tasks_by_status(&self, status: &str) -> SqlResult<Vec<Task>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, title, description, status, priority, due_date, created_at, updated_at, tags, markdown_file FROM tasks WHERE status = ?1 ORDER BY updated_at DESC"
        )?;

        let task_iter = stmt.query_map([status], |row| {
            Ok(Task {
                id: row.get(0)?,
                title: row.get(1)?,
                description: row.get(2)?,
                status: row.get(3)?,
                priority: row.get(4)?,
                due_date: row.get(5)?,
                created_at: row.get(6)?,
                updated_at: row.get(7)?,
                tags: serde_json::from_str(row.get::<_, String>(8)?.as_str())
                    .unwrap_or_else(|_| Vec::new()),
                markdown_file: row.get(9)?,
            })
        })?;

        let mut tasks = Vec::new();
        for task in task_iter {
            tasks.push(task?);
        }
        Ok(tasks)
    }

    pub fn get_tasks_by_tag(&self, tag: &str) -> SqlResult<Vec<Task>> {
        let tasks = self.list_tasks()?;
        let filtered = tasks.into_iter()
            .filter(|t| t.tags.contains(&tag.to_string()))
            .collect();
        Ok(filtered)
    }

    // 实时缓存功能：保存临时修改
    pub fn save_cache(&self, id: &str, content: &str) -> SqlResult<()> {
        let cache_file = self.base_dir.join(format!("cache_{}.json", id));
        fs::write(&cache_file, content)
            .map_err(|e| rusqlite::Error::ToSqlConversion(e.to_string()))?;
        Ok(())
    }

    // 读取缓存
    pub fn load_cache(&self, id: &str) -> SqlResult<Option<String>> {
        let cache_file = self.base_dir.join(format!("cache_{}.json", id));
        if cache_file.exists() {
            let content = fs::read_to_string(&cache_file)
                .map_err(|e| rusqlite::Error::ToSqlConversion(e.to_string()))?;
            Ok(Some(content))
        } else {
            Ok(None)
        }
    }

    // 清除缓存
    pub fn clear_cache(&self, id: &str) -> SqlResult<()> {
        let cache_file = self.base_dir.join(format!("cache_{}.json", id));
        if cache_file.exists() {
            fs::remove_file(&cache_file)
                .map_err(|e| rusqlite::Error::ToSqlConversion(e.to_string()))?;
        }
        Ok(())
    }

    // 备份所有任务到单个文件
    pub fn backup_all(&self, backup_path: &str) -> SqlResult<String> {
        let tasks = self.list_tasks()?;
        let backup = TaskBackup {
            version: "1.0".to_string(),
            exported_at: Utc::now().timestamp(),
            tasks,
        };

        let backup_json = serde_json::to_string_pretty(&backup)
            .map_err(|e| rusqlite::Error::ToSqlConversion(e.to_string()))?;

        fs::write(backup_path, backup_json)
            .map_err(|e| rusqlite::Error::ToSqlConversion(e.to_string()))?;

        Ok(backup_path.to_string())
    }

    // 从备份恢复
    pub fn restore_from_backup(&self, backup_path: &str) -> SqlResult<usize> {
        let backup_content = fs::read_to_string(backup_path)
            .map_err(|e| rusqlite::Error::ToSqlConversion(e.to_string()))?;

        let backup: TaskBackup = serde_json::from_str(&backup_content)
            .map_err(|e| rusqlite::Error::ToSqlConversion(e.to_string()))?;

        let mut restored_count = 0;

        for task in backup.tasks {
            // 检查是否已存在
            if self.get_task(&task.id)?.is_none() {
                self.conn.execute(
                    "INSERT INTO tasks (id, title, description, status, priority, due_date, created_at, updated_at, tags, markdown_file) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
                    [
                        &task.id,
                        &task.title,
                        &task.description,
                        &task.status,
                        &task.priority,
                        &task.due_date.map(|d| d.to_string()).unwrap_or_else(|| "NULL".to_string()),
                        &task.created_at.to_string(),
                        &task.updated_at.to_string(),
                        &serde_json::to_string(&task.tags).unwrap_or_else(|_| "[]".to_string()),
                        &task.markdown_file.unwrap_or_else(|| String::new()),
                    ],
                )?;
                restored_count += 1;
            }
        }

        Ok(restored_count)
    }

    // 导出为单个 Markdown 文件
    pub fn export_markdown(&self, output_path: &str) -> SqlResult<String> {
        let tasks = self.list_tasks()?;
        let mut content = String::from("# MindCortex 任务备份\n\n");
        content.push_str(&format!("**导出时间:** {}\n\n",
            Utc::now().format("%Y-%m-%d %H:%M:%S")
        ));

        content.push_str("---\n\n## 任务列表\n\n");

        for task in &tasks {
            content.push_str(&format!("### {} [{}]\n\n", task.title, task.status));
            if !task.description.is_empty() {
                content.push_str(&format!("{}\n\n", task.description));
            }
            content.push_str(&format!("- **优先级:** {}\n", task.priority));
            content.push_str(&format!("- **状态:** {}\n", task.status));
            if let Some(due_date) = task.due_date {
                content.push_str(&format!("- **截止日期:** {}\n",
                    Utc::timestamp_millis_opt(due_date).unwrap().format("%Y-%m-%d %H:%M:%S")
                ));
            }
            if !task.tags.is_empty() {
                content.push_str(&format!("- **标签:** {}\n", task.tags.join(", ")));
            }
            content.push_str("\n---\n\n");
        }

        fs::write(output_path, content)
            .map_err(|e| rusqlite::Error::ToSqlConversion(e.to_string()))?;

        Ok(output_path.to_string())
    }
}
