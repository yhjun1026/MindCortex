"use strict";
var __createBinding = (this && this.__createBinding) || (Object.create ? (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    var desc = Object.getOwnPropertyDescriptor(m, k);
    if (!desc || ("get" in desc ? !m.__esModule : desc.writable || desc.configurable)) {
      desc = { enumerable: true, get: function() { return m[k]; } };
    }
    Object.defineProperty(o, k2, desc);
}) : (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    o[k2] = m[k];
}));
var __setModuleDefault = (this && this.__setModuleDefault) || (Object.create ? (function(o, v) {
    Object.defineProperty(o, "default", { enumerable: true, value: v });
}) : function(o, v) {
    o["default"] = v;
});
var __importStar = (this && this.__importStar) || (function () {
    var ownKeys = function(o) {
        ownKeys = Object.getOwnPropertyNames || function (o) {
            var ar = [];
            for (var k in o) if (Object.prototype.hasOwnProperty.call(o, k)) ar[ar.length] = k;
            return ar;
        };
        return ownKeys(o);
    };
    return function (mod) {
        if (mod && mod.__esModule) return mod;
        var result = {};
        if (mod != null) for (var k = ownKeys(mod), i = 0; i < k.length; i++) if (k[i] !== "default") __createBinding(result, mod, k[i]);
        __setModuleDefault(result, mod);
        return result;
    };
})();
Object.defineProperty(exports, "__esModule", { value: true });
exports.WebviewManager = void 0;
const vscode = __importStar(require("vscode"));
const Logger_1 = require("../utils/Logger");
/**
 * Webview 管理器
 * 负责创建和管理所有 webview 面板
 */
