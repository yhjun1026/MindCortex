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
exports.SessionManager = void 0;
const vscode = __importStar(require("vscode"));
const Logger_1 = require("../utils/Logger");
/**
 * 会话管理器
 * 负责管理用户会话、标签分类和会话切换
 */
class SessionManager {
    constructor(context) {
        this.sessions = new Map();
        this.SESSIONS_KEY = 'mindcortex.sessions';
        this.CURRENT_SESSION_KEY = 'mindcortex.currentSession';
        this.context = context;
    }
    /**
     * 加载会话数据
     */
    async loadSessions() {
        try {
            const sessionsData = this.context.globalState.get(this.SESSIONS_KEY, []);
            this.sessions.clear();
            sessionsData.forEach(session => {
                this.sessions.set(session.id, session);
            });
            // 加载当前会话
            this.currentSessionId = this.context.globalState.get(this.CURRENT_SESSION_KEY);
            Logger_1.Logger.info(`已加载 ${this.sessions.size} 个会话`);
            // 如果没有当前会话，创建一个默认会话
            if (!this.currentSessionId && this.sessions.size === 0) {
                await this.createDefaultSession();
            }
        }
        catch (error) {
            Logger_1.Logger.error(`加载会话失败: ${error}`);
        }
    }
    /**
     * 保存会话数据
     */
    async saveSessions() {
        try {
            const sessionsArray = Array.from(this.sessions.values());
            await this.context.globalState.update(this.SESSIONS_KEY, sessionsArray);
            if (this.currentSessionId) {
                await this.context.globalState.update(this.CURRENT_SESSION_KEY, this.currentSessionId);
            }
            Logger_1.Logger.info('会话数据已保存');
        }
        catch (error) {
            Logger_1.Logger.error(`保存会话失败: ${error}`);
        }
    }
    /**
     * 创建默认会话
     */
    async createDefaultSession() {
        const session = await this.createSession({
            name: '默认会话',
            description: '默认代码会话',
            language: 'typescript',
            tags: ['default']
        });
        await this.switchSession(session.id);
        return session;
    }
    /**
     * 创建新会话
     */
    async createSession(options) {
        const now = Date.now();
        const session = {
            id: this.generateSessionId(),
            name: options.name || `会话 ${this.sessions.size + 1}`,
            description: options.description,
            language: options.language,
            createdAt: now,
            lastUsedAt: now,
            tags: options.tags || [],
            metadata: options.metadata || {}
        };
        this.sessions.set(session.id, session);
        await this.saveSessions();
        Logger_1.Logger.info(`已创建会话: ${session.name} (${session.id})`);
        vscode.window.showInformationMessage(`已创建会话: ${session.name}`);
        return session;
    }
    /**
     * 切换会话
     */
    async switchSession(sessionId) {
        const session = this.sessions.get(sessionId);
        if (!session) {
            vscode.window.showErrorMessage('会话不存在');
            return undefined;
        }
        this.currentSessionId = sessionId;
        session.lastUsedAt = Date.now();
        await this.saveSessions();
        Logger_1.Logger.info(`已切换到会话: ${session.name}`);
        vscode.window.showInformationMessage(`已切换到会话: ${session.name}`);
        return session;
    }
    /**
     * 获取当前会话
     */
    getCurrentSession() {
        if (!this.currentSessionId) {
            return undefined;
        }
        return this.sessions.get(this.currentSessionId);
    }
    /**
     * 获取所有会话
     */
    getAllSessions() {
        return Array.from(this.sessions.values()).sort((a, b) => {
            return b.lastUsedAt - a.lastUsedAt;
        });
    }
    /**
     * 更新会话
     */
    async updateSession(sessionId, updates) {
        const session = this.sessions.get(sessionId);
        if (!session) {
            vscode.window.showErrorMessage('会话不存在');
            return undefined;
        }
        Object.assign(session, updates);
        session.lastUsedAt = Date.now();
        await this.saveSessions();
        Logger_1.Logger.info(`已更新会话: ${session.name}`);
        return session;
    }
    /**
     * 删除会话
     */
    async deleteSession(sessionId) {
        const session = this.sessions.get(sessionId);
        if (!session) {
            vscode.window.showErrorMessage('会话不存在');
            return false;
        }
        this.sessions.delete(sessionId);
        // 如果删除的是当前会话，切换到第一个可用的会话
        if (this.currentSessionId === sessionId) {
            const remainingSessions = this.getAllSessions();
            if (remainingSessions.length > 0) {
                await this.switchSession(remainingSessions[0].id);
            }
            else {
                this.currentSessionId = undefined;
                await this.createDefaultSession();
            }
        }
        await this.saveSessions();
        Logger_1.Logger.info(`已删除会话: ${session.name}`);
        vscode.window.showInformationMessage(`已删除会话: ${session.name}`);
        return true;
    }
    /**
     * 添加标签到会话
     */
    async addTag(sessionId, tag) {
        const session = this.sessions.get(sessionId);
        if (!session) {
            return;
        }
        if (!session.tags.includes(tag)) {
            session.tags.push(tag);
            await this.saveSessions();
        }
    }
    /**
     * 从会话中移除标签
     */
    async removeTag(sessionId, tag) {
        const session = this.sessions.get(sessionId);
        if (!session) {
            return;
        }
        session.tags = session.tags.filter(t => t !== tag);
        await this.saveSessions();
    }
    /**
     * 按标签筛选会话
     */
    filterSessionsByTag(tag) {
        return this.getAllSessions().filter(session => session.tags.includes(tag));
    }
    /**
     * 管理会话（显示快速选择面板）
     */
    async manageSessions() {
        const options = [
            { label: '$(plus) 创建新会话', description: '创建一个新会话' },
            { label: '$(edit) 重命名当前会话', description: '重命名当前会话' },
            { label: '$(list-ordered) 切换会话', description: '切换到其他会话' },
            { label: '$(trash) 删除会话', description: '删除会话' }
        ];
        const selected = await vscode.window.showQuickPick(options, {
            placeHolder: '会话管理',
            ignoreFocusOut: true
        });
        if (!selected) {
            return;
        }
        switch (selected.label) {
            case '$(plus) 创建新会话':
                await this.promptCreateSession();
                break;
            case '$(edit) 重命名当前会话':
                await this.promptRenameSession();
                break;
            case '$(list-ordered) 切换会话':
                await this.promptSwitchSession();
                break;
            case '$(trash) 删除会话':
                await this.promptDeleteSession();
                break;
        }
    }
    /**
     * 提示创建新会话
     */
    async promptCreateSession() {
        const name = await vscode.window.showInputBox({
            prompt: '输入会话名称',
            placeHolder: '我的新会话'
        });
        if (!name) {
            return;
        }
        const description = await vscode.window.showInputBox({
            prompt: '输入会话描述（可选）',
            placeHolder: '会话描述'
        });
        await this.createSession({
            name,
            description: description
        });
    }
    /**
     * 提示重命名会话
     */
    async promptRenameSession() {
        const currentSession = this.getCurrentSession();
        if (!currentSession) {
            vscode.window.showErrorMessage('没有当前会话');
            return;
        }
        const name = await vscode.window.showInputBox({
            prompt: '输入新的会话名称',
            value: currentSession.name
        });
        if (!name) {
            return;
        }
        await this.updateSession(currentSession.id, { name });
    }
    /**
     * 提示切换会话
     */
    async promptSwitchSession() {
        const sessions = this.getAllSessions();
        if (sessions.length === 0) {
            vscode.window.showInformationMessage('没有可用的会话');
            return;
        }
        const items = sessions.map(session => ({
            label: session.name,
            description: session.description || '',
            detail: this.currentSessionId === session.id ? '$(check) 当前' : undefined
        }));
        const selected = await vscode.window.showQuickPick(items, {
            placeHolder: '选择要切换的会话'
        });
        if (!selected) {
            return;
        }
        const session = sessions.find(s => s.name === selected.label);
        if (session) {
            await this.switchSession(session.id);
        }
    }
    /**
     * 提示删除会话
     */
    async promptDeleteSession() {
        const sessions = this.getAllSessions();
        if (sessions.length === 0) {
            vscode.window.showInformationMessage('没有可用的会话');
            return;
        }
        const items = sessions.map(session => ({
            label: session.name,
            description: session.description || ''
        }));
        const selected = await vscode.window.showQuickPick(items, {
            placeHolder: '选择要删除的会话'
        });
        if (!selected) {
            return;
        }
        const confirm = await vscode.window.showWarningMessage(`确定要删除会话 "${selected.label}" 吗？`, '删除', '取消');
        if (confirm === '删除') {
            const session = sessions.find(s => s.name === selected.label);
            if (session) {
                await this.deleteSession(session.id);
            }
        }
    }
    /**
     * 导出会话
     */
    async exportSession(sessionId) {
        const session = this.sessions.get(sessionId);
        if (!session) {
            throw new Error('会话不存在');
        }
        const json = JSON.stringify(session, null, 2);
        return json;
    }
    /**
     * 导入会话
     */
    async importSession(json) {
        try {
            const session = JSON.parse(json);
            // 生成新的 ID 以避免冲突
            session.id = this.generateSessionId();
            this.sessions.set(session.id, session);
            await this.saveSessions();
            Logger_1.Logger.info(`已导入会话: ${session.name}`);
            return session;
        }
        catch (error) {
            Logger_1.Logger.error(`导入会话失败: ${error}`);
            throw new Error('导入会话失败: JSON 格式错误');
        }
    }
    /**
     * 生成会话 ID
     */
    generateSessionId() {
        return `session_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`;
    }
}
exports.SessionManager = SessionManager;
//# sourceMappingURL=SessionManager.js.map