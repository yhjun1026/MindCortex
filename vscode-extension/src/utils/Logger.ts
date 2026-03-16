import * as vscode from 'vscode';

/**
 * Logger 工具类
 * 提供统一的日志记录功能
 */
export class Logger {
    private static readonly OUTPUT_CHANNEL_NAME = 'MindCortex';
    private static outputChannel: vscode.OutputChannel | undefined;

    /**
     * 获取输出通道
     */
    private static getOutputChannel(): vscode.OutputChannel {
        if (!this.outputChannel) {
            this.outputChannel = vscode.window.createOutputChannel(this.OUTPUT_CHANNEL_NAME);
        }

        return this.outputChannel;
    }

    /**
     * 格式化日志消息
     */
    private static formatMessage(level: string, message: string): string {
        const timestamp = new Date().toISOString();
        return `[${timestamp}] [${level}] ${message}`;
    }

    /**
     * 输出日志
     */
    private static log(level: string, message: string): void {
        const formattedMessage = this.formatMessage(level, message);
        console.log(formattedMessage);

        const channel = this.getOutputChannel();
        channel.appendLine(formattedMessage);
    }

    /**
     * 信息日志
     */
    public static info(message: string): void {
        this.log('INFO', message);
    }

    /**
     * �告日志
     */
    public static warn(message: string): void {
        this.log('WARN', message);
    }

    /**
     * 错误日志
     */
    public static error(message: string): void {
        this.log('ERROR', message);
    }

    /**
     * 调试日志
     */
    public static debug(message: string): void {
        if (this.isDebugEnabled()) {
            this.log('DEBUG', message);
        }
    }

    /**
     * 检查是否启用调试模式
     */
    private static isDebugEnabled(): boolean {
        // 从配置读取调试开关
        const config = vscode.workspace.getConfiguration('mindcortex');
        return config.get('debug', false);
    }

    /**
     * 显示输出通道
     */
    public static showOutputChannel(): void {
        const channel = this.getOutputChannel();
        channel.show(true);
    }

    /**
     * 清除输出通道
     */
    public static clearOutputChannel(): void {
        const channel = this.getOutputChannel();
        channel.clear();
    }

    /**
     * 释放资源
     */
    public static dispose(): void {
        if (this.outputChannel) {
            this.outputChannel.dispose();
            this.outputChannel = undefined;
        }
    }
}
