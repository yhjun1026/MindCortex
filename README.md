# CortexMind 🧠

> 你的AI经验，持续进化

CortexMind 统一连接 OpenCode、ClaudeCode、OpenClaw 等所有 AI 工具，自动将对话记录、代码片段、设计方案和思考过程整理成个人知识库。通过智能提取、向量检索和多模态支持，将零散的 AI 交互转化为连贯的知识图谱，随时检索已涉猎的领域、积累的经验和最佳实践，让你的智慧随每一次使用而持续增长。

---

## ✨ 功能特性

### 🔌 多Agent连接器
- 支持的Agent：OpenCode, ClaudeCode, OpenClaw, Cursor, Windsurf, Cline 等
- 集成方式：
  - 日志/历史文件监听
  - API/Webhook对接
  - 浏览器扩展捕获
  - 终端输出拦截

### 🧠 知识提取引擎
- 自动收集：对话记录、代码片段、设计方案、思考过程
- 智能分类：
  - 按项目/任务归类
  - 按技术栈/领域标记
  - 按时间线组织
- AI整理：
  - 自动生成知识摘要
  - 提取关键经验和最佳实践
  - 识别技术盲区和学习路径

### 📚 知识管理系统
- 存储：本地文件系统（Markdown/JSON）
- 检索：向量数据库（ChromaDB默认，可选Qdrant/Milvus）
- 管理：SQLite（任务元数据、配置、标签）
- 多模态：支持代码、文本、图片、PDF

### ⚙️ AI配置
- 可配置的模型接入：
  - Embedding模型（用于向量检索）
  - Rerank模型（结果重排序）
  - 推理模型（知识整理和问答）
  - 图片模型（视觉理解）
- 支持本地模型（Ollama/LM Studio）和云API

### ☁️ 数据同步
- 云端备份（支持S3、WebDAV、自建服务器）
- 设备间导入/恢复
- 增量同步（只传变更）

---

## 🏗️ 技术栈

- **前端**: React + TypeScript + TailwindCSS + Vite
- **后端**: Rust (Tauri Core)
- **数据库**: SQLite + ChromaDB
- **平台**: Windows, macOS, Linux

---

## 🚀 快速开始

### 环境要求

- Node.js 18+
- Rust 1.70+
- pnpm/npm/yarn

### 安装依赖

```bash
pnpm install
```

### 开发模式

```bash
pnpm tauri dev
```

### 构建

```bash
pnpm tauri build
```

---

## 📂 项目结构

```
MindCortex/
├── src-tauri/          # Rust后端
│   ├── src/
│   │   ├── agents/      # Agent连接器
│   │   ├── extractor/   # 知识提取引擎
│   │   ├── storage/     # 文件系统管理
│   │   ├── database/    # SQLite操作
│   │   └── vector/      # 向量数据库集成
├── src/                 # React前端
│   ├── components/      # UI组件
│   ├── pages/           # 页面
│   └── hooks/           # 数据hooks
├── knowledge/           # 知识库存储目录
│   ├── projects/        # 按项目分类
│   ├── agents/          # 按Agent分类
│   └── timeline/        # 按时间线
└── config/              # 配置文件
```

---

## 🤝 贡献

欢迎提交 Issue 和 Pull Request！

## 📄 许可证

MIT License

---

**CortexMind - 你的AI经验中枢** 🧠
