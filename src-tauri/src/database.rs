use rusqlite::{Connection, Result as SqlResult};
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: String,
    pub project_id: Option<String>,
    pub title: String,
    pub status: String,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeItem {
    pub id: String,
    pub session_id: Option<String>,
    pub task_id: Option<String>,
    pub item_type: String,
    pub title: Option<String>,
    pub summary: Option<String>,
    pub tags: String, // JSON array as string
    pub created_at: i64,
}

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new(db_path: &str) -> SqlResult<Self> {
        let conn = Connection::open(db_path)?;
        Ok(Database { conn })
    }

    pub fn init_tables(&self) -> SqlResult<()> {
        // Projects table
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS projects (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                description TEXT,
                created_at INTEGER NOT NULL,
                updated_at INTEGER NOT NULL
            )",
            [],
        )?;

        // Tasks table
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS tasks (
                id TEXT PRIMARY KEY,
                project_id TEXT,
                title TEXT NOT NULL,
                status TEXT,
                created_at INTEGER NOT NULL,
                updated_at INTEGER NOT NULL,
                FOREIGN KEY (project_id) REFERENCES projects(id)
            )",
            [],
        )?;

        // Sessions table
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS sessions (
                id TEXT PRIMARY KEY,
                agent_type TEXT NOT NULL,
                task_id TEXT,
                timestamp INTEGER NOT NULL,
                FOREIGN KEY (task_id) REFERENCES tasks(id)
            )",
            [],
        )?;

        // Knowledge items table
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS knowledge_items (
                id TEXT PRIMARY KEY,
                session_id TEXT,
                task_id TEXT,
                item_type TEXT NOT NULL,
                title TEXT,
                summary TEXT,
                tags TEXT,
                created_at INTEGER NOT NULL,
                FOREIGN KEY (session_id) REFERENCES sessions(id),
                FOREIGN KEY (task_id) REFERENCES tasks(id)
            )",
            [],
        )?;

        // Tags table
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS tags (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL UNIQUE,
                category TEXT
            )",
            [],
        )?;

        // Create indexes
        self.conn.execute("CREATE INDEX IF NOT EXISTS idx_sessions_agent ON sessions(agent_type)", [])?;
        self.conn.execute("CREATE INDEX IF NOT EXISTS idx_sessions_task ON sessions(task_id)", [])?;
        self.conn.execute("CREATE INDEX IF NOT EXISTS idx_knowledge_session ON knowledge_items(session_id)", [])?;
        self.conn.execute("CREATE INDEX IF NOT EXISTS idx_knowledge_task ON knowledge_items(task_id)", [])?;
        self.conn.execute("CREATE INDEX IF NOT EXISTS idx_knowledge_type ON knowledge_items(item_type)", [])?;

        Ok(())
    }

    pub fn create_project(&self, name: &str, description: Option<&str>) -> SqlResult<Project> {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now().timestamp();

        self.conn.execute(
            "INSERT INTO projects (id, name, description, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5)",
            [&id, name, &description.unwrap_or(""), &now.to_string(), &now.to_string()],
        )?;

        Ok(Project {
            id,
            name: name.to_string(),
            description: description.map(|s| s.to_string()),
            created_at: now,
            updated_at: now,
        })
    }

    pub fn get_all_projects(&self) -> SqlResult<Vec<Project>> {
        let mut stmt = self.conn.prepare("SELECT id, name, description, created_at, updated_at FROM projects ORDER BY updated_at DESC")?;
        let project_iter = stmt.query_map([], |row| {
            Ok(Project {
                id: row.get(0)?,
                name: row.get(1)?,
                description: row.get(2)?,
                created_at: row.get(3)?,
                updated_at: row.get(4)?,
            })
        })?;

        let mut projects = Vec::new();
        for project in project_iter {
            projects.push(project?);
        }
        Ok(projects)
    }

    pub fn get_project_by_id(&self, id: &str) -> SqlResult<Option<Project>> {
        let mut stmt = self.conn.prepare("SELECT id, name, description, created_at, updated_at FROM projects WHERE id = ?1")?;
        let project_iter = stmt.query_map([id], |row| {
            Ok(Project {
                id: row.get(0)?,
                name: row.get(1)?,
                description: row.get(2)?,
                created_at: row.get(3)?,
                updated_at: row.get(4)?,
            })
        })?;

        for project in project_iter {
            return Ok(Some(project?));
        }
        Ok(None)
    }

    pub fn create_task(&self, project_id: Option<&str>, title: &str, status: &str) -> SqlResult<Task> {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now().timestamp();
        let pid = project_id.unwrap_or("").to_string();

        self.conn.execute(
            "INSERT INTO tasks (id, project_id, title, status, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            [&id, &pid, title, status, &now.to_string(), &now.to_string()],
        )?;

        Ok(Task {
            id,
            project_id: project_id.map(|s| s.to_string()),
            title: title.to_string(),
            status: status.to_string(),
            created_at: now,
            updated_at: now,
        })
    }

    pub fn get_all_tasks(&self) -> SqlResult<Vec<Task>> {
        let mut stmt = self.conn.prepare("SELECT id, project_id, title, status, created_at, updated_at FROM tasks ORDER BY updated_at DESC")?;
        let task_iter = stmt.query_map([], |row| {
            Ok(Task {
                id: row.get(0)?,
                project_id: row.get(1)?,
                title: row.get(2)?,
                status: row.get(3)?,
                created_at: row.get(4)?,
                updated_at: row.get(5)?,
            })
        })?;

        let mut tasks = Vec::new();
        for task in task_iter {
            tasks.push(task?);
        }
        Ok(tasks)
    }

    pub fn get_tasks_by_project(&self, project_id: &str) -> SqlResult<Vec<Task>> {
        let mut stmt = self.conn.prepare("SELECT id, project_id, title, status, created_at, updated_at FROM tasks WHERE project_id = ?1 ORDER BY updated_at DESC")?;
        let task_iter = stmt.query_map([project_id], |row| {
            Ok(Task {
                id: row.get(0)?,
                project_id: row.get(1)?,
                title: row.get(2)?,
                status: row.get(3)?,
                created_at: row.get(4)?,
                updated_at: row.get(5)?,
            })
        })?;

        let mut tasks = Vec::new();
        for task in task_iter {
            tasks.push(task?);
        }
        Ok(tasks)
    }

    pub fn create_knowledge_item(&self, session_id: Option<&str>, task_id: Option<&str>, item_type: &str, title: Option<&str>, summary: Option<&str>, tags: &str) -> SqlResult<KnowledgeItem> {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now().timestamp();
        let sid = session_id.unwrap_or("").to_string();
        let tid = task_id.unwrap_or("").to_string();
        let t = title.unwrap_or("").to_string();
        let s = summary.unwrap_or("").to_string();

        self.conn.execute(
            "INSERT INTO knowledge_items (id, session_id, task_id, item_type, title, summary, tags, created_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            [&id, &sid, &tid, item_type, &t, &s, tags, &now.to_string()],
        )?;

        Ok(KnowledgeItem {
            id,
            session_id: session_id.map(|s| s.to_string()),
            task_id: task_id.map(|s| s.to_string()),
            item_type: item_type.to_string(),
            title: title.map(|s| s.to_string()),
            summary: summary.map(|s| s.to_string()),
            tags: tags.to_string(),
            created_at: now,
        })
    }
}
