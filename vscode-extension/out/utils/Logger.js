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
exports.Logger = void 0;
const vscode = __importStar(require("vscode"));
/**
 * Logger 工具类
 * 提供统一的日志记录功能
 */
class Logger {
    /**
     * 获取输出通道
     */
    static getOutputChannel() {
        if (!this.outputChannel) {
            this.outputChannel = vscode.window.createOutputChannel(this.OUTPUT_CHANNEL_NAME);
        }
        return this.outputChannel;
    }
    /**
     * 格式化日志消息
     */
    static formatMessage(level, message) {
        const timestamp = new Date().toISOString();
        return `[${timestamp}] [${level}] ${message}`;
    }
    /**
     * 输出日志
     */
    static log(level, message) {
        const formattedMessage = this.formatMessage(level, message);
        console.log(formattedMessage);
        const channel = this.getOutputChannel();
        channel.appendLine(formattedMessage);
    }
    /**
     * 信息日志
     */
    static info(message) {
        this.log('INFO', message);
    }
    /**
     * �告日志
     */
    static warn(message) {
        this.log('WARN', message);
    }
    /**
     * 错误日志
     */
    static error(message) {
        this.log('ERROR', message);
    }
    /**
     * 调试日志
     */
    static debug(message) {
        if (this.isDebugEnabled()) {
            this.log('DEBUG', message);
        }
    }
    /**
     * 检查是否启用调试模式
     */
    static isDebugEnabled() {
        // 从配置读取调试开关
        const config = vscode.workspace.getConfiguration('mindcortex');
        return config.get('debug', false);
    }
    /**
     * 显示输出通道
     */
    static showOutputChannel() {
        const channel = this.getOutputChannel();
        channel.show(true);
    }
    /**
     * 清除输出通道
     */
    static clearOutputChannel() {
        const channel = this.getOutputChannel();
        channel.clear();
    }
    /**
     * 释放资源
     */
    static dispose() {
        if (this.outputChannel) {
            this.outputChannel.dispose();
            this.outputChannel = undefined;
        }
    }
}
exports.Logger = Logger;
Logger.OUTPUT_CHANNEL_NAME = 'MindCortex';
//# sourceMappingURL=Logger.js.map