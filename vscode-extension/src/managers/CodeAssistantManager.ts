import * as vscode from 'vscode';
import axios from 'axios';
import { WebviewManager } from './WebviewManager';
import { SessionManager } from './SessionManager';
import { Logger } from '../utils/Logger';

export interface CodeSnippet {
    id: string;
    title: string;
    description: string;
    code: string;
    language: string;
    tags: string[];
    createdAt: number;
    usageCount: number;
}

export interface BestPractice {
    id: string;
    title: string;
    description: string;
    language: string;
    category: string;
    content: string;
    examples: string[];
}

export interface Solution {
    id: string;
    title: string;
    problem: string;
    solution: string;
    language: string;
    tags: string[];
    relatedSnippets: string[];
}

/**
 * 代码助手管理器
 * 负责代码片段查询、最佳实践、问题解决和代码推荐
 */
export class CodeAssistantManager {
    private context: vscode.ExtensionContext;
    private webviewManager: WebviewManager;
    private sessionManager: SessionManager;
    private apiEndpoint: string;
    private snippets: Map<string, CodeSnippet> = new Map();
    private readonly SNIPPETS_KEY = 'mindcortex.snippets';

    constructor(context: vscode.ExtensionContext, webviewManager: WebviewManager) {
        this.context = context;
        this.webviewManager = webviewManager;
        this.sessionManager = new SessionManager(context);
        this.apiEndpoint = this.getApiEndpoint();
    }

    /**
     * 获取 API 端点
     */
    private getApiEndpoint(): string {
        const config = vscode.workspace.getConfiguration('mindcortex');
        return config.get('apiEndpoint', 'http://localhost:3000');
    }

