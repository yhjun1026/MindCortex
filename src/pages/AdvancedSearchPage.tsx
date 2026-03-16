import React, { useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { SearchBar } from '../components/SearchBar';
import { SearchResultsEnhanced } from '../components/SearchResultsEnhanced';

interface SearchResult {
  id: string;
  content: string;
  score: number;
  metadata: SearchResultMetadata;
}

interface SearchResultMetadata {
  source: string;
  timestamp: number;
  session_id?: string;
  message_id?: string;
  file_path?: string;
  content_length?: number;
}

interface SearchConfig {
  searchType: 'hybrid' | 'keyword' | 'semantic';
  enableTimeFilter: boolean;
  filters: SearchFilters;
}

interface SearchFilters {
  source?: string;
  file_path?: string;
  tags?: string[];
  time_range?: {
    start: number;
    end: number;
  };
  score_min?: number;
  score_max?: number;
}

export const AdvancedSearchPage: React.FC = () => {
  const [query, setQuery] = useState('');
  const [searchConfig, setSearchConfig] = useState<SearchConfig>({
    searchType: 'hybrid',
    enableTimeFilter: true,
    filters: {},
  });
  const [results, setResults] = useState<SearchResult[]>([]);
  const [isSearching, setIsSearching] = useState(false);
  const [searchHistory, setSearchHistory] = useState<string[]>([]);
  const [selectedResult, setSelectedResult] = useState<SearchResult | null>(null);

  const handleSearch = async () => {
    if (!query.trim()) return;

    setIsSearching(true);
    setResults([]);

    try {
      const searchResults = await invoke<SearchResult[]>('hybrid_search', {
        query,
        searchType: searchConfig.searchType,
        filters: searchConfig.filters,
      });

      setResults(searchResults);

      // 更新搜索历史
      const newHistory = [query, ...searchHistory].slice(0, 10);
      setSearchHistory(newHistory);

      // 保存搜索历史
      await invoke('save_search_history', { history: newHistory });
    } catch (error) {
      console.error('Search failed:', error);
    } finally {
      setIsSearching(false);
    }
  };

  const handleSearchTypeChange = (type: 'hybrid' | 'keyword' | 'semantic') => {
    setSearchConfig(prev => ({ ...prev, searchType: type }));
  };

  const handleResultSelect = (result: SearchResult) => {
    setSelectedResult(result);
  };

  const exportResults = () => {
    const data = JSON.stringify(results, null, 2);
    const blob = new Blob([data], { type: 'application/json' });
    const url = URL.createObjectURL(blob);

    const link = document.createElement('a');
    link.href = url;
    link.download = `search-results-${Date.now()}.json`;
    link.click();

    URL.revokeObjectURL(url);
  };

  const exportResultsCSV = () => {
    const headers = 'ID,Content,Score,Source,Timestamp\n';
    const rows = results.map(r =>
      `"${r.id}","${r.content.replace(/"/g, '""')}","${r.score.toFixed(4)}","${r.metadata.source}","${r.metadata.timestamp}"`
    ).join('\n');

    const csv = headers + rows;
    const blob = new Blob([csv], { type: 'text/csv' });
    const url = URL.createObjectURL(blob);

    const link = document.createElement('a');
    link.href = url;
    link.download = `search-results-${Date.now()}.csv`;
    link.click();

    URL.revokeObjectURL(url);
  };

  return (
    <div className="advanced-search-page">
      <div className="page-header">
        <h1 className="page-title">🔍 高级搜索</h1>
        <p className="page-subtitle">
          混合搜索（关键词 + 语义）、智能过滤和结果分析
        </p>
      </div>

      <div className="search-config">
        <div className="config-section">
          <label>搜索类型</label>
          <div className="search-type-options">
            <button
              className={`type-button ${searchConfig.searchType === 'hybrid' ? 'active' : ''}`}
              onClick={() => handleSearchTypeChange('hybrid')}
            >
              混合搜索
            </button>
            <button
              className={`type-button ${searchConfig.searchType === 'keyword' ? 'active' : ''}`}
              onClick={() => handleSearchTypeChange('keyword')}
            >
              关键词搜索
            </button>
            <button
              className={`type-button ${searchConfig.searchType === 'semantic' ? 'active' : ''}`}
              onClick={() => handleSearchTypeChange('semantic')}
            >
              语义搜索
            </button>
          </div>
        </div>
      </div>

      <div className="search-section">
        <SearchBar onSearch={handleSearch} placeholder="输入关键词或自然语言搜索..." />
      </div>

      {isSearching && (
        <div className="searching-indicator">
          <div className="spinner"></div>
          <p>正在搜索中...</p>
          <p className="searching-hint">
            {searchConfig.searchType === 'hybrid' && '执行关键词 + 语义混合搜索'}
            {searchConfig.searchType === 'keyword' && '执行关键词精确搜索'}
            {searchConfig.searchType === 'semantic' && '执行语义向量搜索'}
          </p>
        </div>
      )}

      {!isSearching && searchHistory.length > 0 && results.length === 0 && (
        <div className="search-history-section">
          <div className="history-header">
            <h3>搜索历史</h3>
            <button
              className="clear-history-button"
              onClick={() => setSearchHistory([])}
            >
              清空
            </button>
          </div>
          <div className="history-list">
            {searchHistory.map((search, index) => (
              <button
                key={index}
                className="history-item"
                onClick={() => setQuery(search)}
              >
                {search}
              </button>
            ))}
          </div>
        </div>
      )}

      {!isSearching && results.length > 0 && (
        <>
          <div className="results-actions">
            <div className="results-stats">
              <span>找到 {results.length} 条结果</span>
              <span>
                平均分数: {
                  (results.reduce((sum, r) => sum + r.score, 0) / results.length * 100).toFixed(1)
                }%
              </span>
            </div>
            <div className="export-buttons">
              <button className="export-button" onClick={exportResults}>
                导出 JSON
              </button>
              <button className="export-button" onClick={exportResultsCSV}>
                导出 CSV
              </button>
            </div>
          </div>

          <SearchResultsEnhanced
            results={results}
            onSelect={handleResultSelect}
          />
        </>
      )}

      {!isSearching && results.length === 0 && query && (
        <div className="no-results">
          <div className="no-results-icon">🔍</div>
          <p className="no-results-text">
            没有找到与 "{query}" 相关的内容
          </p>
          <p className="search-tips">
            <li>💡 尝试使用不同的关键词</li>
            <li>💡 添加更多知识到索引中</li>
            <li>💡 检查搜索过滤设置</li>
          </p>
        </div>
      )}

      {selectedResult && (
        <div className="result-detail-panel">
          <div className="detail-panel-header">
            <h3>详细信息</h3>
            <button
              className="close-panel-button"
              onClick={() => setSelectedResult(null)}
            >
              ✕
            </button>
          </div>
          <div className="detail-panel-content">
            <div className="detail-field">
              <label>ID:</label>
              <span>{selectedResult.id}</span>
            </div>
            {selectedResult.metadata.session_id && (
              <div className="detail-field">
                <label>会话 ID:</label>
                <span>{selectedResult.metadata.session_id}</span>
              </div>
            )}
            {selectedResult.metadata.message_id && (
              <div className="detail-field">
                <label>消息 ID:</label>
                <span>{selectedResult.metadata.message_id}</span>
              </div>
            )}
            {selectedResult.metadata.file_path && (
              <div className="detail-field">
                <label>文件路径:</label>
                <span>{selectedResult.metadata.file_path}</span>
              </div>
            )}
            {selectedResult.metadata.content_length && (
              <div className="detail-field">
                <label>内容长度:</label>
                <span>{selectedResult.metadata.content_length} 字符</span>
              </div>
            )}
            <div className="detail-content">
              <h4>内容</h4>
              <pre>{selectedResult.content}</pre>
            </div>
          </div>
        </div>
      )}

      <style>{`
        .advanced-search-page {
          min-height: 100vh;
          padding: 40px 20px;
          background: #f9f9f9;
        }

        .page-header {
          text-align: center;
          margin-bottom: 40px;
        }

        .page-title {
          font-size: 32px;
          font-weight: 600;
          margin: 0 0 12px 0;
          color: #333;
        }

        .page-subtitle {
          font-size: 16px;
          color: #666;
          margin: 0;
        }

        .search-config {
          background: white;
          border-radius: 8px;
          padding: 20px;
          margin-bottom: 24px;
          box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
        }

        .config-section {
          margin-bottom: 16px;
        }

        .config-section label {
          display: block;
          font-size: 14px;
          font-weight: 500;
          color: #333;
          margin-bottom: 8px;
        }

        .search-type-options {
          display: flex;
          gap: 8px;
        }

        .type-button {
          padding: 8px 16px;
          border: 2px solid #e0e0e0;
          background: white;
          border-radius: 6px;
          cursor: pointer;
          font-size: 14px;
          font-weight: 500;
          transition: all 0.2s;
        }

        .type-button.active {
          background: #4a90e2;
          color: white;
          border-color: #4a90e2;
        }

        .type-button:hover:not(.active) {
          background: #f5f5f5;
          border-color: #d0d0d0;
        }

        .search-section {
          margin-bottom: 32px;
        }

        .searching-indicator {
          text-align: center;
          padding: 60px 40px;
          background: white;
          border-radius: 8px;
          margin-top: 20px;
          box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
        }

        .spinner {
          width: 40px;
          height: 40px;
          border: 3px solid #f3f3f3;
          border-top-color: #4a90e2;
          border-right-color: transparent;
          border-bottom-color: transparent;
          border-radius: 50%;
          animation: spin 1s linear infinite;
          margin: 0 auto 16px;
        }

        @keyframes spin {
          0% { transform: rotate(0deg); }
          100% { transform: rotate(360deg); }
        }

        .searching-indicator p {
          margin: 0;
          font-size: 16px;
          color: #666;
        }

        .searching-hint {
          font-size: 14px;
          color: #999;
          margin-bottom: 0;
        }

        .search-history-hint {
          font-size: 14px;
          color: #999;
        }

        .search-history-section {
          background: white;
          border-radius: 8px;
          padding: 20px;
          margin-top: 20px;
          box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
        }

        .history-header {
          display: flex;
          justify-content: space-between;
          align-items: center;
;
          margin-bottom: 16px;
        }

        .history-header h3 {
          margin: 0;
          font-size: 16px;
          font-weight: 600;
          color: #333;
        }

        .clear-history-button {
          background: transparent;
          border: none;
          color: #999;
          font-size: 14px;
          cursor: pointer;
        }

        .clear-history-button:hover {
          color: #4a90e2;
        }

        .history-list {
          display: flex;
          flex-wrap: wrap;
          gap: 8px;
        }

        .history-item {
          background: #f5f5f5;
          border: 1px solid #e0e0e0;
          padding: 8px 16px;
          border-radius: 16px;
          cursor: pointer;
          font-size: 14px;
          color: #333;
          transition: all 0.2s;
        }

        .history-item:hover {
          background: #e8f4ff;
          border-color: #4a90e2;
          color: #4a90e2;
        }

        .results-actions {
          display: flex;
          justify-content: space-between;
          align-items: center;
          margin-bottom: 24px;
          padding: 16px;
          background: white;
          border-radius: 8px;
          box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
        }

        .results-stats {
          display: flex;
          align-items: center;
          gap: 16px;
          font-size: 14px;
          color: #666;
        }

        .export-buttons {
          display: flex;
          gap: 12px;
        }

        .export-button {
          padding: 8px 16px;
          background: #4a90e2;
          color: white;
          border: none;
          border-radius: 6px;
          cursor: pointer;
          font-size: 14px;
          font-weight: 500;
          transition: background 0.2s;
        }

        .export-button:hover {
          background: #357abd;
        }

        .no-results {
          text-align: center;
          padding: 60px 40px;
          background: white;
          border-radius: 8px;
          margin-top: 20px;
          box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
        }

        .no-results-icon {
          font-size: 48px;
          margin-bottom: 16px;
        }

        .no-results-text {
          font-size: 18px;
          font-weight: 500;
          margin: 0 0 12px 0;
          color: #333;
        }

        .search-tips {
          font-size: 14px;
          color: #999;
          line-height: 1.8;
          margin: 16px 0 8px 0;
        }

        .search-tips li {
          margin-bottom: 4px;
        }

        .result-detail-panel {
          position: fixed;
          top: 0;
          right: 0;
          width: 400px;
          height: 100vh;
          background: white;
          box-shadow: -4px 0 16px rgba(0, 0, 0, 0.15);
          z-index: 1000;
          display: flex;
          flex-direction: column;
        }

        .detail-panel-header {
          display: flex;
          justify-content: space-between;
          align-items: center;
          padding: 20px;
          border-bottom: 2px solid #f0f0f0;
        }

        .detail-panel-header h3 {
          margin: 0;
          font-size: 18px;
          font-weight: 600;
        }

        .close-panel-button {
          background: transparent;
          border: none;
          font-size: 24px;
          cursor: pointer;
          color: #666;
          padding: 0;
        }

        .close-panel-button:hover {
          color: #333;
        }

        .detail-panel-content {
          flex: 1;
          padding: 20px;
          overflow-y: auto;
        }

        .detail-field {
          margin-bottom: 12px;
          display: flex;
        }

        .detail-field label {
          font-weight: 500;
          color: #666;
          margin-right: 16px;
          min-width: 100px;
        }

        .detail-field span {
          flex: 1;
          color: #333;
          word-break: break-word;
        }

        .detail-source-label {
          margin-left: 4px;
          padding: 4px 8px;
          background: #f5f5f5;
          border-radius: 4px;
          font-size: 13px;
          color: #666;
        }

        .detail-content {
          margin-top: 20px;
        }

        .detail-content h4 {
          margin: 0 0 12px 0;
          font-size: 16px;
          font-weight: 600;
        }

        .detail-content pre {
          background: #f5f5f5;
          padding: 12px;
          border-radius: 6px;
          overflow-x: auto;
          font-size: 13px;
          line-height: 1.6;
          white-space: pre-wrap;
        }
      `}</style>
    </div>
  );
};
