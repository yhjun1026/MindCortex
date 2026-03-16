import React, { useState, useEffect } from 'react';

interface Agent {
  id: string;
  name: string;
  type: string;
  status: 'connected' | 'disconnected' | 'connecting';
  description: string;
  config?: Record<string, any>;
  last_used?: number;
}

export const AgentsPage: React.FC = () => {
  const [agents, setAgents] = useState<Agent[]>([]);
  const [loading, setLoading] = useState(false);
  const [showAddModal, setShowAddModal] = useState(false);
  const [newAgentName, setNewAgentName] = useState('');
  const [newAgentType, setNewAgentType] = useState('opencode');

  useEffect(() => {
    loadAgents();
  }, []);

  const loadAgents = async () => {
    setLoading(true);
    try {
      // TODO: 从后端 API 加载代理列表
      // const result = await invoke<Agent[]>('get_agents');
      // setAgents(result);

      // 模拟数据
      setAgents([
        {
          id: '1',
          name: 'OpenCode',
          type: 'opencode',
          status: 'connected',
          description: 'AI 代码助手，支持多种编程语言',
          config: { api_key: '***', model: 'gpt-4' },
          last_used: Date.now() / 1000 - 3600
        },
        {
          id: '2',
          name: 'ClaudeCode',
          type: 'claudecode',
          status: 'connected',
          description: 'Anthropic Claude 代码助手',
          config: { api_key: '***', model: 'claude-3-opus' },
          last_used: Date.now() / 1000 - 7200
        },
        {
          id: '3',
          name: 'OpenClaw',
          type: 'openclaw',
          status: 'disconnected',
          description: '多模态 AI 助手，支持图片和文件',
          config: { endpoint: 'https://api.openclaw.io' }
        }
      ]);
    } catch (error) {
      console.error('Failed to load agents:', error);
    } finally {
      setLoading(false);
    }
  };

  const handleAddAgent = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!newAgentName.trim()) return;

    try {
      // TODO: 调用后端 API 添加代理
      // await invoke('add_agent', {
      //   name: newAgentName,
      //   type: newAgentType
      // });

      // 临时添加模拟数据
      const newAgent: Agent = {
        id: Date.now().toString(),
        name: newAgentName,
        type: newAgentType,
        status: 'disconnected',
        description: '新添加的代理'
      };
      setAgents([...agents, newAgent]);

      setNewAgentName('');
      setShowAddModal(false);
    } catch (error) {
      console.error('Failed to add agent:', error);
    }
  };

  const handleDeleteAgent = async (agentId: string) => {
    try {
      // TODO: 调用后端 API 删除代理
      // await invoke('delete_agent', { id: agentId });

      setAgents(agents.filter(agent => agent.id !== agentId));
    } catch (error) {
      console.error('Failed to delete agent:', error);
    }
  };

  const handleConnectAgent = async (agentId: string) => {
    try {
      setAgents(agents.map(agent =>
        agent.id === agentId
          ? { ...agent, status: 'connecting' as const }
          : agent
      ));

      // TODO: 调用后端 API 连接代理
      // await invoke('connect_agent', { id: agentId });

      // 模拟连接延迟
      setTimeout(() => {
        setAgents(agents.map(agent =>
          agent.id === agentId
            ? { ...agent, status: 'connected' as const, last_used: Date.now() / 1000 }
            : agent
        ));
      }, 2000);
    } catch (error) {
      console.error('Failed to connect agent:', error);
    }
  };

  const handleDisconnectAgent = async (agentId: string) => {
    try {
      // TODO: 调用后端 API 断开连接
      // await invoke('disconnect_agent', { id: agentId });

      setAgents(agents.map(agent =>
        agent.id === agentId
          ? { ...agent, status: 'disconnected' as const }
          : agent
      ));
    } catch (error) {
      console.error('Failed to disconnect agent:', error);
    }
  };

  const getStatusColor = (status: Agent['status']) => {
    switch (status) {
      case 'connected':
        return 'var(--success-color)';
      case 'connecting':
        return '#f59e0b';
      case 'disconnected':
        return 'var(--text-secondary)';
    }
  };

  const getStatusText = (status: Agent['status']) => {
    switch (status) {
      case 'connected':
        return '已连接';
      case 'connecting':
        return '连接中...';
      case 'disconnected':
        return '未连接';
    }
  };

  return (
    <div className="agents-page">
      <div className="page-header">
        <h1 className="page-title">🔌 AI 代理</h1>
        <p className="page-subtitle">管理和配置您的 AI 工具连接</p>
      </div>

      {/* 添加代理按钮 */}
      <div className="actions-bar">
        <button
          className="add-button"
          onClick={() => setShowAddModal(true)}
        >
          + 添加代理
        </button>
      </div>

      {/* 加载状态 */}
      {loading && (
        <div className="loading-state">
          <div className="spinner"></div>
          <p>加载代理列表...</p>
        </div>
      )}

      {/* 代理列表 */}
      {!loading && (
        <div className="agents-list">
          {agents.length === 0 ? (
            <div className="empty-state">
              <p>还没有添加任何代理</p>
              <button onClick={() => setShowAddModal(true)}>添加第一个代理</button>
            </div>
          ) : (
            <div className="agents-grid">
              {agents.map(agent => (
                <div key={agent.id} className="agent-card">
                  <div className="agent-header">
                    <h3 className="agent-name">{agent.name}</h3>
                    <div className="agent-status">
                      <span
                        className="status-indicator"
                        style={{ backgroundColor: getStatusColor(agent.status) }}
                      ></span>
                      <span className="status-text">{getStatusText(agent.status)}</span>
                    </div>
                  </div>

                  <p className="agent-description">{agent.description}</p>

                  <div className="agent-type">
                    <span className="type-label">类型:</span>
                    <span className="type-value">{agent.type}</span>
                  </div>

                  {agent.last_used && (
                    <div className="agent-last-used">
                      <span>最后使用: {new Date(agent.last_used * 1000).toLocaleString()}</span>
                    </div>
                  )}

                  <div className="agent-actions">
                    {agent.status === 'connected' ? (
                      <button
                        className="disconnect-button"
                        onClick={() => handleDisconnectAgent(agent.id)}
                      >
                        断开连接
                      </button>
                    ) : (
                      <button
                        className="connect-button"
                        onClick={() => handleConnectAgent(agent.id)}
                        disabled={agent.status === 'connecting'}
                      >
                        {agent.status === 'connecting' ? '连接中...' : '连接'}
                      </button>
                    )}
                    <button
                      className="delete-button"
                      onClick={() => {
                        if (confirm(`确定要删除 "${agent.name}" 吗？`)) {
                          handleDeleteAgent(agent.id);
                        }
                      }}
                    >
                      删除
                    </button>
                  </div>
                </div>
              ))}
            </div>
          )}
        </div>
      )}

      {/* 添加代理模态框 */}
      {showAddModal && (
        <div className="modal-overlay">
          <div className="modal">
            <div className="modal-header">
              <h2>添加新代理</h2>
              <button
                className="close-button"
                onClick={() => setShowAddModal(false)}
              >
                ✕
              </button>
            </div>
            <form onSubmit={handleAddAgent}>
              <div className="form-group">
                <label>代理名称</label>
                <input
                  type="text"
                  value={newAgentName}
                  onChange={(e) => setNewAgentName(e.target.value)}
                  placeholder="例如: OpenCode"
                  required
                />
              </div>
              <div className="form-group">
                <label>代理类型</label>
                <select
                  value={newAgentType}
                  onChange={(e) => setNewAgentType(e.target.value)}
                >
                  <option value="opencode">OpenCode</option>
                  <option value="claudecode">ClaudeCode</option>
                  <option value="openclaw">OpenClaw</option>
                  <option value="cursor">Cursor</option>
                  <option value="custom">自定义</option>
                </select>
              </div>
              <div className="modal-actions">
                <button type="button" onClick={() => setShowAddModal(false)}>
                  取消
                </button>
                <button type="submit" disabled={!newAgentName.trim()}>
                  添加
                </button>
              </div>
            </form>
          </div>
        </div>
      )}

      <style>{`
        .agents-page {
          padding: 20px;
        }

        .page-header {
          margin-bottom: 32px;
        }

        .page-title {
          font-size: 32px;
          font-weight: bold;
          margin-bottom: 8px;
        }

        .page-subtitle {
          color: var(--text-secondary);
          font-size: 16px;
        }

        .actions-bar {
          margin-bottom: 24px;
        }

        .add-button {
          background: var(--primary-color);
          border: none;
          padding: 10px 20px;
          border-radius: 8px;
          color: white;
          font-size: 14px;
          font-weight: 500;
          cursor: pointer;
          transition: background 0.2s;
        }

        .add-button:hover {
          background: var(--primary-hover);
        }

        .loading-state {
          text-align: center;
          padding: 60px 20px;
        }

        .spinner {
          width: 40px;
          height: 40px;
          border: 3px solid var(--border-color);
          border-top-color: var(--primary-color);
          border-radius: 50%;
          animation: spin 1s linear infinite;
          margin: 0 auto 16px;
        }

        @keyframes spin {
          to { transform: rotate(360deg); }
        }

        .empty-state {
          text-align: center;
          padding: 60px 20px;
          color: var(--text-secondary);
        }

        .empty-state button {
          margin-top: 16px;
          background: var(--primary-color);
          border: none;
          padding: 8px 16px;
          border-radius: 6px;
          color: white;
          cursor: pointer;
        }

        .agents-grid {
          display: grid;
          grid-template-columns: repeat(auto-fill, minmax(350px, 1fr));
          gap: 20px;
        }

        .agent-card {
          background: var(--card-bg);
          border: 1px solid var(--border-color);
          border-radius: 12px;
          padding: 20px;
          transition: all 0.2s;
        }

        .agent-card:hover {
          border-color: var(--primary-color);
        }

        .agent-header {
          display: flex;
          justify-content: space-between;
          align-items: center;
          margin-bottom: 12px;
        }

        .agent-name {
          font-size: 18px;
          font-weight: 600;
          margin: 0;
        }

        .agent-status {
          display: flex;
          align-items: center;
          gap: 6px;
          font-size: 12px;
        }

        .status-indicator {
          width: 8px;
          height: 8px;
          border-radius: 50%;
          animation: pulse 2s infinite;
        }

        @keyframes pulse {
          0%, 100% { opacity: 1; }
          50% { opacity: 0.5; }
        }

        .status-text {
          color: var(--text-secondary);
        }

        .agent-description {
          color: var(--text-secondary);
          font-size: 14px;
          line-height: 1.6;
          margin-bottom: 16px;
        }

        .agent-type {
          display: flex;
          gap: 8px;
          font-size: 13px;
          margin-bottom: 8px;
        }

        .type-label {
          color: var(--text-secondary);
        }

        .type-value {
          color: var(--text-primary);
          font-weight: 500;
        }

        .agent-last-used {
          font-size: 12px;
          color: var(--text-secondary);
          margin-bottom: 16px;
        }

        .agent-actions {
          display: flex;
          gap: 8px;
          padding-top: 16px;
          border-top: 1px solid var(--border-color);
        }

        .connect-button,
        .disconnect-button {
          flex: 1;
          padding: 8px 16px;
          border: none;
          border-radius: 6px;
          font-size: 13px;
          cursor: pointer;
          transition: all 0.2s;
        }

        .connect-button {
          background: var(--primary-color);
          color: white;
        }

        .connect-button:hover:not(:disabled) {
          background: var(--primary-hover);
        }

        .connect-button:disabled {
          opacity: 0.6;
          cursor: not-allowed;
        }

        .disconnect-button {
          background: var(--danger-color);
          color: white;
        }

        .disconnect-button:hover {
          opacity: 0.8;
        }

        .delete-button {
          padding: 8px 16px;
          background: transparent;
          border: 1px solid var(--border-color);
          color: var(--text-secondary);
          border-radius: 6px;
          font-size: 13px;
          cursor: pointer;
          transition: all 0.2s;
        }

        .delete-button:hover {
          border-color: var(--danger-color);
          color: var(--danger-color);
        }

        /* Modal */
        .modal-overlay {
          position: fixed;
          top: 0;
          left: 0;
          right: 0;
          bottom: 0;
          background: rgba(0, 0, 0, 0.7);
          display: flex;
          align-items: center;
          justify-content: center;
          z-index: 1000;
        }

        .modal {
          background: var(--card-bg);
          border: 1px solid var(--border-color);
          border-radius: 12px;
          padding: 24px;
          width: 100%;
          max-width: 400px;
        }

        .modal-header {
          display: flex;
          justify-content: space-between;
          align-items: center;
          margin-bottom: 24px;
        }

        .modal-header h2 {
          font-size: 20px;
          font-weight: 600;
          margin: 0;
        }

        .close-button {
          background: transparent;
          border: none;
          font-size: 24px;
          cursor: pointer;
          color: var(--text-secondary);
          padding: 0;
        }

        .form-group {
          margin-bottom: 20px;
        }

        .form-group label {
          display: block;
          margin-bottom: 8px;
          font-size: 14px;
          font-weight: 500;
          color: var(--text-secondary);
        }

        .form-group input,
        .form-group select {
          width: 100%;
          padding: 10px 12px;
          background: var(--bg-color);
          border: 1px solid var(--border-color);
          border-radius: 6px;
          color: var(--text-primary);
          font-size: 14px;
        }

        .form-group input:focus,
        .form-group select:focus {
          outline: none;
          border-color: var(--primary-color);
        }

        .modal-actions {
          display: flex;
          gap: 12px;
          justify-content: flex-end;
          margin-top: 24px;
        }

        .modal-actions button {
          padding: 10px 20px;
          border-radius: 6px;
          font-size: 14px;
          cursor: pointer;
          transition: all 0.2s;
        }

        .modal-actions button[type="button"] {
          background: transparent;
          border: 1px solid var(--border-color);
          color: var(--text-secondary);
        }

        .modal-actions button[type="button"]:hover {
          border-color: var(--text-primary);
          color: var(--text-primary);
        }

        .modal-actions button[type="submit"] {
          background: var(--primary-color);
          border: none;
          color: white;
        }

        .modal-actions button[type="submit"]:hover:not(:disabled) {
          background: var(--primary-hover);
        }

        .modal-actions button[type="submit"]:disabled {
          opacity: 0.6;
          cursor: not-allowed;
        }
      `}</style>
    </div>
  );
};