class WebviewManager {
    constructor(context) {
        this.panels = new Map();
        this.context = context;
    }
    /**
     * 打开主面板
     */
    openMainPanel() {
        this.openWebviewPanel('mindcortex.main', 'MindCortex', this.getMainWebviewContent());
    }
    /**
     * 打开搜索面板
     */
    openSearchPanel() {
        return this.openWebviewPanel('mindcortex.search', 'MindCortex 搜索', this.getSearchWebviewContent());
    }
    /**
     * 打开知识图谱
     */
    openKnowledgeGraph() {
        this.openWebviewPanel('mindcortex.graph', 'MindCortex 知识图谱', this.getGraphWebviewContent());
    }
    /**
     * 打开代码助手
     */
    openCodeAssistant() {
        return this.openWebviewPanel('mindcortex.assistant', 'MindCortex 代码助手', this.getAssistantWebviewContent());
    }
    /**
     * 通用 Webview 面板创建方法
     */
    openWebviewPanel(viewType, title, content) {
        // 如果面板已存在，则显示它
        if (this.panels.has(viewType)) {
            const panel = this.panels.get(viewType);
            if (panel) {
                panel.reveal(vscode.ViewColumn.One);
                return panel;
            }
        }
        // 创建新面板
        const panel = vscode.window.createWebviewPanel(viewType, title, vscode.ViewColumn.One, {
            enableScripts: true,
            retainContextWhenHidden: true
        });
        // 设置 Webview 内容
        panel.webview.html = content;
        // 设置消息处理
        panel.webview.onDidReceiveMessage(async (message) => {
            await this.handleWebviewMessage(viewType, message, panel);
        }, undefined, this.context.subscriptions);
        // 关闭时清理
        panel.onDidDispose(() => {
            this.panels.delete(viewType);
            Logger_1.Logger.info(`Webview 面板已关闭: ${viewType}`);
        }, this.context.subscriptions);
        this.panels.set(viewType, panel);
        Logger_1.Logger.info(`Webview 面板已创建: ${viewType}`);
        return panel;
    }
    /**
     * 处理 Webview 消息
     */
    async handleWebviewMessage(viewType, message, panel) {
        Logger_1.Logger.info(`收到 Webview 消息 [${viewType}]: ${message.type}`);
        switch (message.type) {
            case 'search':
                await this.handleSearchMessage(message, panel);
                break;
            case 'assistant':
                await this.handleAssistantMessage(message, panel);
                break;
            case 'graph':
                await this.handleGraphMessage(message, panel);
                break;
            case 'ready':
                // Webview 准备就绪
                panel.webview.postMessage({ type: 'initialized' });
                break;
            default:
                Logger_1.Logger.warn(`未知的消息类型: ${message.type}`);
        }
    }
    /**
     * 处理搜索消息
     */
    async handleSearchMessage(message, panel) {
        // TODO: 实现搜索逻辑
        const { query } = message;
        // 发送搜索结果回 Webview
        panel.webview.postMessage({
            type: 'searchResults',
            results: []
        });
    }
    /**
     * 处理代码助手消息
     */
    async handleAssistantMessage(message, panel) {
        const { action } = message;
        switch (action) {
            case 'insertCode':
                // 插入代码到编辑器
                const editor = vscode.window.activeTextEditor;
                const code = message.code;
                if (editor && code) {
                    await editor.edit(editBuilder => {
                        editBuilder.insert(editor.selection.active, code);
                    });
                }
                break;
            default:
                Logger_1.Logger.warn(`未知的助手动作: ${action}`);
        }
    }
    /**
     * 处理知识图谱消息
     */
    async handleGraphMessage(message, panel) {
        // TODO: 实现图谱消息处理
        Logger_1.Logger.info(`图谱消息: ${JSON.stringify(message)}`);
    }
    /**
     * 获取主面板 HTML 内容
     */
    getMainWebviewContent() {
        return this.getCommonHtml(`
            <div class="container">
                <header>
                    <h1>MindCortex</h1>
                    <p>智能知识管理助手</p>
                </header>
                <div class="dashboard">
                    <div class="card">
                        <h2>搜索</h2>
                        <p>语义搜索代码和文档</p>
                        <button onclick="sendMessage({type: 'openSearch'})">开始搜索</button>
                    </div>
                    <div class="card">
                        <h2>代码助手</h2>
                        <p>智能代码建议和片段</p>
                        <button onclick="sendMessage({type: 'openAssistant'})">打开助手</button>
                    </div>
                    <div class="card">
                        <h2>知识图谱</h2>
                        <p>可视化知识关联</p>
                        <button onclick="sendMessage({type: 'openGraph'})">查看图谱</button>
                    </div>
                </div>
            </div>
        `);
    }
    /**
     * 获取搜索面板 HTML 内容
     */
    getSearchWebviewContent() {
        return this.getCommonHtml(`
            <div class="search-container">
                <header>
                    <h1>知识库搜索</h1>
                </header>
                <div class="search-box">
                    <input
                        type="text"
                        id="searchInput"
                        placeholder="输入关键词或语义查询..."
                        onkeypress="if(event.key === 'Enter') performSearch()"
                    />
                    <button onclick="performSearch()">搜索</button>
                </div>
                <div class="search-filters">
                    <label>
                        <input type="checkbox" id="filterCode" checked />
                        代码
                    </label>
                    <label>
                        <input type="checkbox" id="filterDocs" checked />
                        文档
                    </label>
                    <label>
                        <input type="checkbox" id="filterSnippets" checked />
                        片段
                    </label>
                </div>
                <div id="searchResults" class="search-results"></div>
            </div>

            <script>
                function performSearch() {
                    const query = document.getElementById('searchInput').value;
                    const filters = {
                        code: document.getElementById('filterCode').checked,
                        docs: document.getElementById('filterDocs').checked,
                        snippets: document.getElementById('filterSnippets').checked
                    };

                    vscode.postMessage({
                        type: 'search',
                        query,
                        filters
                    });
                }
            </script>
        `);
    }
    /**
     * 获取代码助手面板 HTML 内容
     */
    getAssistantWebviewContent() {
        return this.getCommonHtml(`
            <div class="assistant-container">
                <header>
                    <h1>代码助手</h1>
                </header>
                <div class="assistant-tabs">
                    <button class="tab active" data-tab="snippets">代码片段</button>
                    <button class="tab" data-tab="bestPractices">最佳实践</button>
                    <button class="tab" data-tab="solutions">问题解决</button>
                </div>
                <div class="assistant-content">
                    <div id="snippets" class="tab-content active">
                        <input
                            type="text"
                            id="snippetSearch"
                            placeholder="搜索代码片段..."
                            onkeypress="if(event.key === 'Enter') searchSnippets()"
                        />
                        <button onclick="searchSnippets()">搜索</button>
                        <div id="snippetResults"></div>
                    </div>
                    <div id="bestPractices" class="tab-content">
                        <h2>最佳实践</h2>
                        <p>根据当前文件语言显示最佳实践建议</p>
                    </div>
                    <div id="solutions" class="tab-content">
                        <h2>问题解决</h2>
                        <p>常见问题和解决方案</p>
                    </div>
                </div>
            </div>

            <script>
                function searchSnippets() {
                    const query = document.getElementById('snippetSearch').value;
                    vscode.postMessage({
                        type: 'assistant',
                        action: 'searchSnippets',
                        query
                    });
                }
            </script>
        `);
    }
    /**
     * 获取知识图谱 HTML 内容
     */
    getGraphWebviewContent() {
        return this.getCommonHtml(`
            <div class="graph-container">
                <header>
                    <h1>知识图谱</h1>
                </header>
                <div class="graph-controls">
                    <input
                        type="text"
                        id="nodeSearch"
                        placeholder="搜索节点..."
                        onkeypress="if(event.key === 'Enter') searchNode()"
                    />
                    <button onclick="searchNode()">搜索</button>
                    <button onclick="resetView()">重置视图</button>
                </div>
                <div id="graphCanvas" class="graph-canvas"></div>
                <div id="nodeDetails" class="node-details"></div>
            </div>

            <script>
                // TODO: 集成 Cytoscape.js 或 D3.js
                function searchNode() {
                    const query = document.getElementById('nodeSearch').value;
                    vscode.postMessage({
                        type: 'graph',
                        action: 'searchNode',
                        query
                    });
                }

                function resetView() {
                    vscode.postMessage({
                        type: 'graph',
                        action: 'resetView'
                    });
                }
            </script>
        `);
    }
    /**
     * 获取通用 HTML 模板
     */
    getCommonHtml(bodyContent) {
        return `<!DOCTYPE html>
<html lang="zh-CN">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>MindCortex</title>
    <style>
        * {
            margin: 0;
            padding: 0;
            box-sizing: border-box;
        }

        body {
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, Cantarell, sans-serif;
            background: var(--vscode-editor-background);
            color: var(--vscode-editor-foreground);
            padding: 20px;
            max-width: 1200px;
            margin: 0 auto;
        }

        header {
            margin-bottom: 30px;
        }

        header h1 {
            font-size: 28px;
            margin-bottom: 8px;
            color: var(--vscode-textLink-foreground);
        }

        header p {
            font-size: 14px;
            color: var(--vscode-descriptionForeground);
        }

        .dashboard {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
            gap: 20px;
        }

        .card {
            background: var(--vscode-editor-inactiveSelectionBackground);
            border-radius: 8px;
            padding: 20px;
            transition: transform 0.2s;
        }

        .card:hover {
            transform: translateY(-2px);
        }

        .card h2 {
            font-size: 18px;
            margin-bottom: 12px;
        }

        .card p {
            font-size: 14px;
            color: var(--vscode-descriptionForeground);
            margin-bottom: 16px;
        }

        button {
            background: var(--vscode-button-background);
            color: var(--vscode-button-foreground);
            border: none;
            padding: 8px 16px;
            border-radius: 4px;
            cursor: pointer;
            font-size: 14px;
            transition: background 0.2s;
        }

        button:hover {
            background: var(--vscode-button-hoverBackground);
        }

        input[type="text"] {
            background: var(--vscode-input-background);
            color: var(--vscode-input-foreground);
            border: 1px solid var(--vscode-input-border);
            padding: 8px 12px;
            border-radius: 4px;
            font-size: 14px;
            width: 100%;
            margin-bottom: 10px;
        }

        .search-box {
            display: flex;
            gap: 10px;
            margin-bottom: 20px;
        }

        .search-box input {
            flex: 1;
            margin-bottom: 0;
        }

        .search-filters {
            display: flex;
            gap: 20px;
            margin-bottom: 20px;
        }

        .search-filters label {
            display: flex;
            align-items: center;
            gap: 8px;
            font-size: 14px;
        }

        .search-results {
            margin-top: 20px;
        }

        .assistant-tabs {
            display: flex;
            gap: 10px;
            margin-bottom: 20px;
        }

        .assistant-tabs .tab {
            background: var(--vscode-editor-inactiveSelectionBackground);
            padding: 10px 20px;
            border-radius: 4px;
            border: none;
        }

        .assistant-tabs .tab.active {
            background: var(--vscode-button-background);
            color: var(--vscode-button-foreground);
        }

        .tab-content {
            display: none;
        }

        .tab-content.active {
            display: block;
        }

        .graph-canvas {
            height: 500px;
            background: var(--vscode-editor-inactiveSelectionBackground);
            border-radius: 8px;
            margin-top: 20px;
        }

        .node-details {
            margin-top: 20px;
            padding: 15px;
            background: var(--vscode-editor-inactiveSelectionBackground);
            border-radius: 8px;
        }
    </style>
</head>
<body>
    ${bodyContent}
    <script>
        const vscode = acquireVsCodeApi();

        function sendMessage(message) {
            vscode.postMessage(message);
        }

        // 通知扩展已就绪
        vscode.postMessage({ type: 'ready' });
    </script>
</body>
</html>`;
    }
    /**
     * 获取 Webview 面板
     */
    getPanel(viewType) {
        return this.panels.get(viewType);
    }
    /**
     * 检查面板是否存在
     */
    hasPanel(viewType) {
        return this.panels.has(viewType);
    }
    /**
     * 关闭所有面板
     */
    closeAllPanels() {
        this.panels.forEach((panel, viewType) => {
            panel.dispose();
            Logger_1.Logger.info(`已关闭面板: ${viewType}`);
        });
        this.panels.clear();
    }
    /**
     * 释放资源
     */
    dispose() {
        this.closeAllPanels();
        Logger_1.Logger.info('所有 Webview 面板已释放');
    }
}
exports.WebviewManager = WebviewManager;
//# sourceMappingURL=WebviewManager.js.map