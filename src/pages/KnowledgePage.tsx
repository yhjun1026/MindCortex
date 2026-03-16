import React, { useState, useEffect } from 'react';

interface KnowledgeItem {
  id: string;
  title: string;
  content: string;
  tags: string[];
  source: string;
  created_at: number;
  updated_at: number;
}

export const KnowledgePage: React.FC = () => {
  const [items, setItems] = useState<KnowledgeItem[]>([]);
  const [filteredItems, setFilteredItems] = useState<KnowledgeItem[]>([]);
  const [searchQuery, setSearchQuery] = useState('');
  const [selectedTag, setSelectedTag] = useState<string | null>(null);
  const [loading, setLoading] = useState(false);
  const [selectedItem, setSelectedItem] = useState<KnowledgeItem | null>(null);

  useEffect(() => {
    loadKnowledgeItems();
  }, []);

  useEffect(() => {
    filterItems();
  }, [searchQuery, selectedTag, items]);

  const loadKnowledgeItems = async () => {
    setLoading(true);
    try {
      // TODO: 从后端 API 加载知识库项目
      // const result = await invoke<KnowledgeItem[]>('get_knowledge_items');
      // setItems(result);

      // 模拟数据
      setItems([
        {
          id: '1',
          title: 'React Hooks 最佳实践',
          content: 'React Hooks 是 React 16.8 引入的新特性，它允许你在不编写 class 的情况下使用 state 以及其他的 React 特性。',
          tags: ['React', 'Frontend', 'Best Practices'],
          source: 'AI Chat Session',
          created_at: Date.now() / 1000 - 86400,
          updated_at: Date.now() / 1000
        },
        {
          id: '2',
          title: 'TypeScript 类型系统',
          content: 'TypeScript 是 JavaScript 的超集，添加了静态类型系统。它提供了接口、类型别名、泛型等强大功能。',
          tags: ['TypeScript', 'Language', 'Types'],
          source: 'AI Chat Session',
          created_at: Date.now() / 1000 - 172800,
          updated_at: Date.now() / 1000 - 86400
        },
        {
          id: '3',
          title: 'Tauri 应用开发',
          content: 'Tauri 是一个使用 Web 前端构建应用的框架，它使用 Rust 作为后端，比 Electron 更轻量高效。',
          tags: ['Tauri', 'Rust', 'Development'],
          source: 'Documentation',
          created_at: Date.now() / 1000 - 259200,
          updated_at: Date.now() / 1000 - 172800
        }
      ]);
    } catch (error) {
      console.error('Failed to load knowledge items:', error);
    } finally {
      setLoading(false);
    }
  };

  const filterItems = () => {
    let filtered = items;

    // 按搜索查询过滤
    if (searchQuery) {
      const query = searchQuery.toLowerCase();
      filtered = filtered.filter(item =>
        item.title.toLowerCase().includes(query) ||
        item.content.toLowerCase().includes(query) ||
        item.tags.some(tag => tag.toLowerCase().includes(query))
      );
    }

    // 按标签过滤
    if (selectedTag) {
      filtered = filtered.filter(item => item.tags.includes(selectedTag));
    }

    setFilteredItems(filtered);
  };

  const getAllTags = () => {
    const tags = new Set<string>();
    items.forEach(item => item.tags.forEach(tag => tags.add(tag)));
    return Array.from(tags);
  };

  const handleSearch = (e: React.ChangeEvent<HTMLInputElement>) => {
    setSearchQuery(e.target.value);
  };

  const handleTagClick = (tag: string) => {
    setSelectedTag(selectedTag === tag ? null : tag);
  };

  const handleItemClick = (item: KnowledgeItem) => {
    setSelectedItem(item);
  };

  return (
    <div className="knowledge-page">
      <div className="page-header">
        <h1 className="page-title">🧠 知识库</h1>
        <p className="page-subtitle">管理和浏览您的 AI 知识积累</p>
      </div>

      {/* 搜索栏 */}
      <div className="search-section">
        <input
          type="text"
          className="search-input"
          placeholder="搜索知识内容..."
          value={searchQuery}
          onChange={handleSearch}
        />
      </div>

      {/* 标签过滤 */}
      {getAllTags().length > 0 && (
        <div className="tags-section">
          <h3>标签</h3>
          <div className="tags-list">
            {getAllTags().map(tag => (
              <button
                key={tag}
                className={`tag-button ${selectedTag === tag ? 'active' : ''}`}
                onClick={() => handleTagClick(tag)}
              >
                {tag}
              </button>
            ))}
          </div>
        </div>
      )}

      {/* 加载状态 */}
      {loading && (
        <div className="loading-state">
          <div className="spinner"></div>
          <p>加载知识库...</p>
        </div>
      )}

      {/* 知识项目列表 */}
      {!loading && (
        <div className="knowledge-items">
          {filteredItems.length === 0 ? (
            <div className="empty-state">
              <p>没有找到相关知识项目</p>
              {searchQuery && <button onClick={() => setSearchQuery('')}>清除搜索</button>}
            </div>
          ) : (
            <div className="items-grid">
              {filteredItems.map(item => (
                <div
                  key={item.id}
                  className="knowledge-item-card"
                  onClick={() => handleItemClick(item)}
                >
                  <h3 className="item-title">{item.title}</h3>
                  <p className="item-preview">
                    {item.content.length > 150
                      ? item.content.substring(0, 150) + '...'
                      : item.content}
                  </p>
                  <div className="item-tags">
                    {item.tags.map(tag => (
                      <span key={tag} className="item-tag">{tag}</span>
                    ))}
                  </div>
                  <div className="item-meta">
                    <span>来源: {item.source}</span>
                    <span>更新于: {new Date(item.updated_at * 1000).toLocaleDateString()}</span>
                  </div>
                </div>
              ))}
            </div>
          )}
        </div>
      )}

      {/* 详情面板 */}
      {selectedItem && (
        <div className="detail-panel">
          <div className="detail-panel-header">
            <h2>{selectedItem.title}</h2>
            <button className="close-button" onClick={() => setSelectedItem(null)}>
              ✕
            </button>
          </div>
          <div className="detail-panel-content">
            <div className="detail-tags">
              {selectedItem.tags.map(tag => (
                <span key={tag} className="detail-tag">{tag}</span>
              ))}
            </div>
            <div className="detail-text">
              {selectedItem.content}
            </div>
            <div className="detail-meta">
              <p><strong>来源:</strong> {selectedItem.source}</p>
              <p><strong>创建时间:</strong> {new Date(selectedItem.created_at * 1000).toLocaleString()}</p>
              <p><strong>更新时间:</strong> {new Date(selectedItem.updated_at * 1000).toLocaleString()}</p>
            </div>
          </div>
        </div>
      )}

      <style>{`
        .knowledge-page {
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

        .search-section {
          margin-bottom: 24px;
        }

        .search-input {
          width: 100%;
          padding: 12px 16px;
          background: var(--bg-color);
          border: 1px solid var(--border-color);
          border-radius: 8px;
          color: var(--text-primary);
          font-size: 14px;
        }

        .search-input:focus {
          outline: none;
          border-color: var(--primary-color);
        }

        .tags-section {
          margin-bottom: 24px;
        }

        .tags-section h3 {
          font-size: 14px;
          font-weight: 600;
          margin-bottom: 12px;
          color: var(--text-secondary);
        }

        .tags-list {
          display: flex;
          flex-wrap: wrap;
          gap: 8px;
        }

        .tag-button {
          background: var(--card-bg);
          border: 1px solid var(--border-color);
          padding: 6px 12px;
          border-radius: 16px;
          font-size: 12px;
          cursor: pointer;
          transition: all 0.2s;
          color: var(--text-secondary);
        }

        .tag-button:hover {
          border-color: var(--primary-color);
        }

        .tag-button.active {
          background: var(--primary-color);
          border-color: var(--primary-color);
          color: white;
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

        .knowledge-items {
          margin-top: 24px;
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

        .items-grid {
          display: grid;
          grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
          gap: 20px;
        }

        .knowledge-item-card {
          background: var(--card-bg);
          border: 1px solid var(--border-color);
          border-radius: 12px;
          padding: 20px;
          cursor: pointer;
          transition: all 0.2s;
        }

        .knowledge-item-card:hover {
          border-color: var(--primary-color);
          transform: translateY(-2px);
        }

        .item-title {
          font-size: 18px;
          font-weight: 600;
          margin-bottom: 12px;
        }

        .item-preview {
          color: var(--text-secondary);
          font-size: 14px;
          line-height: 1.6;
          margin-bottom: 16px;
        }

        .item-tags {
          display: flex;
          flex-wrap: wrap;
          gap: 6px;
          margin-bottom: 12px;
        }

        .item-tag {
          background: var(--border-color);
          padding: 4px 10px;
          border-radius: 12px;
          font-size: 11px;
          color: var(--text-secondary);
        }

        .item-meta {
          display: flex;
          justify-content: space-between;
          font-size: 12px;
          color: var(--text-secondary);
        }

        .detail-panel {
          position: fixed;
          top: 0;
          right: 0;
          width: 500px;
          height: 100vh;
          background: var(--sidebar-bg);
          border-left: 1px solid var(--border-color);
          display: flex;
          flex-direction: column;
          z-index: 1000;
        }

        .detail-panel-header {
          display: flex;
          justify-content: space-between;
          align-items: center;
          padding: 24px;
          border-bottom: 1px solid var(--border-color);
        }

        .detail-panel-header h2 {
          font-size: 20px;
          font-weight: 600;
        }

        .close-button {
          background: transparent;
          border: none;
          font-size: 24px;
          cursor: pointer;
          color: var(--text-secondary);
          padding: 4px;
        }

        .close-button:hover {
          color: var(--text-primary);
        }

        .detail-panel-content {
          flex: 1;
          padding: 24px;
          overflow-y: auto;
        }

        .detail-tags {
          display: flex;
          flex-wrap: wrap;
          gap: 8px;
          margin-bottom: 20px;
        }

        .detail-tag {
          background: var(--primary-color);
          padding: 6px 12px;
          border-radius: 16px;
          font-size: 12px;
          color: white;
        }

        .detail-text {
          color: var(--text-primary);
          line-height: 1.8;
          margin-bottom: 24px;
          white-space: pre-wrap;
        }

        .detail-meta {
          padding-top: 20px;
          border-top: 1px solid var(--border-color);
          color: var(--text-secondary);
          font-size: 14px;
        }

        .detail-meta p {
          margin-bottom: 8px;
        }

        .detail-meta strong {
          font-weight: 600;
          color: var(--text-primary);
        }
      `}</style>
    </div>
  );
};
