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
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
exports.SearchManager = void 0;
const vscode = __importStar(require("vscode"));
const axios_1 = __importDefault(require("axios"));
const SessionManager_1 = require("./SessionManager");
const Logger_1 = require("../utils/Logger");
/**
 * 搜索管理器
 * 负责实现搜索功能，包括快捷键搜索、右键菜单和搜索面板
 */
class SearchManager {
    constructor(context, webviewManager) {
        this.searchHistory = [];
        this.MAX_HISTORY = 50;
        this.context = context;
        this.webviewManager = webviewManager;
        this.sessionManager = new SessionManager_1.SessionManager(context); // 临时创建，后续通过依赖注入优化
        this.apiEndpoint = this.getApiEndpoint();
    }
    /**
     * 获取 API 端点
     */
    getApiEndpoint() {
        const config = vscode.workspace.getConfiguration('mindcortex');
        return config.get('apiEndpoint', 'http://localhost:3000');
    }
    /**
     * 显示搜索面板
     */
    showSearchPanel() {
        const panel = this.webviewManager.openSearchPanel();
        if (panel) {
            Logger_1.Logger.info('搜索面板已打开');
        }
    }
    /**
     * 搜索选中文本
     */
    async searchSelected() {
        const editor = vscode.window.activeTextEditor;
        if (!editor) {
            vscode.window.showWarningMessage('没有选中文本');
            return;
        }
        const selection = editor.selection;
        const selectedText = editor.document.getText(selection);
        if (!selectedText.trim()) {
            vscode.window.showWarningMessage('请先选择要搜索的文本');
            return;
        }
        Logger_1.Logger.info(`搜索选中文本: ${selectedText}`);
        // 打开搜索面板并执行搜索
        const panel = this.webviewManager.openSearchPanel();
        if (panel) {
            // 发送搜索查询到 webview
            panel.webview.postMessage({
                type: 'initSearch',
                query: selectedText
            });
            // 执行搜索
            const results = await this.executeSearch(selectedText);
            // 显示结果
            this.displaySearchResults(results, panel);
        }
    }
    /**
     * 执行搜索
     */
    async executeSearch(query, options) {
        try {
            // 保存到搜索历史
            this.addToHistory(query);
            const searchOptions = {
                query,
                filters: {
                    code: true,
                    docs: true,
                    snippets: true
                },
                maxResults: 20,
                ...options
            };
            Logger_1.Logger.info(`执行搜索: ${query}`);
            // 调用后端 API
            const response = await axios_1.default.post(`${this.apiEndpoint}/api/search`, searchOptions, {
                timeout: 10000
            });
            const results = response.data.results || [];
            Logger_1.Logger.info(`搜索完成，找到 ${results.length} 个结果`);
            return results;
        }
        catch (error) {
            Logger_1.Logger.error(`搜索失败: ${error}`);
            // 如果 API 调用失败，返回模拟结果
            return this.getMockResults(query, options?.maxResults);
        }
    }
    /**
     * 显示搜索结果
     */
    displaySearchResults(results, panel) {
        if (!panel) {
            panel = this.webviewManager.getPanel('mindcortex.search');
        }
        if (panel) {
            panel.webview.postMessage({
                type: 'searchResults',
                results: results,
                query: this.searchHistory[0]
            });
        }
        else {
            // 在输出通道显示结果
            const channel = vscode.window.createOutputChannel('MindCortex 搜索结果');
            channel.clear();
            channel.appendLine(`搜索结果 (${results.length} 个):`);
            channel.appendLine(''.padEnd(60, '-'));
            results.forEach((result, index) => {
                channel.appendLine(`\n${index + 1}. ${result.title}`);
                channel.appendLine(`   类型: ${result.type}`);
                channel.appendLine(`   评分: ${result.score.toFixed(3)}`);
                if (result.filePath) {
                    channel.appendLine(`   文件: ${result.filePath}`);
                }
                channel.appendLine(`   内容: ${result.content.substring(0, 100)}...`);
            });
            channel.show(true);
        }
    }
    /**
     * 添加到搜索历史
     */
    addToHistory(query) {
        // 移除重复项
        this.searchHistory = this.searchHistory.filter(q => q !== query);
        // 添加到开头
        this.searchHistory.unshift(query);
        // 限制历史长度
        if (this.searchHistory.length > this.MAX_HISTORY) {
            this.searchHistory = this.searchHistory.slice(0, this.MAX_HISTORY);
        }
    }
    /**
     * 获取搜索历史
     */
    getSearchHistory() {
        return [...this.searchHistory];
    }
    /**
     * 清除搜索历史
     */
    clearSearchHistory() {
        this.searchHistory = [];
        Logger_1.Logger.info('搜索历史已清除');
    }
    /**
     * 刷新索引
     */
    async refreshIndex() {
        try {
            Logger_1.Logger.info('正在刷新索引...');
            const response = await axios_1.default.post(`${this.apiEndpoint}/api/index/refresh`, {
                timeout: 60000
            });
            if (response.data.success) {
                const count = response.data.count || 0;
                vscode.window.showInformationMessage(`索引刷新成功，已索引 ${count} 个文件`);
                Logger_1.Logger.info(`索引刷新成功: ${count} 个文件`);
            }
            else {
                vscode.window.showWarningMessage('索引刷新完成，但可能存在问题');
            }
        }
        catch (error) {
            Logger_1.Logger.error(`刷新索引失败: ${error}`);
            vscode.window.showErrorMessage('刷新索引失败，请检查 API 连接');
        }
    }
    /**
     * 索引单个文档
     */
    async indexDocument(uri) {
        try {
            const document = await vscode.workspace.openTextDocument(uri);
            const content = document.getText();
            await axios_1.default.post(`${this.apiEndpoint}/api/index/document`, {
                filePath: uri.fsPath,
                content: content,
                language: document.languageId
            }, {
                timeout: 5000
            });
            Logger_1.Logger.info(`文档已索引: ${uri.fsPath}`);
        }
        catch (error) {
            Logger_1.Logger.error(`索引文档失败 [${uri.fsPath}]: ${error}`);
        }
    }
    /**
     * 快速搜索（通过输入框）
     */
    async quickSearch() {
        const query = await vscode.window.showInputBox({
            prompt: '搜索知识库',
            placeHolder: '输入关键词或语义查询...',
            ignoreFocusOut: true
        });
        if (!query) {
            return;
        }
        const results = await this.executeSearch(query);
        if (results.length === 0) {
            vscode.window.showInformationMessage('未找到相关结果');
            return;
        }
        // 显示结果选择器
        const items = results.slice(0, 10).map(result => ({
            label: result.title,
            description: `${result.type} - 评分: ${result.score.toFixed(3)}`,
            detail: result.content.substring(0, 100)
        }));
        const selected = await vscode.window.showQuickPick(items, {
            placeHolder: `找到 ${results.length} 个结果`,
            ignoreFocusOut: true
        });
        if (selected) {
            // 打开对应的文档或显示详情
            const result = results.find(r => r.title === selected.label);
            if (result && result.filePath) {
                await vscode.commands.executeCommand('vscode.open', vscode.Uri.file(result.filePath));
            }
        }
    }
    /**
     * 获取搜索建议
     */
    async getSearchSuggestions(prefix) {
        try {
            const response = await axios_1.default.get(`${this.apiEndpoint}/api/search/suggestions`, {
                params: { prefix },
                timeout: 3000
            });
            return response.data.suggestions || [];
        }
        catch (error) {
            Logger_1.Logger.error(`获取搜索建议失败: ${error}`);
            return [];
        }
    }
    /**
     * 获取模拟搜索结果（用于测试和降级）
     */
    getMockResults(query, maxResults = 20) {
        const mockResults = [
            {
                id: 'mock_1',
                title: '示例代码片段',
                content: '这是一个示例代码片段，用于演示搜索功能。',
                type: 'code',
                language: 'typescript',
                score: 0.85,
                metadata: {
                    source: 'mock'
                }
            },
            {
                id: 'mock_2',
                title: 'API 文档',
                content: '这是 API 文档的示例内容，包含详细的接口说明。',
                type: 'doc',
                score: 0.78,
                metadata: {
                    source: 'mock'
                }
            },
            {
                id: 'mock_3',
                title: '最佳实践',
                content: '这是最佳实践建议，帮助开发者写出更好的代码。',
                type: 'snippet',
                language: 'javascript',
                score: 0.72,
                metadata: {
                    source: 'mock'
                }
            }
        ];
        return mockResults.slice(0, maxResults);
    }
    /**
     * 搜索统计信息
     */
    getStatistics() {
        return {
            totalSearches: this.searchHistory.length,
            lastSearch: this.searchHistory[0]
        };
    }
    /**
     * 清除资源
     */
    dispose() {
        this.searchHistory = [];
        Logger_1.Logger.info('搜索管理器已释放');
    }
}
exports.SearchManager = SearchManager;
//# sourceMappingURL=SearchManager.js.map