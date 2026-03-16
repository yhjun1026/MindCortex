import React, { useState, useEffect, useRef } from 'react';

// 前端搜索结果接口定义（共享）
export interface SearchResult {
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

interface SearchBarProps {
  onSearch?: (query: string) => void;
  placeholder?: string;
}

export const SearchBar: React.FC<SearchBarProps> = ({ 
  onSearch, 
  placeholder = "Search your knowledge base..." 
}) => {
  const [query, setQuery] = useState('');
  const [isSearching, setIsSearching] = useState(false);
  const [suggestions, setSuggestions] = useState<string[]>([]);
  const [showSuggestions, setShowSuggestions] = useState(false);
  const inputRef = useRef<HTMLInputElement>(null);

  // 获取搜索建议
  useEffect(() => {
    if (query.length > 2) {
      fetchSuggestions(query);
    } else {
      setSuggestions([]);
      setShowSuggestions(false);
    }
  }, [query]);

  const fetchSuggestions = async (_q: string) => {
    try {
      // TODO: 实现搜索建议 API
      // const result = await invoke('get_search_suggestions', { query: q });
      // setSuggestions(result);
    } catch (error) {
      console.error('Failed to fetch suggestions:', error);
    }
  };

  const handleSearch = async (e?: React.FormEvent) => {
    if (e) {
      e.preventDefault();
    }

    if (!query.trim()) return;

    setIsSearching(true);
    try {
      if (onSearch) {
        onSearch(query);
      }
    } finally {
      setIsSearching(false);
    }
  };

  const handleSuggestionClick = (suggestion: string) => {
    setQuery(suggestion);
    setShowSuggestions(false);
    handleSearch();
  };

  return (
    <div className="search-bar-container">
      <form onSubmit={handleSearch} className="search-form">
        <div className="search-input-wrapper">
          <input
            ref={inputRef}
            type="text"
            className="search-input"
            placeholder={placeholder}
            value={query}
            onChange={(e) => setQuery(e.target.value)}
            onFocus={() => setShowSuggestions(true)}
            onBlur={() => setTimeout(() => setShowSuggestions(false), 200)}
          />
          <button
            type="submit"
            className="search-button"
            disabled={isSearching || !query.trim()}
          >
            {isSearching ? (
              <span className="searching-spinner">⏳</span>
            ) : (
              <span className="search-icon">🔍</span>
            )}
          </button>
        </div>

        {showSuggestions && suggestions.length > 0 && (
          <div className="search-suggestions">
            {suggestions.map((suggestion, index) => (
              <div
                key={index}
                className="suggestion-item"
                onClick={() => handleSuggestionClick(suggestion)}
              >
                {suggestion}
              </div>
            ))}
          </div>
        )}
      </form>

      <style>{`
        .search-bar-container {
          width: 100%;
          max-width: 800px;
          margin: 0 auto;
          position: relative;
        }

        .search-form {
          position: relative;
        }

        .search-input-wrapper {
          display: flex;
          align-items: center;
          background: #f5f5f5;
          border-radius: 8px;
          padding: 8px;
          box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
        }

        .search-input {
          flex: 1;
          border: none;
          background: transparent;
          padding: 12px 16px;
          font-size: 16px;
          outline: none;
        }

        .search-button {
          background: #4a90e2;
          color: white;
          border: none;
          padding: 12px 24px;
          border-radius: 6px;
          cursor: pointer;
          font-size: 16px;
          transition: background 0.2s;
        }

        .search-button:hover:not(:disabled) {
          background: #357abd;
        }

        .search-button:disabled {
          background: #ccc;
          cursor: not-allowed;
        }

        .search-icon, .searching-spinner {
          font-size: 18px;
        }

        .search-suggestions {
          position: absolute;
          top: 100%;
          left: 0;
          right: 0;
          background: white;
          border-radius: 8px;
          box-shadow: 0 4px 16px rgba(0, 0, 0, 0.15);
          margin-top: 8px;
          max-height: 300px;
          overflow-y: auto;
          z-index: 1000;
        }

        .suggestion-item {
          padding: 12px 16px;
          cursor: pointer;
          border-bottom: 1px solid #f0f0f0;
        }

        .suggestion-item:hover {
          background: #f5f5f5;
        }

        .suggestion-item:last-child {
          border-bottom: none;
        }
      `}</style>
    </div>
  );
};
