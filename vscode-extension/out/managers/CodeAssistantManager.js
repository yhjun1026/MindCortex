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
exports.CodeAssistantManager = void 0;
const vscode = __importStar(require("vscode"));
const axios_1 = __importDefault(require("axios"));
const SessionManager_1 = require("./SessionManager");
const Logger_1 = require("../utils/Logger");
/**
 * 代码助手管理器
 * 负责代码片段查询、最佳实践、问题解决和代码推荐
 */
class CodeAssistantManager {
    constructor(context, webviewManager) {
        this.snippets = new Map();
        this.SNIPPETS_KEY = 'mindcortex.snippets';
        this.context = context;
        this.webviewManager = webviewManager;
        this.sessionManager = new SessionManager_1.SessionManager(context);
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
     * 打开代码助手面板
     */
    openPanel() {
        const panel = this.webviewManager.openCodeAssistant();
        if (panel) {
            Logger_1.Logger.info('代码助手面板已打开');
            // 发送当前文件语言信息
            const language = this.getCurrentLanguage();
            panel.webview.postMessage({
                type: 'languageChanged',
                language
            });
        }
    }
    /**
     * 搜索代码片段
     */
    async searchSnippets(query, language) {
        try {
            Logger_1.Logger.info(`搜索代码片段: ${query} (${language || 'any'})`);
            const response = await axios_1.default.post(`${this.apiEndpoint}/api/assistant/snippets/search`, {
                query,
                language: language || this.getCurrentLanguage()
            }, {
                timeout: 5000
            });
            const snippets = response.data.snippets || [];
            ;
            return snippets;
        }
        catch (error) {
            Logger_1.Logger.error(`搜索片段失败: ${error}`);
            return this.getMockSnippets(query, language);
        }
    }
    /**
     * 添加代码片段
     */
    async addSnippet() {
        const editor = vscode.window.activeTextEditor;
        if (!editor) {
            vscode.window.showWarningMessage('没有活动的编辑器');
            return;
        }
        const selectedText = editor.document.getText(editor.selection);
        if (!selectedText.trim()) {
            vscode.window.showWarningMessage('请先选择要保存为片段的代码');
            return;
        }
        // 输入片段标题
        const title = await vscode.window.showInputBox({
            prompt: '输入片段标题',
            placeHolder: '我的代码片段'
        });
        if (!title) {
            return;
        }
        // 输入片段描述
        const description = await vscode.window.showInputBox({
            prompt: '输入片段描述（可选）',
            placeHolder: '片段功能说明'
        });
        // 输入标签（逗号分隔）
        const tagsInput = await vscode.window.showInputBox({
            prompt: '输入标签（逗号分隔）',
            placeHolder: 'tag1, tag2, tag3'
        });
        const tags = tagsInput
            ? tagsInput.split(',').map(tag => tag.trim()).filter(tag => tag)
            : [];
        const snippet = {
            id: this.generateSnippetId(),
            title,
            description: description || '',
            code: selectedText,
            language: editor.document.languageId,
            tags,
            createdAt: Date.now(),
            usageCount: 0
        };
        this.snippets.set(snippet.id, snippet);
        await this.saveSnippets();
        vscode.window.showInformationMessage(`已添加片段: ${title}`);
        Logger_1.Logger.info(`已添加片段: ${title}`);
    }
    /**
     * 插入代码片段
     */
    async insertSnippet(snippetId) {
        const snippet = this.snippets.get(snippetId);
        if (!snippet) {
            vscode.window.showErrorMessage('片段不存在');
            return;
        }
        const editor = vscode.window.activeTextEditor;
        if (!editor) {
            vscode.window.showErrorMessage('没有活动的编辑器');
            return;
        }
        await editor.edit(editBuilder => {
            editBuilder.insert(editor.selection.active, snippet.code);
        });
        // 更新使用计数
        snippet.usageCount++;
        await this.saveSnippets();
        vscode.window.showInformationMessage(`已插入片段: ${snippet.title}`);
        Logger_1.Logger.info(`已插入片段: ${snippet.title}`);
    }
    /**
     * 获取最佳实践
     */
    async getBestPractices(language) {
        try {
            const response = await axios_1.default.get(`${this.apiEndpoint}/api/assistant/best-practices`, {
                params: {
                    language: language || this.getCurrentLanguage()
                },
                timeout: 5000
            });
            return response.data.practices || [];
        }
        catch (error) {
            Logger_1.Logger.error(`获取最佳实践失败: ${error}`);
            return this.getMockBestPractices(language);
        }
    }
    /**
     * 搜索问题解决方案
     */
    async searchSolutions(query, language) {
        try {
            const response = await axios_1.default.post(`${this.apiEndpoint}/api/assistant/solutions/search`, {
                query,
                language: language || this.getCurrentLanguage()
            }, {
                timeout: 5000
            });
            return response.data.solutions || [];
        }
        catch (error) {
            Logger_1.Logger.error(`搜索解决方案失败: ${error}`);
            return this.getMockSolutions(query, language);
        }
    }
    /**
     * 获取代码推荐
     */
    async getCodeRecommendations(context) {
        try {
            const response = await axios_1.default.post(`${this.apiEndpoint}/api/assistant/recommendations`, {
                context,
                language: this.getCurrentLanguage()
            }, {
                timeout: 5000
            });
            return response.data.recommendations || [];
        }
        catch (error) {
            Logger_1.Logger.error(`获取代码推荐失败: ${error}`);
            return [];
        }
    }
    /**
     * 分析当前代码
     */
    async analyzeCurrentCode() {
        const editor = vscode.window.activeTextEditor;
        if (!editor) {
            vscode.window.showWarningMessage('没有活动的编辑器');
            return;
        }
        const code = editor.document.getText(editor.selection) || editor.document.getText();
        if (!code.trim()) {
            vscode.window.showWarningMessage('没有代码可分析');
            return;
        }
        try {
            const response = await axios_1.default.post(`${this.apiEndpoint}/api/assistant/analyze`, {
                code,
                language: editor.document.languageId
            }, {
                timeout: 10000
            });
            const analysis = response.data;
            // 显示分析结果
            const channel = vscode.window.createOutputChannel('代码分析结果');
            channel.clear();
            channel.appendLine('代码分析结果:');
            channel.appendLine(''.padEnd(60, '-'));
            if (analysis.suggestions && analysis.suggestions.length > 0) {
                channel.appendLine('\n💡 建议:');
                analysis.suggestions.forEach((suggestion) => {
                    channel.appendLine(`  - ${suggestion.message}`);
                });
            }
            if (analysis.issues && analysis.issues.length > 0) {
                channel.appendLine('\n⚠️  问题:');
                analysis.issues.forEach((issue) => {
                    const icon = issue.severity === 'error' ? '❌' : '⚠️';
                    channel.appendLine(`  ${icon} ${issue.message}`);
                });
            }
            if (analysis.bestPractices && analysis.bestPractices.length > 0) {
                channel.appendLine('\n✨ 最佳实践:');
                analysis.bestPractices.forEach((practice) => {
                    channel.appendLine(`  - ${practice.title}`);
                    channel.appendLine(`    ${practice.description}`);
                });
            }
            channel.show(true);
            Logger_1.Logger.info('代码分析完成');
        }
        catch (error) {
            Logger_1.Logger.error(`代码分析失败: ${error}`);
            vscode.window.showErrorMessage('代码分析失败，请检查 API 连接');
        }
    }
    /**
     * 获取当前文件语言
     */
    getCurrentLanguage() {
        const editor = vscode.window.activeTextEditor;
        if (!editor) {
            return undefined;
        }
        return editor.document.languageId;
    }
    /**
     * 增加片段使用计数
     */
    async incrementSnippetUsage(snippetId) {
        const snippet = this.snippets.get(snippetId);
        if (snippet) {
            snippet.usageCount++;
            await this.saveSnippets();
        }
    }
    /**
     * 保存片段
     */
    async saveSnippets() {
        const snippetsArray = Array.from(this.snippets.values());
        await this.context.globalState.update(this.SNIPPETS_KEY, snippetsArray);
    }
    /**
     * 加载片段
     */
    async loadSnippets() {
        const snippetsData = this.context.globalState.get(this.SNIPPETS_KEY, []);
        this.snippets.clear();
        snippetsData.forEach(snippet => {
            this.snippets.set(snippet.id, snippet);
        });
        Logger_1.Logger.info(`已加载 ${this.snippets.size} 个代码片段`);
    }
    /**
     * 生成片段 ID
     */
    generateSnippetId() {
        return `snippet_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`;
    }
    /**
     * 获取所有片段
     */
    getAllSnippets() {
        return Array.from(this.snippets.values());
    }
    /**
     * 删除片段
     */
    async deleteSnippet(snippetId) {
        const snippet = this.snippets.get(snippetId);
        if (!snippet) {
            return false;
        }
        this.snippets.delete(snippetId);
        await this.saveSnippets();
        vscode.window.showInformationMessage(`已删除片段: ${snippet.title}`);
        Logger_1.Logger.info(`已删除片段: ${snippet.title}`);
        return true;
    }
    /**
     * 管理片段
     */
    async manageSnippets() {
        const options = [
            { label: '$(plus) 添加新片段', description: '从选中的代码创建片段' },
            { label: '$(search) 搜索片段', description: '搜索现有片段' },
            { label: '$(list-ordered) 列出所有片段', description: '查看所有代码片段' },
            { label: '$(trash) 删除片段', description: '删除现有片段' }
        ];
        const selected = await vscode.window.showQuickPick(options, {
            placeHolder: '代码片段管理',
            ignoreFocusOut: true
        });
        if (!selected) {
            return;
        }
        switch (selected.label) {
            case '$(plus) 添加新片段':
                await this.addSnippet();
                break;
            case '$(search) 搜索片段':
                await this.promptSearchSnippets();
                break;
            case '$(list-ordered) 列出所有片段':
                await this.listSnippets();
                break;
            case '$(trash) 删除片段':
                await this.promptDeleteSnippet();
                break;
        }
    }
    /**
     * 提示搜索片段
     */
    async promptSearchSnippets() {
        const query = await vscode.window.showInputBox({
            prompt: '搜索代码片段',
            placeHolder: '输入关键词...'
        });
        if (!query) {
            return;
        }
        const snippets = await this.searchSnippets(query);
        if (snippets.length === 0) {
            vscode.window.showInformationMessage('未找到匹配的片段');
            return;
        }
        const items = snippets.map(snippet => ({
            label: snippet.title,
            description: snippet.description || snippet.language,
            detail: `${snippet.tags.join(', ')} (${snippet.usageCount} 次使用)`
        }));
        const selected = await vscode.window.showQuickPick(items, {
            placeHolder: `找到 ${snippets.length} 个片段`
        });
        if (selected) {
            const snippet = snippets.find(s => s.title === selected.label);
            if (snippet) {
                await this.insertSnippet(snippet.id);
            }
        }
    }
    /**
     * 列出所有片段
     */
    async listSnippets() {
        const snippets = this.getAllSnippets();
        if (snippets.length === 0) {
            vscode.window.showInformationMessage('没有保存的代码片段');
            return;
        }
        const channel = vscode.window.createOutputChannel('代码片段');
        channel.clear();
        channel.appendLine(`代码片段列表 (${snippets.length} 个):`);
        channel.appendLine(''.padEnd(60, '-'));
        snippets.forEach((snippet, index) => {
            channel.appendLine(`\n${index + 1}. ${snippet.title}`);
            channel.appendLine(`   语言: ${snippet.language}`);
            channel.appendLine(`   标签: ${snippet.tags.join(', ') || '无'}`);
            channel.appendLine(`   使用次数: ${snippet.usageCount}`);
            if (snippet.description) {
                channel.appendLine(`   描述: ${snippet.description}`);
            }
        });
        channel.show(true);
    }
    /**
     * 提示删除片段
     */
    async promptDeleteSnippet() {
        const snippets = this.getAllSnippets();
        if (snippets.length === 0) {
            vscode.window.showInformationMessage('没有可删除的代码片段');
            return;
        }
        const items = snippets.map(snippet => ({
            label: snippet.title,
            description: snippet.description || snippet.language
        }));
        const selected = await vscode.window.showQuickPick(items, {
            placeHolder: '选择要删除的片段'
        });
        if (!selected) {
            return;
        }
        const confirm = await vscode.window.showWarningMessage(`确定要删除片段 "${selected.label}" 吗？`, '删除', '取消');
        if (confirm === '删除') {
            const snippet = snippets.find(s => s.title === selected.label);
            if (snippet) {
                await this.deleteSnippet(snippet.id);
            }
        }
    }
    /**
     * 获取模拟片段数据
     */
    getMockSnippets(query, language) {
        return [
            {
                id: 'mock_snippet_1',
                title: '初始化 React 组件',
                description: '标准的 React 函数组件初始化模板',
                code: `import React from 'react';

interface Props {
    // 定义 props
}

const Component: React.FC<Props> = ({}) => {
    return (
        <div>
            {/* 组件内容 */}
        </div>
    );
};

export default Component;`,
                language: language || 'typescript',
                tags: ['react', 'component', 'template'],
                createdAt: Date.now(),
                usageCount: 0
            }
        ];
    }
    /**
     * 获取模拟最佳实践
     */
    getMockBestPractices(language) {
        return [
            {
                id: 'mock_practice_1',
                title: '使用 TypeScript 类型',
                description: '为所有组件和函数定义明确的类型',
                language: language || 'typescript',
                category: 'Type Safety',
                content: '使用 TypeScript 可以在编译时捕获类型错误，提高代码质量。',
                examples: ['interface Props {}', 'const Component: React.FC<Props> = ({}) => {}']
            }
        ];
    }
    /**
     * 获取模拟解决方案
     */
    getMockSolutions(query, language) {
        return [
            {
                id: 'mock_solution_1',
                title: '处理异步状态',
                problem: '如何在 React 中正确处理异步操作的状态',
                solution: '使用 useEffect 和 useState 来管理异步操作的状态',
                language: language || 'typescript',
                tags: ['async', 'react', 'state'],
                relatedSnippets: []
            }
        ];
    }
    /**
     * 清除资源
     */
    dispose() {
        this.snippets.clear();
        Logger_1.Logger.info('代码助手管理器已释放');
    }
}
exports.CodeAssistantManager = CodeAssistantManager;
//# sourceMappingURL=CodeAssistantManager.js.map