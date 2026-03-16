import * as vscode from 'vscode';
import axios from 'axios';
import { WebviewManager } from './WebviewManager';
import { SessionManager } from './SessionManager';
import { Logger } from '../utils/Logger';

export interface SearchResult {
    id: string;
    title: string;
    content: string;
    type: 'code' | 'doc' | 'snippet';
    language?: string;
    filePath?: string;
    score: number;
    metadata?: Record<string, any>;
}

export interface SearchOptions {
    query: string;
    filters?: {
        code?: boolean;
        docs?: boolean;
        snippets?: boolean;
    };
    maxResults?: number;
}

/**
 * 搜索管理器
 * 负责实现搜索功能，包括快捷键搜索、右键菜单和搜索面板
 */
export class SearchManager {
    private context: vscode.ExtensionContext;
    private webviewManager: WebviewManager;
    private sessionManager: SessionManager;
    private apiEndpoint: string;
    private searchHistory: string[] = [];
    private readonly MAX_HISTORY = 50;

    constructor(context: vscode.ExtensionContext, webviewManager: WebviewManager) {
        this.context = context;
        this.webviewManager = webviewManager;
        this.sessionManager = new SessionManager(context); // 临时创建，后续通过依赖注入优化
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
     * 显示搜索面板
     */
    public showSearchPanel(): void {
        const panel = this.webviewManager.openSearchPanel();
        if (panel) {
            Logger.info('搜索面板已打开');
        }
    }

    /**
     * 搜索选中文本
     */
    public async searchSelected(): Promise<void> {
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

        Logger.info(`搜索选中文本: ${selectedText}`);

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
    public async executeSearch(query: string, options?: Partial<SearchOptions>): Promise<SearchResult[]> {
        try {
            // 保存到搜索历史
            this.addToHistory(query);

            const searchOptions: SearchOptions = {
                query,
                filters: {
                    code: true,
                    docs: true,
                    snippets: true
                },
                maxResults: 20,
                ...options
            };

            Logger.info(`执行搜索: ${query}`);

            // 调用后端 API
            const response = await axios.post(`${this.apiEndpoint}/api/search`, searchOptions, {
                timeout: 10000
            });

            const results: SearchResult[] = response.data.results || [];

            Logger.info(`搜索完成，找到 ${results.length} 个结果`);

            return results;
        } catch (error) {
            Logger.error(`搜索失败: ${error}`);

            // 如果 API 调用失败，返回模拟结果
            return this.getMockResults(query, options?.maxResults);
        }
    }

    /**
     * 显示搜索结果
     */
    public displaySearchResults(results: SearchResult[], panel?: vscode.WebviewPanel): void {
        if (!panel) {
            panel = this.webviewManager.getPanel('mindcortex.search');
        }

        if (panel) {
            panel.webview.postMessage({
                type: 'searchResults',
                results: results,
                query: this.searchHistory[0]
            });
        } else {
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
    private addToHistory(query: string): void {
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
    public getSearchHistory(): string[] {
        return [...this.searchHistory];
    }

    /**
     * 清除搜索历史
     */
    public clearSearchHistory(): void {
        this.searchHistory = [];
        Logger.info('搜索历史已清除');
    }

    /**
     * 刷新索引
     */
    public async refreshIndex(): Promise<void> {
        try {
            Logger.info('正在刷新索引...');

            const response = await axios.post(`${this.apiEndpoint}/api/index/refresh`, {
                timeout: 60000
            });

            if (response.data.success) {
                const count = response.data.count || 0;
                vscode.window.showInformationMessage(`索引刷新成功，已索引 ${count} 个文件`);
                Logger.info(`索引刷新成功: ${count} 个文件`);
            } else {
                vscode.window.showWarningMessage('索引刷新完成，但可能存在问题');
            }
        } catch (error) {
            Logger.error(`刷新索引失败: ${error}`);
            vscode.window.showErrorMessage('刷新索引失败，请检查 API 连接');
        }
    }

    /**
     * 索引单个文档
     */
    public async indexDocument(uri: vscode.Uri): Promise<void> {
        try {
            const document = await vscode.workspace.openTextDocument(uri);
            const content = document.getText();

            await axios.post(`${this.apiEndpoint}/api/index/document`, {
                filePath: uri.fsPath,
                content: content,
                language: document.languageId
            }, {
            timeout: 5000
            });

            Logger.info(`文档已索引: ${uri.fsPath}`);
        } catch (error) {
            Logger.error(`索引文档失败 [${uri.fsPath}]: ${error}`);
        }
    }

    /**
     * 快速搜索（通过输入框）
     */
    public async quickSearch(): Promise<void> {
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
        const items: vscode.QuickPickItem[] = results.slice(0, 10).map(result => ({
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
    public async getSearchSuggestions(prefix: string): Promise<string[]> {
        try {
            const response = await axios.get(`${this.apiEndpoint}/api/search/suggestions`, {
                params: { prefix },
                timeout: 3000
            });

            return response.data.suggestions || [];
        } catch (error) {
            Logger.error(`获取搜索建议失败: ${error}`);
            return [];
        }
    }

    /**
     * 获取模拟搜索结果（用于测试和降级）
     */
    private getMockResults(query: string, maxResults: number = 20): SearchResult[] {
        const mockResults: SearchResult[] = [
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
    public getStatistics(): { totalSearches: number; lastSearch: string | undefined; } {
        return {
            totalSearches: this.searchHistory.length,
            lastSearch: this.searchHistory[0]
        };
    }

    /**
     * 清除资源
     */
    public dispose(): void {
        this.searchHistory = [];
        Logger.info('搜索管理器已释放');
    }
}
