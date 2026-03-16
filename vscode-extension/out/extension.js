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
exports.activate = activate;
exports.deactivate = deactivate;
exports.getExtensionContext = getExtensionContext;
exports.getManagers = getManagers;
const vscode = __importStar(require("vscode"));
const StatusBarManager_1 = require("./managers/StatusBarManager");
const CommandManager_1 = require("./managers/CommandManager");
const WebviewManager_1 = require("./managers/WebviewManager");
const SessionManager_1 = require("./managers/SessionManager");
const SearchManager_1 = require("./managers/SearchManager");
const CodeAssistantManager_1 = require("./managers/CodeAssistantManager");
const Logger_1 = require("./utils/Logger");
let extensionContext;
let statusBarManager;
let commandManager;
let webviewManager;
let sessionManager;
let searchManager;
let codeAssistantManager;
/**
 * 插件激活入口
 */
async function activate(context) {
    extensionContext = context;
    Logger_1.Logger.info('MindCortex VSCode 扩展正在激活...');
    try {
        // 初始化管理器
        statusBarManager = new StatusBarManager_1.StatusBarManager(context);
        commandManager = new CommandManager_1.CommandManager(context);
        webviewManager = new WebviewManager_1.WebviewManager(context);
        sessionManager = new SessionManager_1.SessionManager(context);
        searchManager = new SearchManager_1.SearchManager(context, webviewManager);
        codeAssistantManager = new CodeAssistantManager_1.CodeAssistantManager(context, webviewManager);
        // 注册命令
        registerCommands();
        // 创建状态栏
        statusBarManager.createStatusBar();
        // 加载会话
        await sessionManager.loadSessions();
        Logger_1.Logger.info('MindCortex VSCode 扩展激活成功');
        vscode.window.showInformationMessage('MindCortex 插件已激活');
        // 监听文档变化，自动索引
        if (vscode.workspace.getConfiguration('mindcortex').get('autoIndex', true)) {
            setupDocumentWatcher();
        }
    }
    catch (error) {
        Logger_1.Logger.error(`插件激活失败: ${error}`);
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
            handler: () => codeAssistantManager.addSnippet()
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
        extensionContext.subscriptions.push(vscode.commands.registerCommand(cmd.id, cmd.handler));
    });
    Logger_1.Logger.info(`已注册 ${commands.length} 个命令`);
}
/**
 * 设置文档监听器
 */
function setupDocumentWatcher() {
    const watcher = vscode.workspace.createFileSystemWatcher('**/*.{ts,js,tsx,jsx,py,java,go,rs,cpp,c,h}');
    watcher.onDidChange(async (uri) => {
        Logger_1.Logger.info(`文档已修改: ${uri.path}`);
        await searchManager.indexDocument(uri);
    });
    watcher.onDidCreate(async (uri) => {
        Logger_1.Logger.info(`文档已创建: ${uri.path}`);
        await searchManager.indexDocument(uri);
    });
    extensionContext.subscriptions.push(watcher);
    Logger_1.Logger.info('文档监听器已启动');
}
/**
 * 插件停用
 */
function deactivate() {
    Logger_1.Logger.info('MindCortex VSCode 扩展正在停用...');
    // 清理资源
    if (statusBarManager) {
        statusBarManager.dispose();
    }
    if (webviewManager) {
        webviewManager.dispose();
    }
    Logger_1.Logger.info('MindCortex VSCode 扩展已停用');
}
/**
 * 获取扩展上下文
 */
function getExtensionContext() {
    return extensionContext;
}
/**
 * 获取管理器实例
 */
function getManagers() {
    return {
        statusBar: statusBarManager,
        webview: webviewManager,
        session: sessionManager,
        search: searchManager,
        codeAssistant: codeAssistantManager
    };
}
//# sourceMappingURL=extension.js.map