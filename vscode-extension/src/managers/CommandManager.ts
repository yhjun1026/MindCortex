import * as vscode from 'vscode';
import { Logger } from '../utils/Logger';

/**
 * 命令管理器
 * 负责注册和管理 VSCode 命令
 */
export class CommandManager {
    private context: vscode.ExtensionContext;
    private commands: Map<string, vscode.Disposable> = new Map();

    constructor(context: vscode.ExtensionContext) {
        this.context = context;
    }

    /**
     * 注册命令
     */
    public registerCommand(
        commandId: string,
        callback: (...args: any[]) => any,
        thisArg?: any
    ): void {
        const disposable = vscode.commands.registerCommand(
            commandId,
            callback,
            thisArg
        );

        this.commands.set(commandId, disposable);
        this.context.subscriptions.push(disposable);

        Logger.info(`已注册命令: ${commandId}`);
    }

    /**
     * 执行命令
     */
    public async executeCommand(
        commandId: string,
        ...args: any[]
    ): Promise<unknown> {
        try {
            Logger.info(`执行命令: ${commandId}`);
            return await vscode.commands.executeCommand(commandId, ...args);
        } catch (error) {
            Logger.error(`命令执行失败 [${commandId}]: ${error}`);
            throw error;
        }
    }

    /**
     * 执行编辑器命令
     */
    public async executeEditorCommand(
        commandId: string,
        ...args: any[]
    ): Promise<unknown> {
        const editor = vscode.window.activeTextEditor;
        if (!editor) {
            Logger.warn('没有活动的编辑器');
            return;
        }

        try {
            Logger.info(`执行编辑器命令: ${commandId}`);
            return await vscode.commands.executeCommand(
                commandId,
                editor,
                ...args
            );
        } catch (error) {
            Logger.error(`编辑器命令执行失败 [${commandId}]: ${error}`);
            throw error;
        }
    }

    /**
     * 获取选中文本
     */
    public getSelectedText(): string | undefined {
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
    public getCurrentFilePath(): string | undefined {
        const editor = vscode.window.activeTextEditor;
        if (!editor) {
            return undefined;
        }

        return editor.document.uri.fsPath;
    }

    /**
     * 获取当前文档语言
     */
    public getCurrentLanguage(): string | undefined {
        const editor = vscode.window.activeTextEditor;
        if (!editor) {
            return undefined;
        }

        return editor.document.languageId;
    }

    /**
     * 在编辑器中插入文本
     */
    public async insertText(text: string): Promise<boolean> {
        const editor = vscode.window.activeTextEditor;
        if (!editor) {
            vscode.window.showWarningMessage('没有活动的编辑器');
            return false;
        }

        const position = editor.selection.active;
        await editor.edit(editBuilder => {
            editBuilder.insert(position, text);
        });

        Logger.info('已插入文本到编辑器');
        return true;
    }

    /**
     * 替换选中的文本
     */
    public async replaceSelectedText(text: string): Promise<boolean> {
        const editor = vscode.window.activeTextEditor;
        if (!editor) {
            vscode.window.showWarningMessage('没有活动的编辑器');
            return false;
        }

        const selection = editor.selection;
        await editor.edit(editBuilder => {
            editBuilder.replace(selection, text);
        });

        Logger.info('已替换选中的文本');
        return true;
    }

    /**
     * 打开文件
     */
    public async openFile(uri: vscode.Uri): Promise<void> {
        try {
            const document = await vscode.workspace.openTextDocument(uri);
            await vscode.window.showTextDocument(document);
            Logger.info(`已打开文件: ${uri.fsPath}`);
        } catch (error) {
            Logger.error(`打开文件失败 [${uri.fsPath}]: ${error}`);
            throw error;
        }
    }

    /**
     * 显示输入框
     */
    public async showInputBox(
        options: vscode.InputBoxOptions
    ): Promise<string | undefined> {
        return await vscode.window.showInputBox(options);
    }

    /**
     * 显示快速选择框
     */
    public async showQuickPick<T extends vscode.QuickPickItem>(
        items: T[] | Thenable<T[]>,
        options?: vscode.QuickPickOptions
    ): Promise<T | undefined> {
        return await vscode.window.showQuickPick(items, options);
    }

    /**
     * 显示信息消息
     */
    public showInformationMessage(
        message: string,
        ...items: string[]
    ): Thenable<string | undefined> {
        return vscode.window.showInformationMessage(message, ...items);
    }

    /**
     * 显示警告消息
     */
    public showWarningMessage(
        message: string,
        ...items: string[]
    ): Thenable<string | undefined> {
        return vscode.window.showWarningMessage(message, ...items);
    }

    /**
     * 显示错误消息
     */
    public showErrorMessage(
        message: string,
        ...items: string[]
    ): Thenable<string | undefined> {
        return vscode.window.showErrorMessage(message, ...items);
    }

    /**
     * 释放所有命令
     */
    public dispose(): void {
        this.commands.forEach((disposable, commandId) => {
            disposable.dispose();
            Logger.info(`已释放命令: ${commandId}`);
        });

        this.commands.clear();
        Logger.info('所有命令已释放');
    }
}
