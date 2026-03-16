// 搜索结果组件
// 带有搜索结果分组、筛选和显示功能

import React, { useState, useEffect } from 'react';

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

interface SearchStats {
  total: number;
  source_counts: Record<string, number>;
  type_counts: Record<string, number>;
  average_score: number;
}

export const SearchResults: React.FC<{
  results: SearchResult[];
  onSelect?: (result: SearchResult) => void;
  maxResults?: number;
  filters?: SearchFilters;
  showStats?: boolean;
}> = ({ 
  results, 
  onSelect, 
  maxResults,
  filters: defaultFilters,
  showStats = true,
}) => {
  const [expandedIds, setExpandedIds] = useState<Set<string>>(new Set());
  const [groupBy, setGroupBy] = useState<'source' | 'none' | 'time'>('none');
  const [sortBy, setSortBy] = useState<'score' | 'time'>('score');

  // 应用过滤
  const filteredResults = results.filter(result => {
    if (filters?.source && result.metadata.source !== filters.source) {
      return false;
    }
    if (filters?.file_path && result.metadata.file_path) {
      const filePath = result.metadata.file_path.toLowerCase();
      const filterPath = filters.file_path.toLowerCase();
      if (!filePath.includes(filterPath)) {
        return false;
      }
    }
    if (filters?.time_range) {
      const timestamp = result.metadata.timestamp;
      if (timestamp < filters.time_range.start || timestamp > filters.time_range.end) {
        return false;
      }
    }
    if (filters?.score_min && result.score < filters.score_min) {
      return false;
    }
    if (filters?.score_max && result.score > filters.score_max) {
      return false;
    }
    return true;
  });

  // 分组结果
  const groupedResults = React.useMemo(() => {
    if (groupBy === 'none') {
      return { 'all': filteredResults };
    }
    
    const groups: Record<string, SearchResult[]> = {};
    
    for (const result of filteredResults) {
      const key = groupBy === 'source' ? result.metadata.source : 
                 groupBy === 'time' ? 
                   new Date(result.metadata.timestamp * 1000).toLocaleDateString('zh-CN', {
                     year: 'numeric',
                     month: 'short',
                     day: 'numeric',
                     hour: '2-digit',
                     minute: '2-digit'
                   }) : 'unknown';
      
      if (!groups[key]) {
        groups[key] = [];
      }
      groups[key].push(result);
    }
    
    return groups;
  }, [filteredResults, groupBy]);

  // 排序结果
  const sortedResults = React.useMemo(() => {
    if (sortBy === 'none') return filteredResults;
    
    return filteredResults.sort((a, b) => {
      if (sortBy === 'score') {
        return b.score - a.score;
      } else if (sortBy === 'time') {
        return b.metadata.timestamp - a.metadata.timestamp;
      }
      return 0;
    });
  }, [filteredResults, sortBy]);

  // 限制结果数量
  const displayResults = maxResults ? sortedResults.slice(0, maxResults) : sortedResults;

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
    return date.toLocaleString('zh-CN', {
      year: 'numeric',
      month: 'short',
      day: 'numeric',
      hour: '2-digit',
      minute: '2-digit',
    });
  };

  const getSourceIcon = (source: string) => {
    const icons: Record<string, string> = {
      opencode: '💻',
      claudecode: '🤖',
      cursor: '🎯',
      file: '📄',
      memory: '🧠',
      unknown: '📝',
    };
    return icons[source] || icons.unknown;
  };

  // 计算统计信息
  const stats = React.useMemo(() => {
    const sourceCounts: Record<string, number> = {};
    const typeCounts: Record<string, number> = {};
    let totalScore = 0;

    filteredResults.forEach(result => {
      const source = result.metadata.source;
      source_counts[source] = (source_counts[source] || 0) + 1;
      
      const type = 'result'; // 可以根据需要改为其他类型
      typeCounts[type] = (typeCounts[type] || 0) + 1;
      totalScore += result.score;
    });

    const averageScore = filteredResults.length > 0 
      ? totalScore / filteredResults.length 
      : 0;

    return {
      total: filteredResults.length,
      source_counts,
      type_counts,
      average_score,
    };
  }, [filteredResults]);

  return (
    <div className="search-results">
      {/* 统计信息 */}
      {showStats && stats && (
        <div className="results-stats">
          <div className="stats-item">
            <span className="stats-label">总数:</span>
            <span className="stats-value">{stats.total}</span>
          </div>
          <div className="stats-item">
            <span className="stats-label">平均分数:</span>
            <span className="stats-value">{formatScore(stats.average_score)}%</span>
          </div>
          <div className="stats-item">
            <span className="stats-label">来源分布:</span>
            <span className="stats-value">
              {Object.entries(stats.source_counts).map(([source, count]) => (
                <span key={source}>
                  {getSourceIcon(source)} {source}: {count}
                </span>
              ))}
            </span>
          </div>
        </div>
      )}

      {/* 无结果 */}
      {displayResults.length === 0 && (
        <div className="empty-state">
          <div className="empty-icon">🔍</div>
          <div className="empty-message">没有找到相关内容</div>
          <div className="empty-hint">
            尝试使用不同的关键词或添加更多知识到索引中
          </div>
        </div>
      )}

      {/* 分组显示结果 */}
      {displayResults.length > 0 && (
        <div className="results-container">
          {Object.entries(groupedResults).map(([groupName, groupResults]) => (
            <div key={groupName} className="result-group">
              <div className="group-header">
                <h3 className="group-title">
                  {groupName === 'all' ? '搜索结果' : groupName}
                </h3>
                <span className="group-count">
                  ({groupResults.length} 条)
                </span>
              </div>

              <div className="group-results">
                {groupResults.map((result) => {
                  const isExpanded = expandedIds.has(result.id);

                  return (
                    <div
                      key={result.id}
                      className={`result-item ${isExpanded ? 'expanded' : ''}`}
                      onClick={() => onSelect?.(result)}
                    >
                      <div className="result-header">
                        <div className="result-source">
                          <span className="source-icon">
                            {getSourceIcon(result.metadata.source)}
                          </span>
                          <span className="source-label">
                            {result.metadata.source}
                          </span>
                        </div>
                        <div className="result-score">
                          {formatScore(result.score)}%
                        </div>
                      </div>

                      <div className="result-content">
                        {isExpanded ? (
                          <pre className="content-text-expanded">
                            {result.content}
                          </pre>
                        ) : (
                          <p className="content-text">
                            {result.content.slice(0, 200)}
                            {result.content.length > 200 && '...'}
                          </p>
                        )}
                        {result.content.length > 200 && !isExpanded && (
                          <button
                            className="expand-button"
                            onClick={() => toggleExpand(result.id)}
                          >
                            展开更多
                          </button>
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
            </div>
          ))}
        </div>
      )}
    </div>
  );
};
