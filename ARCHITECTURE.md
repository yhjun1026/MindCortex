# CortexMind 架构设计

## 1. 系统架构

```
┌─────────────────────────────────────────────────────────────────┐
│                         Frontend (React)                         │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐        │
│  │ Dashboard│  │ Knowledge│  │  Search  │  │ Settings │        │
│  └────┬─────┘  └────┬─────┘  └────┬─────┘  └────┬─────┘        │
└───────┼─────────────┼─────────────┼─────────────┼────────────────┘
        │             │             │             │
        └─────────────┴─────────────┴─────────────┘
                      Tauri IPC (Command/Event)
┌─────────────────────────────────────────────────────────────────┐
│                     Backend (Rust)                             │
│  ┌─────────────────────────────────────────────────────────┐   │
│  │                    Core Layer                          │   │
│  │  ┌────────┐  ┌────────┐  ┌────────┐  ┌────────┐      │   │
│  │  │ Agents │  │Extractor│  │Storage │  │ Vector │      │   │
│  │  └───┬────┘  └───┬────┘  └───┬────┘  └───┬────┘      │   │
│  └──────┼────────────┼────────────┼────────────┼───────────┘   │
│         │            │            │            │               │
│  ┌──────┴────────────┴────────────┴────────────┴───────────┐   │
│  │                Data Layer                            │   │
│  │  ┌────────────────────────────────────────────────┐  │   │
│  │  │  SQLite (metadata) + ChromaDB (vectors)        │  │   │
│  │  └────────────────────────────────────────────────┘  │   │
│  └───────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│                    External AI Agents                         │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐        │
│  │ OpenCode │  │ClaudeCode│  │OpenClaw  │  │ Cursor   │        │
│  └────┬─────┘  └────┬─────┘  └────┬─────┘  └────┬─────┘        │
└───────┼─────────────┼─────────────┼─────────────┼────────────────┘
        │             │             │             │
        ▼             ▼             ▼             ▼
  (Log Files)   (API/Logs)   (Webhook)   (Terminal)
```

## 2. 核心模块

### 2.1 Agent连接器 (agents/)

负责与各AI工具建立连接和数据采集：

```rust
// src-tauri/src/agents/mod.rs
pub mod opencode;
;
pub mod openclaw;
pub mod cursor;
pub mod generic; // 通用文件监听

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentConfig {
    pub agent_type: String,        // "opencode", "claudecode", etc.
    pub connection_type: String,    // "file", "api", "webhook", "terminal"
    pub config: serde_json::Value,  // Agent specific config
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionData {
    pub id: String,
    pub agent_type: String,
    pub timestamp: i64,
    pub messages: Vec<Message>,
    pub metadata: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub role: String,               // "user", "assistant", "system"
    pub content: String,
    pub timestamp: i64,
    pub attachments: Vec<Attachment>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attachment {
    pub r#type: String,             // "code", "image", "file"
    pub content: String,
    pub language: Option<String>,   // for code
}
```

### 2.2 知识提取引擎 (extractor/)

使用AI模型处理原始数据，提取和整理知识：

```rust
// src-tauri/src/extractor/mod.rs
pub mod classification;    // 分类：项目、任务、技术栈
pub mod summarization;     // 摘要生成
pub mod insight;          // 洞察提取

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeItem {
    pub id: String,
    pub source_session_id: String,
    pub item_type: String,           // "code", "design", "insight", "task"
    pub title: String,
    pub summary: String,
    pub content: String,
    pub tags: Vec<String>,            // 技术栈标签
    pub project: Option<String>,      // 所属项目
    pub embedding: Option<Vec<f32>>, // 向量（可选，可能由ChromaDB生成）
    pub timestamp: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectNode {
    pub id: String,
    pub name: String,
    pub description: String,
    pub technologies: Vec<String>,
    pub tasks: Vec<String>,           // 关联的任务ID
    pub knowledge_items: Vec<String>, // 关联的知识项ID
    pub created_at: i64,
    pub updated_at: i64,
}
```

