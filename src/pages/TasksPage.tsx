import React, { useState, useEffect } from 'react';

interface Task {
  id: string;
  title: string;
  description: string;
  status: 'todo' | 'in-progress' | 'completed';
  priority: 'low' | 'medium' | 'high';
  due_date?: number;
  created_at: number;
  updated_at: number;
  tags?: string[];
}

export const TasksPage: React.FC = () => {
  const [tasks, setTasks] = useState<Task[]>([]);
  const [loading, setLoading] = useState(false);
  const [filterStatus, setFilterStatus] = useState<string>('all');
  const [showAddModal, setShowAddModal] = useState(false);
  const [editingTask, setEditingTask] = useState<Task | null>(null);

  const [newTitle, setNewTitle] = useState('');
  const [newDescription, setNewDescription] = useState('');
  const [newPriority, setNewPriority] = useState<'low' | 'medium' | 'high'>('medium');
  const [newDueDate, setNewDueDate] = useState('');

  useEffect(() => {
    loadTasks();
  }, []);

  const loadTasks = async () => {
    setLoading(true);
    try {
      setTasks([
        {
          id: '1',
          title: '完成 MindCortex 前端开发',
          description: '实现所有缺失的功能页面',
          status: 'in-progress',
          priority: 'high',
          due_date: Date.now() / 1000 + 86400,
          created_at: Date.now() / 1000 - 86400,
          updated_at: Date.now() / 1000,
          tags: ['开发', '前端']
        },
        {
          id: '2',
          title: '编写技术文档',
          description: '为项目编写完整的使用文档',
          status: 'todo',
          priority: 'medium',
          created_at: Date.now() / 1000 - 172800,
          updated_at: Date.now() / 1000 - 86400,
          tags: ['文档']
        },
        {
          id: '3',
          title: '测试后端 API',
          description: '测试所有 Tauri 后端 API 接口',
          status: 'completed',
          priority: 'high',
          created_at: Date.now() / 1000 - 259200,
          updated_at: Date.now() / 1000 - 172800,
          tags: ['测试', '后端']
        }
      ]);
    } catch (error) {
      console.error('Failed to load tasks:', error);
    } finally {
      setLoading(false);
    }
  };

  const handleAddTask = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!newTitle.trim()) return;

    const newTask: Task = {
      id: Date.now().toString(),
      title: newTitle,
      description: newDescription,
      status: 'todo',
      priority: newPriority,
      due_date: newDueDate ? Math.floor(new Date(newDueDate).getTime() / 1000) : undefined,
      created_at: Date.now() / 1000,
      updated_at: Date.now() / 1000
    };
    setTasks([newTask, ...tasks]);

    setNewTitle('');
    setNewDescription('');
    setNewPriority('medium');
    setNewDueDate('');
    setShowAddModal(false);
  };

  const handleUpdateTask = async (updatedTask: Task) => {
    setTasks(tasks.map(task => task.id === updatedTask.id ? updatedTask : task));
    setEditingTask(null);
  };

  const handleDeleteTask = async (taskId: string) => {
    setTasks(tasks.filter(task => task.id !== taskId));
  };

  const handleStatusChange = async (taskId: string, newStatus: Task['status']) => {
    const task = tasks.find(t => t.id === taskId);
    if (!task) return;

    const updatedTask = {
      ...task,
      status: newStatus,
      updated_at: Date.now() / 1000
    };

    await handleUpdateTask(updatedTask);
  };

  const getFilteredTasks = () => {
    if (filterStatus === 'all') return tasks;
    return tasks.filter(task => task.status === filterStatus);
  };

  const getPriorityColor = (priority: Task['priority']) => {
    switch (priority) {
      case 'high':
        return 'var(--danger-color)';
      case 'medium':
        return '#f59e0b';
      case 'low':
        return 'var(--success-color)';
    }
  };

  const getStatusBadge = (status: Task['status']) => {
    switch (status) {
      case 'todo':
        return { text: '待办', color: '#6b7280' };
      case 'in-progress':
        return { text: '进行中', color: '#3b82f6' };
      case 'completed':
        return { text: '已完成', color: 'var(--success-color)' };
    }
  };

  const isOverdue = (task: Task) => {
    return task.due_date && task.due_date < Date.now() / 1000 && task.status !== 'completed';
  };

  const todoCount = tasks.filter(t => t.status === 'todo').length;
  const inProgressCount = tasks.filter(t => t.status === 'in-progress').length;
  const completedCount = tasks.filter(t => t.status === 'completed').length;

  return (
    <div className="tasks-page">
      <div className="page-header">
        <h1 className="page-title">✅ 待办事项</h1>
        <p className="page-subtitle">管理和跟踪您的任务</p>
      </div>

      <div className="toolbar">
        <div className="filter-tabs">
          <button
            className={`filter-tab ${filterStatus === 'all' ? 'active' : ''}`}
            onClick={() => setFilterStatus('all')}
          >
            全部 ({tasks.length})
          </button>
          <button
            className={`filter-tab ${filterStatus === 'todo' ? 'active' : ''}`}
            onClick={() => setFilterStatus('todo')}
          >
            待办 ({todoCount})
          </button>
          <button
            className={`filter-tab ${filterStatus === 'in-progress' ? 'active' : ''}`}
            onClick={() => setFilterStatus('in-progress')}
          >
            进行中 ({inProgressCount})
          </button>
          <button
            className={`filter-tab ${filterStatus === 'completed' ? 'active' : ''}`}
            onClick={() => setFilterStatus('completed')}
          >
            已完成 ({completedCount})
          </button>
        </div>

        <button
          className="add-task-button"
          onClick={() => setShowAddModal(true)}
        >
          + 新建任务
        </button>
      </div>

      {loading && (
        <div className="loading-state">
          <div className="spinner"></div>
          <p>加载任务列表...</p>
        </div>
      )}

      {!loading && (
        <div className="tasks-list">
          {getFilteredTasks().length === 0 ? (
            <div className="empty-state">
              <p>没有找到任务</p>
              <button onClick={() => setShowAddModal(true)}>创建第一个任务</button>
            </div>
          ) : (
            <div className="tasks-container">
              {getFilteredTasks().map(task => {
                const statusBadge = getStatusBadge(task.status);
                const overdue = isOverdue(task);

                return (
                  <div key={task.id} className="task-card">
                    <div className="task-header">
                      <div className="task-title">
                        <input
                          type="checkbox"
                          checked={task.status === 'completed'}
                          onChange={() => {
                            const newStatus = task.status === 'completed' ? 'todo' : 'completed';
                            handleStatusChange(task.id, newStatus);
                          }}
                          className="task-checkbox"
                        />
                        <h3
                          className={task.status === 'completed' ? 'completed-title' : ''}
                        >
                          {task.title}
                          {overdue && <span className="overdue-badge">已过期</span>}
                        </h3>
                      </div>

                      <div className="task-meta">
                        <span
                          className="priority-badge"
                          style={{ backgroundColor: getPriorityColor(task.priority) }}
                        >
                          {task.priority}
                        </span>
                        <span
                          className="status-badge"
                          style={{ backgroundColor: statusBadge.color }}
                        >
                          {statusBadge.text}
                        </span>
                      </div>
                    </div>

                    {task.description && (
                      <p className="task-description">{task.description}</p>
                    )}

                    <div className="task-footer">
                      {task.due_date && (
                        <span className={`due-date ${overdue ? 'overdue' : ''}`}>
                          📅 {new Date(task.due_date * 1000).toLocaleDateString()}
                        </span>
                      )}

                      {task.tags && task.tags.length > 0 && (
                        <div className="task-tags">
                          {task.tags.map(tag => (
                            <span key={tag} className="tag">#{tag}</span>
                          ))}
                        </div>
                      )}

                      <div className="task-actions">
                        <button
                          className="action-button"
                          onClick={() => setEditingTask(task)}
                          title="编辑"
                        >
                          ✏️
                        </button>
                        <button
                          className="action-button delete"
                          onClick={() => {
                            if (confirm('确定要删除这个任务吗？')) {
                              handleDeleteTask(task.id);
                            }
                          }}
                          title="删除"
                        >
                          🗑️
                        </button>
                      </div>
                    </div>
                  </div>
                );
              })}
            </div>
          )}
        </div>
      )}

      {showAddModal && (
        <div className="modal-overlay">
          <div className="modal">
            <div className="modal-header">
              <h2>新建任务</h2>
              <button className="close-button" onClick={() => setShowAddModal(false)}>
                ✕
              </button>
            </div>
            <form onSubmit={handleAddTask}>
              <div className="form-group">
                <label>标题 *</label>
                <input
                  type="text"
                  value={newTitle}
                  onChange={(e) => setNewTitle(e.target.value)}
                  placeholder="任务标题"
                  required
                />
              </div>
              <div className="form-group">
                <label>描述</label>
                <textarea
                  value={newDescription}
                  onChange={(e) => setNewDescription(e.target.value)}
                  placeholder="任务描述"
                  rows={3}
                />
              </div>
              <div className="form-group">
                <label>优先级</label>
                <select
                  value={newPriority}
                  onChange={(e) => setNewPriority(e.target.value as Task['priority'])}
                >
                  <option value="low">低</option>
                  <option value="medium">中</option>
                  <option value="high">高</option>
                </select>
              </div>
              <div className="form-group">
                <label>截止日期</label>
                <input
                  type="date"
                  value={newDueDate}
                  onChange={(e) => setNewDueDate(e.target.value)}
                />
              </div>
              <div className="modal-actions">
                <button type="button" onClick={() => setShowAddModal(false)}>
                  取消
                </button>
                <button type="submit" disabled={!newTitle.trim()}>
                  创建任务
                </button>
              </div>
            </form>
          </div>
        </div>
      )}

      {editingTask && (
        <div className="modal-overlay">
          <div className="modal">
            <div className="modal-header">
              <h2>编辑任务</h2>
              <button className="close-button" onClick={() => setEditingTask(null)}>
                ✕
              </button>
            </div>
            <form
              onSubmit={(e) => {
                e.preventDefault();
                handleUpdateTask(editingTask);
              }}
            >
              <div className="form-group">
                <label>标题 *</label>
                <input
                  type="text"
                  value={editingTask.title}
                  onChange={(e) => setEditingTask({ ...editingTask, title: e.target.value })}
                  required
                />
              </div>
              <div className="form-group">
                <label>描述</label>
                <textarea
                  value={editingTask.description}
                  onChange={(e) => setEditingTask({ ...editingTask, description: e.target.value })}
                  rows={3}
                />
              </div>
              <div className="form-group">
                <label>优先级</label>
                <select
                  value={editingTask.priority}
                  onChange={(e) => setEditingTask({ ...editingTask, priority: e.target.value as Task['priority'] })}
                >
                  <option value="low">低</option>
                  <option value="medium">中</option>
                  <option value="high">高</option>
                </select>
              </div>
              <div className="form-group">
                <label>截止日期</label>
                <input
                  type="date"
                  value={editingTask.due_date ? new Date(editingTask.due_date * 1000).toISOString().split('T')[0] : ''}
                  onChange={(e) => setEditingTask({
                    ...editingTask,
                    due_date: e.target.value ? Math.floor(new Date(e.target.value).getTime() / 1000) : undefined
                  })}
                />
              </div>
              <div className="modal-actions">
                <button type="button" onClick={() => setEditingTask(null)}>
                  取消
                </button>
                <button type="submit">
                  保存修改
                </button>
              </div>
            </form>
          </div>
        </div>
      )}

      <style>{`
        .tasks-page {
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

        .toolbar {
          display: flex;
          justify-content: space-between;
          align-items: center;
          margin-bottom: 24px;
          flex-wrap: wrap;
          gap: 16px;
        }

        .filter-tabs {
          display: flex;
          gap: 8px;
          background: var(--card-bg);
          padding: 4px;
          border-radius: 8px;
          border: 1px solid var(--border-color);
        }

        .filter-tab {
          padding: 8px 16px;
          background: transparent;
          border: none;
          border-radius: 6px;
          font-size: 13px;
          cursor: pointer;
          transition: all 0.2s;
          color: var(--text-secondary);
        }

        .filter-tab:hover {
          background: var(--border-color);
        }

        .filter-tab.active {
          background: var(--primary-color);
          color: white;
        }

        .add-task-button {
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

        .add-task-button:hover {
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

        .tasks-container {
          display: flex;
          flex-direction: column;
          gap: 16px;
        }

        .task-card {
          background: var(--card-bg);
          border: 1px solid var(--border-color);
          border-radius: 12px;
          padding: 20px;
          transition: all 0.2s;
        }

        .task-card:hover {
          border-color: var(--primary-color);
        }

        .task-header {
          display: flex;
          justify-content: space-between;
          align-items: flex-start;
          margin-bottom: 12px;
        }

        .task-title {
          display: flex;
          align-items: flex-start;
          gap: 12px;
          flex: 1;
        }

        .task-checkbox {
          width: 20px;
          height: 20px;
          margin-top: 2px;
          cursor: pointer;
          accent-color: var(--primary-color);
        }

        .task-title h3 {
          flex: 1;
          font-size: 16px;
          font-weight: 600;
          margin: 0;
          line-height: 1.5;
        }

        .completed-title {
          text-decoration: line-through;
          color: var(--text-secondary);
        }

        .overdue-badge {
          margin-left: 8px;
          background: var(--danger-color);
          color: white;
          font-size: 11px;
          padding: 2px 8px;
          border-radius: 10px;
        }

        .task-meta {
          display: flex;
          gap: 8px;
          align-items: center;
        }

        .priority-badge,
        .status-badge {
          padding: 4px 10px;
          border-radius: 10px;
          font-size: 11px;
          color: white;
          font-weight: 500;
        }

        .task-description {
          color: var(--text-secondary);
          font-size: 14px;
          line-height: 1.6;
          margin: 0 0 16px 0;
        }

        .task-footer {
          display: flex;
          justify-content: space-between;
          align-items: center;
          flex-wrap: wrap;
          gap: 12px;
          padding-top: 16px;
          border-top: 1px solid var(--border-color);
        }

        .due-date {
          font-size: 13px;
          color: var(--text-secondary);
        }

        .due-date.overdue {
          color: var(--danger-color);
          font-weight: 500;
        }

        .task-tags {
          display: flex;
          gap: 6px;
        }

        .tag {
          background: var(--border-color);
          padding: 4px 8px;
          border-radius: 6px;
          font-size: 12px;
          color: var(--text-secondary);
        }

        .task-actions {
          display: flex;
          gap: 8px;
        }

        .action-button {
          background: transparent;
          border: 1px solid var(--border-color);
          padding: 6px 12px;
          border-radius: 6px;
          font-size: 13px;
          cursor: pointer;
          transition: all 0.2s;
        }

        .action-button:hover {
          border-color: var(--primary-color);
        }

        .action-button.delete:hover {
          border-color: var(--danger-color);
        }

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
          max-width: 500px;
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
        .form-group textarea,
        .form-group select {
          width: 100%;
          padding: 10px 12px;
          background: var(--bg-color);
          border: 1px solid var(--border-color);
          border-radius: 6px;
          color: var(--text-primary);
          font-size: 14px;
        }

        .form-group textarea {
          resize: vertical;
          font-family: inherit;
        }

        .form-group input:focus,
        .form-group textarea:focus,
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