    /**
     * 打开代码助手面板
     */
    public openPanel(): void {
        const panel = this.webviewManager.openCodeAssistant();
        if (panel) {
            Logger.info('代码助手面板已打开');

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
    public async searchSnippets(query: string, language?: string): Promise<CodeSnippet[]> {
        try {
            Logger.info(`搜索代码片段: ${query} (${language || 'any'})`);

            const response = await axios.post(`${this.apiEndpoint}/api/assistant/snippets/search`, {
                query,
                language: language || this.getCurrentLanguage()
            }, {
                timeout: 5000
            });

            const snippets: CodeSnippet[] = response.data.snippets || [];

;

            return snippets;
        } catch (error) {
            Logger.error(`搜索片段失败: ${error}`);
            return this.getMockSnippets(query, language);
        }
    }

    /**
     * 添加代码片段
     */
    public async addSnippet(): Promise<void> {
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

        const snippet: CodeSnippet = {
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
        Logger.info(`已添加片段: ${title}`);
    }

    /**
     * 插入代码片段
     */
    public async insertSnippet(snippetId: string): Promise<void> {
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
        Logger.info(`已插入片段: ${snippet.title}`);
    }

    /**
     * 获取最佳实践
     */
    public async getBestPractices(language?: string): Promise<BestPractice[]> {
        try {
            const response = await axios.get(`${this.apiEndpoint}/api/assistant/best-practices`, {
                params: {
                    language: language || this.getCurrentLanguage()
                },
                timeout: 5000
            });

            return response.data.practices || [];
        } catch (error) {
            Logger.error(`获取最佳实践失败: ${error}`);
            return this.getMockBestPractices(language);
        }
    }

    /**
     * 搜索问题解决方案
     */
    public async searchSolutions(query: string, language?: string): Promise<Solution[]> {
        try {
            const response = await axios.post(`${this.apiEndpoint}/api/assistant/solutions/search`, {
                query,
                language: language || this.getCurrentLanguage()
            }, {
                timeout: 5000
            });

            return response.data.solutions || [];
        } catch (error) {
            Logger.error(`搜索解决方案失败: ${error}`);
            return this.getMockSolutions(query, language);
        }
    }

    /**
     * 获取代码推荐
     */
    public async getCodeRecommendations(context: string): Promise<CodeSnippet[]> {
        try {
            const response = await axios.post(`${this.apiEndpoint}/api/assistant/recommendations`, {
                context,
                language: this.getCurrentLanguage()
            }, {
                timeout: 5000
            });

            return response.data.recommendations || [];
        } catch (error) {
            Logger.error(`获取代码推荐失败: ${error}`);
            return [];
        }
    }

    /**
     * 分析当前代码
     */
    public async analyzeCurrentCode(): Promise<void> {
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
            const response = await axios.post(`${this.apiEndpoint}/api/assistant/analyze`, {
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
                analysis.suggestions.forEach((suggestion: { message: string; }) => {
                    channel.appendLine(`  - ${suggestion.message}`);
                });
            }

            if (analysis.issues && analysis.issues.length > 0) {
                channel.appendLine('\n⚠️  问题:');
                analysis.issues.forEach((issue: { message: string; severity: any; }) => {
                    const icon = issue.severity === 'error' ? '❌' : '⚠️';
                    channel.appendLine(`  ${icon} ${issue.message}`);
                });
            }

            if (analysis.bestPractices && analysis.bestPractices.length > 0) {
                channel.appendLine('\n✨ 最佳实践:');
                analysis.bestPractices.forEach((practice: any) => {
                    channel.appendLine(`  - ${practice.title}`);
                    channel.appendLine(`    ${practice.description}`);
                });
            }

            channel.show(true);

            Logger.info('代码分析完成');
        } catch (error) {
            Logger.error(`代码分析失败: ${error}`);
            vscode.window.showErrorMessage('代码分析失败，请检查 API 连接');
        }
    }

    /**
     * 获取当前文件语言
     */
    private getCurrentLanguage(): string | undefined {
        const editor = vscode.window.activeTextEditor;
        if (!editor) {
            return undefined;
        }

        return editor.document.languageId;
    }

    /**
     * 增加片段使用计数
     */
    private async incrementSnippetUsage(snippetId: string): Promise<void> {
        const snippet = this.snippets.get(snippetId);
        if (snippet) {
            snippet.usageCount++;
            await this.saveSnippets();
        }
    }

    /**
     * 保存片段
     */
    private async saveSnippets(): Promise<void> {
        const snippetsArray = Array.from(this.snippets.values());
        await this.context.globalState.update(
            this.SNIPPETS_KEY,
            snippetsArray
        );
    }

    /**
     * 加载片段
     */
    public async loadSnippets(): Promise<void> {
        const snippetsData = this.context.globalState.get<CodeSnippet[]>(
            this.SNIPPETS_KEY,
            []
        );

        this.snippets.clear();
        snippetsData.forEach(snippet => {
            this.snippets.set(snippet.id, snippet);
        });

        Logger.info(`已加载 ${this.snippets.size} 个代码片段`);
    }

    /**
     * 生成片段 ID
     */
    private generateSnippetId(): string {
        return `snippet_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`;
    }

    /**
     * 获取所有片段
     */
    public getAllSnippets(): CodeSnippet[] {
        return Array.from(this.snippets.values());
    }

    /**
     * 删除片段
     */
    public async deleteSnippet(snippetId: string): Promise<boolean> {
        const snippet = this.snippets.get(snippetId);
        if (!snippet) {
            return false;
        }

        this.snippets.delete(snippetId);
        await this.saveSnippets();

        vscode.window.showInformationMessage(`已删除片段: ${snippet.title}`);
        Logger.info(`已删除片段: ${snippet.title}`);

        return true;
    }

    /**
     * 管理片段
     */
    public async manageSnippets(): Promise<void> {
        const options: vscode.QuickPickItem[] = [
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
    private async promptSearchSnippets(): Promise<void> {
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

        const items: vscode.QuickPickItem[] = snippets.map(snippet => ({
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
    private async listSnippets(): Promise<void> {
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
    private async promptDeleteSnippet(): Promise<void> {
        const snippets = this.getAllSnippets();

        if (snippets.length === 0) {
            vscode.window.showInformationMessage('没有可删除的代码片段');
            return;
        }

        const items: vscode.QuickPickItem[] = snippets.map(snippet => ({
            label: snippet.title,
            description: snippet.description || snippet.language
        }));

        const selected = await vscode.window.showQuickPick(items, {
            placeHolder: '选择要删除的片段'
        });

        if (!selected) {
            return;
        }

        const confirm = await vscode.window.showWarningMessage(
            `确定要删除片段 "${selected.label}" 吗？`,
            '删除',
            '取消'
            );

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
    private getMockSnippets(query: string, language?: string): CodeSnippet[] {
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
    private getMockBestPractices(language?: string): BestPractice[] {
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
    private getMockSolutions(query: string, language?: string): Solution[] {
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
    public dispose(): void {
        this.snippets.clear();
        Logger.info('代码助手管理器已释放');
    }
}
