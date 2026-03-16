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
exports.CommandManager = void 0;
const vscode = __importStar(require("vscode"));
const Logger_1 = require("../utils/Logger");
/**
 * 命令管理器
 * 负责注册和管理 VSCode 命令
 */
class CommandManager {
    constructor(context) {
        this.commands = new Map();
        this.context = context;
    }
    /**
     * 注册命令
     */
    registerCommand(commandId, callback, thisArg) {
        const disposable = vscode.commands.registerCommand(commandId, callback, thisArg);
        this.commands.set(commandId, disposable);
        this.context.subscriptions.push(disposable);
        Logger_1.Logger.info(`已注册命令: ${commandId}`);
    }
    /**
     * 执行命令
     */
    async executeCommand(commandId, ...args) {
        try {
            Logger_1.Logger.info(`执行命令: ${commandId}`);
            return await vscode.commands.executeCommand(commandId, ...args);
        }
        catch (error) {
            Logger_1.Logger.error(`命令执行失败 [${commandId}]: ${error}`);
            throw error;
        }
    }
    /**
     * 执行编辑器命令
     */
    async executeEditorCommand(commandId, ...args) {
        const editor = vscode.window.activeTextEditor;
        if (!editor) {
            Logger_1.Logger.warn('没有活动的编辑器');
            return;
        }
        try {
            Logger_1.Logger.info(`执行编辑器命令: ${commandId}`);
            return await vscode.commands.executeCommand(commandId, editor, ...args);
        }
        catch (error) {
            Logger_1.Logger.error(`编辑器命令执行失败 [${commandId}]: ${error}`);
            throw error;
        }
    }
    /**
     * 获取选中文本
     */
    getSelectedText() {
        const editor = vscode.window.activeTextEditor;
        if (!editor) {
            return undefined;
        }
        const selection = editor.selection;
        const document = editor.document;
        const text = document.getText(selection);
        return text || undefined;
    }
    /**
     * 获取当前文件路径
     */
    getCurrentFilePath() {
        const editor = vscode.window.activeTextEditor;
        if (!editor) {
            return undefined;
        }
        return editor.document.uri.fsPath;
    }
    /**
     * 获取当前文档语言
     */
    getCurrentLanguage() {
        const editor = vscode.window.activeTextEditor;
        if (!editor) {
            return undefined;
        }
        return editor.document.languageId;
    }
    /**
     * 在编辑器中插入文本
     */
    async insertText(text) {
        const editor = vscode.window.activeTextEditor;
        if (!editor) {
            vscode.window.showWarningMessage('没有活动的编辑器');
            return false;
        }
        const position = editor.selection.active;
        await editor.edit(editBuilder => {
            editBuilder.insert(position, text);
        });
        Logger_1.Logger.info('已插入文本到编辑器');
        return true;
    }
    /**
     * 替换选中的文本
     */
    async replaceSelectedText(text) {
        const editor = vscode.window.activeTextEditor;
        if (!editor) {
            vscode.window.showWarningMessage('没有活动的编辑器');
            return false;
        }
        const selection = editor.selection;
        await editor.edit(editBuilder => {
            editBuilder.replace(selection, text);
        });
        Logger_1.Logger.info('已替换选中的文本');
        return true;
    }
    /**
     * 打开文件
     */
    async openFile(uri) {
        try {
            const document = await vscode.workspace.openTextDocument(uri);
            await vscode.window.showTextDocument(document);
            Logger_1.Logger.info(`已打开文件: ${uri.fsPath}`);
        }
        catch (error) {
            Logger_1.Logger.error(`打开文件失败 [${uri.fsPath}]: ${error}`);
            throw error;
        }
    }
    /**
     * 显示输入框
     */
    async showInputBox(options) {
        return await vscode.window.showInputBox(options);
    }
    /**
     * 显示快速选择框
     */
    async showQuickPick(items, options) {
        return await vscode.window.showQuickPick(items, options);
    }
    /**
     * 显示信息消息
     */
    showInformationMessage(message, ...items) {
        return vscode.window.showInformationMessage(message, ...items);
    }
    /**
     * 显示警告消息
     */
    showWarningMessage(message, ...items) {
        return vscode.window.showWarningMessage(message, ...items);
    }
    /**
     * 显示错误消息
     */
    showErrorMessage(message, ...items) {
        return vscode.window.showErrorMessage(message, ...items);
    }
    /**
     * 释放所有命令
     */
    dispose() {
        this.commands.forEach((disposable, commandId) => {
            disposable.dispose();
            Logger_1.Logger.info(`已释放命令: ${commandId}`);
        });
        this.commands.clear();
        Logger_1.Logger.info('所有命令已释放');
    }
}
exports.CommandManager = CommandManager;
//# sourceMappingURL=CommandManager.js.map