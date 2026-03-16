import * as vscode from 'vscode';
import { Logger } from '../utils/Logger';

/**
 * 状态栏管理器
 * 负责管理 VSCode 状态栏的显示和更新
 */
export class StatusBarManager {
    private statusBarItem: vscode.StatusBarItem | undefined;
    private context: vscode.ExtensionContext;
    private isOnline: boolean = false;
    private searchCount: number = 0;

    constructor(context: vscode.ExtensionContext) {
        this.context = context;
    }

    /**
     * 创建状态栏项
     */
    public createStatusBar(): void {
        Logger.info('创建状态栏项...');

        this.statusBarItem = vscode.window.createStatusBarItem(
            vscode.StatusBarAlignment.Left,
            100
        );

        this.updateStatus();
        this.statusBarItem.command = 'mindcortex.openMainPanel';
        this.statusBarItem.show();

        Logger.info('状态栏项已创建');
    }

    /**
     * 更新状态栏状态
     */
    public updateStatus(): void {
        if (!this.statusBarItem) {
            return;
        }

        const icon = this.isOnline ? '$(check)' : '$(circle-outline)';
        const tooltip = this.isOnline ? 'MindCortex 已连接' : 'MindCortex 未连接';
        const text = `${icon} MindCortex${this.searchCount > 0 ? ` (${this.searchCount})` : ''}`;

        this.statusBarItem.text = text;
        this.statusBarItem.tooltip = tooltip;

        Logger.info(`状态栏已更新: ${text}`);
    }

    /**
     * 设置在线状态
     */
    public setOnline(online: boolean): void {
        this.isOnline = online;
        this.updateStatus();
    }

    /**
     * 更新搜索计数
     */
    public updateSearchCount(count: number): void {
        this.searchCount = count;
        this.updateStatus();
    }

    /**
     * 增加搜索计数
     */
    public incrementSearchCount(): void {
        this.searchCount++;
        this.updateStatus();
    }

    /**
     * 显示临时消息
     */
    public showTemporaryMessage(message: string, duration: number = 3000): void {
        if (this.statusBarItem) {
            const originalText = this.statusBarItem.text;
            this.statusBarItem.text = `$(loading~spin) ${message}`;

            setTimeout(() => {
                if (this.statusBarItem) {
                    this.statusBarItem.text = originalText;
                }
            }, duration);
        }
    }

    /**
     * 显示错误状态
     */
    public showError(message: string): void {
        if (this.statusBarItem) {
            this.statusBarItem.text = `$(error) MindCortex`;
            this.statusBarItem.tooltip = message;
            this.statusBarItem.backgroundColor = new vscode.ThemeColor('statusBarItem.errorBackground');
        }
    }

    /**
     * 显示警告状态
     */
    public showWarning(message: string): void {
        if (this.statusBarItem) {
            this.statusBarItem.text = `$(warning) MindCortex`;
            this.statusBarItem.tooltip = message;
            this.statusBarItem.backgroundColor = new vscode.ThemeColor('statusBarItem.warningBackground');
        }
    }

    /**
     * 重置为正常状态
     */
    public resetStatus(): void {
        this.isOnline = true;
        this.searchCount = 0;

        if (this.statusBarItem) {
            this.statusBarItem.backgroundColor = undefined;
            this.updateStatus();
        }
    }

    /**
     * 释放资源
     */
    public dispose(): void {
        if (this.statusBarItem) {
            this.statusBarItem.dispose();
            this.statusBarItem = undefined as any;
        }
        Logger.info('状态栏已释放');
    }
}
