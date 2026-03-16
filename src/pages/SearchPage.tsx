import React, { useState, useEffect } from 'react';
import { SearchBar } from '../components/SearchBar';
import { SearchResults } from '../components/SearchResults';
import { invoke } from '@tauri-apps/api/core';

interface SearchResult {
  id: string;
  content: string;
  score: number;
  metadata: {
    source: string;
    timestamp: number;
    session_id?: string;
    message_id?: string;
    file_path?: string;
  };
}

export const SearchPage: React.FC = () => {
  const [results, setResults] = useState<SearchResult[]>([]);
  const [isSearching, setIsSearching] = useState(false);
  const [searchHistory, setSearchHistory] = useState<string[]>([]);
  const [selectedResult, setSelectedResult] = useState<SearchResult | null>(null);

  // 加载搜索历史
  useEffect(() => {
    loadSearchHistory();
  }, []);

  const loadSearchHistory = async () => {
    try {
      // TODO: 从本地存储加载搜索历史
      // const history = await invoke('get_search_history');
      // setSearchHistory(history);
    } catch (error) {
      console.error('Failed to load search history:', error);
    }
  };

  const handleSearch = async (query: string) => {
    setIsSearching(true);
    setResults([]);

    try {
      // 调用后端搜索 API
      const searchResults = await invoke<SearchResult[]>('search_knowledge', {
        query,
        topK: 10,
      });

      setResults(searchResults);

      // 更新搜索历史
      const newHistory = [query, ...searchHistory].slice(0, 10);
      setSearchHistory(newHistory);

      // TODO: 保存搜索历史
      // await invoke('save_search_history', { history: newHistory });
    } catch (error) {
      console.error('Search failed:', error);
      // TODO: 显示错误提示
    } finally {
      setIsSearching(false);
    }
  };

  const handleResultSelect = (result: SearchResult) => {
    setSelectedResult(result);
    // TODO: 打开详细视图或侧边栏
  };

  const clearHistory = async () => {
    setSearchHistory([]);
    // TODO: 清除本地存储的搜索历史
  };

  return (
    <div className="search-page">
      <div className="search-page-header">
        <h1 className="page-title">知识搜索</h1>
        <p className="page-subtitle">
          在您的 AI 对话和文档中快速找到相关信息
        </p>
      </div>

      <div className="search-page-content">
        <SearchBar
          onSearch={handleSearch}
          placeholder="输入关键词搜索您的知识库..."
        />

        {isSearching && (
          <div className="searching-indicator">
            <div className="spinner"></div>
            <p>正在搜索...</p>
          </div>
        )}

        {!isSearching && results.length === 0 && searchHistory.length > 0 && (
          <div className="search-history-section">
            <div className="history-header">
              <h3>搜索历史</h3>
              <button
                className="clear-history-button"
                onClick={clearHistory}
              >
                清除
              </button>
            </div>
            <div className="history-list">
              {searchHistory.map((query, index) => (
                <button
                  key={index}
                  className="history-item"
                  onClick={() => handleSearch(query)}
                >
                  {query}
                </button>
              ))}
            </div>
          </div>
        )}

        {!isSearching && results.length > 0 && (
          <SearchResults
            results={results}
            onSelect={handleResultSelect}
          />
        )}
      </div>

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
            <div className="detail-source">
              <span>来源:</span>
              <span>{selectedResult.metadata.source}</span>
            </div>
            <div className="detail-timestamp">
              <span>时间:</span>
              <span>
                {new Date(selectedResult.metadata.timestamp * 1000).toLocaleString('zh-CN')}
              </span>
            </div>
            {selectedResult.metadata.session_id && (
              <div className="detail-session">
                <span>会话 ID:</span>
                <span>{selectedResult.metadata.session_id}</span>
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
        .search-page {
          min-height: 100vh;
          padding: 40px 20px;
          background: #f9f9f9;
        }

        .search-page-header {
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

        .search-page-content {
          max-width: 800px;
          margin: 0 auto;
        }

        .searching-indicator {
          text-align: center;
          padding: 40px;
          background: white;
          border-radius: 8px;
          margin-top: 20px;
          box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
        }

        .spinner {
          width: 40px;
          height: 40px;
          border: 3px solid #f3f3f3;
          border-top: 3px solid #4a90e2;
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
          color: #666;
          font-size: 16px;
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
          margin-bottom: 16px;
        }

        .history-header h3 {
          margin: 0;
          font-size: 16px;
          font-weight: 600;
        }

        .clear-history-button {
          background: transparent;
          border: none;
          color: #666;
          cursor: pointer;
          font-size: 14px;
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
          border: none;
          padding: 8px 16px;
          border-radius: 16px;
          cursor: pointer;
          font-size: 14px;
          color: #333;
          transition: all 0.2s;
        }

        .history-item:hover {
          background: #4a90e2;
          color: white;
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

        .detail-source,
        .detail-timestamp,
        .detail-session {
          display: flex;
          justify-content: space-between;
          margin-bottom: 12px;
          font-size: 14px;
        }

        .detail-source span:first-child,
        .detail-timestamp span:first-child,
        .detail-session span:first-child {
          font-weight: 500;
          color: #666;
        }

        .detail-content {
          margin-top: 20px;
        }

        .detail-content h4 {
          margin: 0 0 12px;
          font-size: 16px;
          font-weight: 600;
        }

        .detail-content pre {
          background: #f5f5f5;
;
          padding: 12px;
          border-radius: 6px;
          overflow-x: auto;
          font-size: 13px;
          line-height: 1.6;
        }
      `}</style>
    </div>
  );
};
