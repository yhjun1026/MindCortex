# MindCortex VSCode 扩展

VSCode 扩展，提供智能知识库搜索、代码助手和知识图谱功能。

## 功能特性

### 1. 插件基础框架
- ✅ VSCode 扩展 API 集成
- ✅ 命令面板实现
- ✅ 状态栏集成
- ✅ 消息通知系统

### 2. 搜索功能集成
- ✅ 快捷键搜索（Cmd+Shift+M）
- ✅ 选中内容搜索（Cmd+Shift+Alt+M）
- ✅ 右键上下文菜单
- ✅ 搜索面板（Webview）
- ✅ 搜索历史管理

### 3. 代码助手集成
- ✅ 代码片段管理
- ✅ 片段搜索和插入
- ✅ 最佳实践建议
- ✅ 问题解决方案
- ✅ 代码推荐

### 4. 会话管理
- ✅ 会话创建和切换
- ✅ 会话标签分类
- ✅ 会话元数据管理
- ✅ 会话导出导入
- ✅ 自动记录

## 安装

### 从源码构建

1. 克隆仓库：
```bash
git clone https://github.com/mindcortex/MindCortex.git
cd MindCortex/vscode-extension
```

2. 安装依赖：
```bash
npm install
```

3. 编译：
```bash
npm run compile
```

4. 在 VSCode 中运行：
- 按 F5 启动扩展开发主机

## 使用说明

### 搜索功能

#### 快捷键搜索
- `Cmd+Shift+M` (Mac) / `Ctrl+Shift+M` (Windows/Linux) 打开搜索面板
- `Cmd+Shift+Alt+M` (Mac) / `Ctrl+Shift+Alt+M` (Windows/Linux) 搜索选中文本

#### 右键菜单
- 选中代码或文本后，右键菜单中选择"搜索选中文本"

#### 命令面板
- `Cmd+Shift+P` (Mac) / `Ctrl+Shift+P` (Windows/Linux)
- 输入 "MindCortex" 查看所有可用命令

### 代码助手

#### 添加代码片段
1. 选择要保存的代码
2. 右键菜单 → "添加代码片段"
3. 输入标题、描述和标签

#### 搜索代码片段
1. 打开代码助手面板
2. 在搜索框中输入关键词
3. 点击插入按钮将代码片段插入到编辑器

### 会话管理

#### 创建会话
- 命令面板 → "管理会话" → "创建新会话"

#### 切换会话
- 命令面板 → "管理会话" → "切换会话"

## 配置

扩展支持以下配置选项（在 VSCode 设置中搜索 "MindCortex"）：

| 配置项 | 类型 | 默认值 | 说明 |
|--------|------|--------|------|
| `mindcortex.apiEndpoint` | string | `http://localhost:3000` | MindCortex API 端点 |
| `mindcortex.autoIndex` | boolean | `true` | 自动索引代码文件 |
| `mindcortex.indexInterval` | number | `300000` | 自动索引间隔（毫秒） |
| `mindcortex.enableCodeAssistant` | boolean | `true` | 启用代码助手 |
| `mindcortex.maxSearchResults` | number | `20` | 最大搜索结果数 |
| `mindcortex.debug` | boolean | `false` | 启用调试日志 |

## 架构

### 管理器（Managers）

- **StatusBarManager**: 状态栏管理
- **CommandManager**: 命令管理
- **WebviewManager**: Webview 面板管理
- **SessionManager**: 会话管理
- **SearchManager**: 搜索功能管理
- **CodeAssistantManager**: 代码助手管理

### 工具类（Utils）

- **Logger**: 日志记录工具

## 开发

### 项目结构

```
vscode-extension/
├── src/
│   ├── extension.ts          # 扩展入口
│   ├── managers/             # 管理器
│   │   ├── StatusBarManager.ts
│   │   ├── CommandManager.ts
│   │   ├── WebviewManager.ts
│   │   ├── SessionManager.ts
│   │   ├── SearchManager.ts
│   │   └── CodeAssistantManager.ts
│   ├── utils/                # 工具类
│   │   └── Logger.ts
│   └── types/                # 类型定义
│       └── index.ts
├── resources/
│   └── icon.svg              # 扩展图标
├── package.json              # 扩展清单
├── tsconfig.json             # TypeScript 配置
└── README.md
```

### 编译

```bash
npm run compile
```

### 测试

```bash
npm run test
```

## 待实现功能

### Phase 3: 知识图谱可视化
- [ ] 图谱数据构建
- [ ] 图谱可视化引擎（Cytoscape.js 集成）
- [ ] 图谱分析功能
- [ ] 前端图谱视图

### Phase 4: 自然语言查询
- [ ] RAG 框架
- [ ] 查询优化
- [ ] 多模态查询
- [ ] AI 对话界面

## 贡献

欢迎提交 Issue 和 Pull Request！

## 许可证

MIT License

## 联系方式

- 项目主页: https://github.com/mindcortex/MindCortex
- 问题反馈: https://github.com/mindcortex/MindCortex/issues