### 2.3 存储管理 (storage/)

文件系统管理，组织知识库：

```rust
// src-tauri/src/storage/mod.rs
pub mod file_manager;
pub mod project_tree;

#[derive(Debug, Clone)]
pub struct StorageConfig {
    pub base_path: String,
    pub projects_path: String,
    pub agents_path: String,
    pub timeline_path: String,
}

// 知识库文件结构
// knowledge/
//   projects/
//     {project_id}/
//       metadata.json
//       tasks/
//         {task_id}.md
//       insights/
//         {insight_id}.md
//   agents/
//     {agent_type}/
//       sessions/
//         {session_id}.json
//   timeline/
//     {year}/{month}/
//       {day}.json
```

### 2.4 向量数据库集成 (vector/)

集成ChromaDB进行向量检索：

```rust
// src-tauri/src/vector/mod.rs
pub mod chromadb;

#[derive(Debug, Clone)]
pub struct VectorConfig {
    pub db_path: String,
    pub collection_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub id: String,
    pub score: f32,
    pub content: String,
    pub metadata: serde_json::Value,
}
```

## 3. 数据库设计 (SQLite)

### 3.1 核心表

```sql
-- 项目表
CREATE TABLE projects (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL
);

-- 任务表
CREATE TABLE tasks (
    id TEXT PRIMARY KEY,
    project_id TEXT,
    title TEXT NOT NULL,
    status TEXT,  -- active, completed, archived
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL,
    FOREIGN KEY (project_id) REFERENCES projects(id)
);

-- 会话表
CREATE TABLE sessions (
    id TEXT PRIMARY KEY,
    agent_type TEXT NOT NULL,
    task_id TEXT,
    timestamp INTEGER NOT NULL,
    FOREIGN KEY (task_id) REFERENCES tasks(id)
);

-- 知识项表
CREATE TABLE knowledge_items (
    id TEXT PRIMARY KEY,
    session_id TEXT,
    task_id TEXT,
    item_type TEXT NOT NULL,
    title TEXT,
    summary TEXT,
    tags TEXT,  -- JSON array
    created_at INTEGER NOT NULL,
    FOREIGN KEY (session_id) REFERENCES sessions(id),
    FOREIGN KEY (task_id) REFERENCES tasks(id)
);

-- 标签表
CREATE TABLE tags (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    category TEXT  -- "technology", "language", "framework", etc.
);

-- 索引
CREATE INDEX idx_sessions_agent ON sessions(agent_type);
CREATE INDEX idx_sessions_task ON sessions(task_id);
CREATE INDEX idx_knowledge_session ON knowledge_items(session_id);
CREATE INDEX idx_knowledge_task ON knowledge_items(task_id);
CREATE INDEX idx_knowledge_type ON knowledge_items(item_type);
```

## 4. 前端架构 (React + TypeScript)

### 4.1 页面结构

```
src/
├── App.tsx
├── main.tsx
├── components/
│   ├── common/
│   │   ├── Layout.tsx
│   │   ├── Sidebar.tsx
│   │   └── Header.tsx
│   ├── knowledge/
│   │   ├── KnowledgeTree.tsx
│   │   ├── KnowledgeCard.tsx
│   │   └── CodeBlock.tsx
│   └── search/
│       ├── SearchBar.tsx
│       ├── SearchResults.tsx
│       └── FilterPanel.tsx
├── pages/
│   ├── Dashboard.tsx
│   ├── Knowledge.tsx
│   ├── Search.tsx
│   ├── Projects.tsx
│   ├── Agents.tsx
│   └── Settings.tsx
└── hooks/
    ├── useKnowledge.ts
    ├── useSearch.ts
    ├── useAgents.ts
    └── useSettings.ts
```

### 4.2 状态管理

使用 React Context + Hooks 进行状态管理：

