import * as vscode from 'vscode';
import { StatusBarManager } from './managers/StatusBarManager';
import { CommandManager } from './managers/CommandManager';
import { WebviewManager } from './managers/WebviewManager';
import { SessionManager } from './managers/SessionManager';
import { SearchManager } from './managers/SearchManager';
import { CodeAssistantManager } from './managers/CodeAssistantManager';
import { Logger } from './utils/Logger';

let extensionContext: vscode.ExtensionContext;
let statusBarManager: StatusBarManager;
let commandManager: CommandManager;
let webviewManager: WebviewManager;
let sessionManager: SessionManager;
let searchManager: SearchManager;
let codeAssistantManager: CodeAssistantManager;

/**
 * 插件激活入口
 */
export async function activate(context: vscode.ExtensionContext) {
    extensionContext = context;

    Logger.info('MindCortex VSCode 扩展正在激活...');

    try {
        // 初始化管理器
        statusBarManager = new StatusBarManager(context);
        commandManager = new CommandManager(context);
        webviewManager = new WebviewManager(context);
        sessionManager = new SessionManager(context);
        searchManager = new SearchManager(context, webviewManager);
        codeAssistantManager = new CodeAssistantManager(context, webviewManager);

        // 注册命令
        registerCommands();

        // 创建状态栏
        statusBarManager.createStatusBar();

        // 加载会话
        await sessionManager.loadSessions();

        Logger.info('MindCortex VSCode 扩展激活成功');
        vscode.window.showInformationMessage('MindCortex 插件已激活');

        // 监听文档变化，自动索引
        if (vscode.workspace.getConfiguration('mindcortex').get('autoIndex', true)) {
            setupDocumentWatcher();
        }

    } catch (error) {
        Logger.error(`插件激活失败: ${error}`);
        vscode.window.showErrorMessage(`MindCortex 插件激活失败: ${error}`);
    }
}

/**
 * 注册所有命令
 */
function registerCommands() {
    const commands = [
        {
            id: 'mindcortex.openMainPanel',
            handler: () => webviewManager.openMainPanel()
        },
        {
            id: 'mindcortex.search',
            handler: () => searchManager.showSearchPanel()
        },
        {
            id: 'mindcortex.searchSelected',
            handler: () => searchManager.searchSelected()
        },
        {
            id: 'mindcortex.openCodeAssistant',
            handler: () => codeAssistantManager.openPanel()
        },
        {
            id: 'mindcortex.openKnowledgeGraph',
            handler: () => webviewManager.openKnowledgeGraph()
        },
        {
            id: 'mindcortex.addSnippet',
            handler:() => codeAssistantManager.addSnippet()
        },
        {
            id: 'mindcortex.manageSessions',
            handler: () => sessionManager.manageSessions()
        },
        {
            id: 'mindcortex.refreshIndex',
            handler: () => searchManager.refreshIndex()
        }
    ];

    commands.forEach(cmd => {
        extensionContext.subscriptions.push(
            vscode.commands.registerCommand(cmd.id, cmd.handler)
        );
    });

    Logger.info(`已注册 ${commands.length} 个命令`);
}

/**
 * 设置文档监听器
 */
function setupDocumentWatcher() {
    const watcher = vscode.workspace.createFileSystemWatcher(
        '**/*.{ts,js,tsx,jsx,py,java,go,rs,cpp,c,h}'
    );

    watcher.onDidChange(async (uri) => {
        Logger.info(`文档已修改: ${uri.path}`);
        await searchManager.indexDocument(uri);
    });

    watcher.onDidCreate(async (uri) => {
        Logger.info(`文档已创建: ${uri.path}`);
        await searchManager.indexDocument(uri);
    });

    extensionContext.subscriptions.push(watcher);
    Logger.info('文档监听器已启动');
}

/**
 * 插件停用
 */
export function deactivate() {
    Logger.info('MindCortex VSCode 扩展正在停用...');

    // 清理资源
    if (statusBarManager) {
        statusBarManager.dispose();
    }

    if (webviewManager) {
        webviewManager.dispose();
    }

    Logger.info('MindCortex VSCode 扩展已停用');
}

/**
 * 获取扩展上下文
 */
export function getExtensionContext(): vscode.ExtensionContext {
    return extensionContext;
}

/**
 * 获取管理器实例
 */
export function getManagers() {
    return {
        statusBar: statusBarManager,
        webview: webviewManager,
        session: sessionManager,
        search: searchManager,
        codeAssistant: codeAssistantManager
    };
}
