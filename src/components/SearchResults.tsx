import React, { useState } from 'react';

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

interface SearchResultsProps {
  results: SearchResult[];
  onSelect?: (result: SearchResult) => void;
  maxResults?: number;
}

export const SearchResults: React.FC<SearchResultsProps> = ({ 
  results, 
  onSelect,
  maxResults 
}) => {
  const [expandedIds, setExpandedIds] = useState<Set<string>>(new Set());
  
  // 限制显示结果数量
  const displayResults = maxResults ? results.slice(0, maxResults) : results;

  const toggleExpand = (id: string) => {
    setExpandedIds(prev => {
      const newSet = new Set(prev);
      if (newSet.has(id)) {
        newSet.delete(id);
      } else {
        newSet.add(id);
      }
      return newSet;
    });
  };

  const formatScore = (score: number) => {
    return (score * 100).toFixed(1);
  };

  const formatTimestamp = (timestamp: number) => {
    const date = new Date(timestamp * 1000);
    return date.toLocaleDateString('zh-CN', {
      year: 'numeric',
      month: 'short',
      day: 'numeric',
      hour: '2-digit',
      minute: '2-digit'
    });
  };

  const getSourceIcon = (source: string) => {
    const icons: Record<string, string> = {
      opencode: '💻',
      claudecode: '🤖',
      cursor: '🎯',
      file: '📄',
      memory: '🧠',
    };
    return icons[source] || '📝';
  };

  const getSourceLabel = (source: string) => {
    const labels: Record<string, string> = {
      opencode: 'OpenCode',
      claudecode: 'Claude Code',
      cursor: 'Cursor',
      file: '文件',
      memory: '记忆',
    };
    return labels[source] || source;
  };

  if (results.length === 0) {
    return (
      <div className="search-results-empty">
        <div className="empty-icon">🔍</div>
        <div className="empty-message">
          没有找到相关内容
        </div>
        <div className="empty-hint">
          尝试使用不同的关键词或添加更多知识到索引中
        </div>
        <style>{`
          .search-results-empty {
            text-align: center;
            padding: 40px 20px;
            color: #666;
          }

          .empty-icon {
            font-size: 48px;
            margin-bottom: 16px;
          }

          .empty-message {
            font-size: 18px;
            font-weight: 500;
            margin-bottom: 8px;
          }

          .empty-hint {
            font-size: 14px;
            color: #999;
          }
        `}</style>
      </div>
    );
  }

  return (
    <div className="search-results">
      <div className="results-header">
        <h3 className="results-title">
          搜索结果 ({results.length} 条)
        </h3>
        {maxResults && results.length > maxResults && (
          <span className="results-truncated">
            显示前 {maxResults} 条
          </span>
        )}
      </div>

      <div className="results-list">
        {displayResults.map((result) => {
          const isExpanded = expandedIds.has(result.id);
          const truncatedContent = result.content.slice(0, 200);
          const needsTruncation = result.content.length > 200;

          return (
            <div
              key={result.id}
              className="result-item"
              onClick={() => onSelect?.(result)}
            >
              <div className="result-header">
                <div className="result-source">
                  <span className="source-icon">
                    {getSourceIcon(result.metadata.source)}
                  </span>
                  <span className="source-label">
                    {getSourceLabel(result.metadata.source)}
                  </span>
                </div>
                <div className="result-score">
                  相似度 {formatScore(result.score)}%
                </div>
              </div>

              <div className="result-content">
                {needsTruncation && !isExpanded ? (
                  <>
                    <p className="content-text">
                      {truncatedContent}...
                    </p>
                    <button
                      className="expand-button"
                      onClick={(e) => {
                        e.stopPropagation();
                        toggleExpand(result.id);
                      }}
                    >
                      展开更多
                    </button>
                  </>
                ) : (
                  <p className="content={needsTruncation ? 'expanded' : ''}">
                    {result.content}
                  </p>
                )}
              </div>

              <div className="result-footer">
                {result.metadata.session_id && (
                  <span className="result-meta">
                    会话: {result.metadata.session_id.slice(0, 8)}...
                  </span>
                )}
                <span className="result-timestamp">
                  {formatTimestamp(result.metadata.timestamp)}
                </span>
              </div>
            </div>
          );
        })}
      </div>

      <style>{`
        .search-results {
          width: 100%;
          max-width: 800px;
          margin: 0 auto;
        }

        .results-header {
          display: flex;
          justify-content: space-between;
          align-items: center;
          margin-bottom: 20px;
          padding-bottom: 12px;
          border-bottom: 2px solid #f0f0f0;
        }

        .results-title {
          font-size: 18px;
          font-weight: 600;
          margin: 0;
        }

        .results-truncated {
          font-size: 14px;
          color: #999;
        }

        .results-list {
          display: flex;
          flex-direction: column;
          gap: 16px;
        }

        .result-item {
          background: white;
          border-radius: 8px;
          padding: 16px;
          box-shadow: 0 1px 4px rgba(0, 0, 0, 0.1);
          cursor: pointer;
          transition: all 0.2s;
        }

        .result-item:hover {
          box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
          transform: translateY(-1px);
        }

        .result-header {
          display: flex;
          justify-content: space-between;
          align-items: center;
          margin-bottom: 12px;
        }

        .result-source {
          display: flex;
          align-items: center;
          gap: 8px;
        }

        .source-icon {
          font-size: 16px;
        }

        .source-label {
          font-size: 14px;
          font-weight: 500;
          color: #666;
        }

        .result-score {
          font-size: 14px;
          font-weight: 500;
          color: #4a90e2;
          background: #e8f4ff;
          padding: 4px 12px;
          border-radius: 12px;
        }

        .result-content {
          margin-bottom: 12px;
        }

        .content-text {
          font-size: 14px;
          line-height: 1.6;
          color: #333;
          margin: 0;
          white-space: pre-wrap;
        }

        .content-text.expanded {
          white-space: pre-wrap;
        }

        .expand-button {
          background: transparent;
          border: none;
          color: #4a90e2;
          font-size: 14px;
          cursor: pointer;
          padding: 0;
          margin-top: 8px;
        }

        .expand-button:hover {
          text-decoration: underline;
        }

        .result-footer {
          display: flex;
          justify-content: space-between;
          align-items: center;
          font-size: 12px;
          color: #999;
        }

        .result-meta {
          background: #f5f5f5;
          padding: 2px 8px;
          border-radius: 4px;
        }

        .result-timestamp {
          font-size: 12px;
          color: #999;
        }
      `}</style>
    </div>
  );
};