```typescript
// src/contexts/AppContext.tsx
interface AppState {
  // 知识库状态
  knowledge: KnowledgeState;
  // 搜索状态
  search: SearchState;
  // Agent连接状态
  agents: AgentsState;
  // 设置
  settings: SettingsState;
}

interface KnowledgeState {
  projects: Project[];
  currentProject: Project | null;
  knowledgeItems: KnowledgeItem[];
  loading: boolean;
}

interface SearchState {
  query: string;
  results: SearchResult[];
  filters: SearchFilters;
  searching: boolean;
}
```

## 5. Tauri Commands

前端与后端通信的命令接口：

```rust
// src-tauri/src/lib.rs
#[tauri::command]
async fn search_knowledge(
    query: String,
    filters: SearchFilters,
    state: State<'_, AppState>,
) -> Result<Vec<SearchResult>, String> {
    // 实现逻辑
}

#[tauri::command]
async fn get_projects(state: State<'_, AppState>) -> Result<Vec<Project>, String> {
    // 实现逻辑
}

#[tauri::command]
async fn get_knowledge_items(
    project_id: Option<String>,
    task_id: Option<String>,
    state: State<'_, AppState>,
) -> Result<Vec<KnowledgeItem>, String> {
    // 实现逻辑
}

#[tauri::command]
async fn add_agent_connection(
    config: AgentConfig,
    state: State<'_, AppState>,
) -> Result<String, String> {
    // 实现逻辑
}

#[tauri::command]
async fn sync_agent_sessions(
    agent_id: String,
    state: State<'_, AppState>,
) -> Result<Vec<SessionData>, String> {
    // 实现逻辑
}
```

## 6. 配置系统

### 6.1 配置文件结构

```json
{
  "version": "1.0.0",
  "models": {
    "embedding": {
      "provider": "ollama",  // or "openai", "anthropic", etc.
      "model": "nomic-embed-text",
      "api_key": "",
      "base_url": "http://localhost:11434"
    },
    "rerank": {
      "provider": "ollama",
      "model": "bge-reranker-v2",
      "api_key": "",
      "base_url": "http://localhost:11434"
    },
    "reasoning": {
      "provider": "ollama",
      "model": "llama3.2",
      "api_key": "",
      "base_url": "http://localhost:11434"
    },
    "image": {
      "provider": "ollama",
      "model": "llava",
      "api_key": "",
      "base_url": "http://localhost:11434"
    }
  },
  "vector_db": {
    "type": "chromadb",
    "path": "./data/chromadb",
    "collection_name": "cortexmind"
  },
  "storage": {
    "base_path": "./knowledge",
    "auto_sync": true,
    "sync_interval": 300  // seconds
  },
  "agents": []
}
```

## 7. 开发路线图

### Phase 1: 基础框架 (v0.1.0)
- [x] Tauri项目初始化
- [ ] SQLite数据库设计实现
- [ ] 基础UI框架（Dashboard, Settings）
- [ ] 配置系统

### Phase 2: Agent连接 (v0.2.0)
- [ ] OpenCode连接器
- [ ] OpenClaw连接器
- [ ] 通用文件监听器
- [ ] 会话数据采集

### Phase 3: 知识提取 (v0.3.0)
- [ ] 基础分类逻辑
- [ ] AI摘要生成
- [ ] 项目自动识别
- [ ] 技术栈标签提取

### Phase 4: 向量检索 (v0.4.0)
- [ ] ChromaDB集成
- [ ] 向量嵌入
- [ ] 语义搜索
- [ ] 结果重排序

### Phase 5: 知识管理 (v0.5.0)
- [ ] 知识树可视化
- [ ] 按时间线浏览
- [ ] 多维度筛选
- [ ] 知识导出

### Phase 6: 同步备份 (v0.6.0)
- [ ] 云端备份（S3）
- [ ] 设备间同步
- [ ] 增量同步
- [ ] 冲突解决

---

**架构版本**: 1.0.0
**最后更新**: 2026-02-28
