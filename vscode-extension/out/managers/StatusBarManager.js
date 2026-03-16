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
exports.StatusBarManager = void 0;
const vscode = __importStar(require("vscode"));
const Logger_1 = require("../utils/Logger");
/**
 * 状态栏管理器
 * 负责管理 VSCode 状态栏的显示和更新
 */
class StatusBarManager {
    constructor(context) {
        this.isOnline = false;
        this.searchCount = 0;
        this.context = context;
    }
    /**
     * 创建状态栏项
     */
    createStatusBar() {
        Logger_1.Logger.info('创建状态栏项...');
        this.statusBarItem = vscode.window.createStatusBarItem(vscode.StatusBarAlignment.Left, 100);
        this.updateStatus();
        this.statusBarItem.command = 'mindcortex.openMainPanel';
        this.statusBarItem.show();
        Logger_1.Logger.info('状态栏项已创建');
    }
    /**
     * 更新状态栏状态
     */
    updateStatus() {
        if (!this.statusBarItem) {
            return;
        }
        const icon = this.isOnline ? '$(check)' : '$(circle-outline)';
        const tooltip = this.isOnline ? 'MindCortex 已连接' : 'MindCortex 未连接';
        const text = `${icon} MindCortex${this.searchCount > 0 ? ` (${this.searchCount})` : ''}`;
        this.statusBarItem.text = text;
        this.statusBarItem.tooltip = tooltip;
        Logger_1.Logger.info(`状态栏已更新: ${text}`);
    }
    /**
     * 设置在线状态
     */
    setOnline(online) {
        this.isOnline = online;
        this.updateStatus();
    }
    /**
     * 更新搜索计数
     */
    updateSearchCount(count) {
        this.searchCount = count;
        this.updateStatus();
    }
    /**
     * 增加搜索计数
     */
    incrementSearchCount() {
        this.searchCount++;
        this.updateStatus();
    }
    /**
     * 显示临时消息
     */
    showTemporaryMessage(message, duration = 3000) {
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
    showError(message) {
        if (this.statusBarItem) {
            this.statusBarItem.text = `$(error) MindCortex`;
            this.statusBarItem.tooltip = message;
            this.statusBarItem.backgroundColor = new vscode.ThemeColor('statusBarItem.errorBackground');
        }
    }
    /**
     * 显示警告状态
     */
    showWarning(message) {
        if (this.statusBarItem) {
            this.statusBarItem.text = `$(warning) MindCortex`;
            this.statusBarItem.tooltip = message;
            this.statusBarItem.backgroundColor = new vscode.ThemeColor('statusBarItem.warningBackground');
        }
    }
    /**
     * 重置为正常状态
     */
    resetStatus() {
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
    dispose() {
        if (this.statusBarItem) {
            this.statusBarItem.dispose();
            this.statusBarItem = undefined;
        }
        Logger_1.Logger.info('状态栏已释放');
    }
}
exports.StatusBarManager = StatusBarManager;
//# sourceMappingURL=StatusBarManager.js.map