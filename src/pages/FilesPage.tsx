import React, { useState, useEffect } from 'react';

interface FileItem {
  id: string;
  name: string;
  path: string;
  size: number;
  type: 'file' | 'folder';
  modified_at: number;
  extension?: string;
  preview?: string;
}

export const FilesPage: React.FC = () => {
  const [files, setFiles] = useState<FileItem[]>([]);
  const [filteredFiles, setFilteredFiles] = useState<FileItem[]>([]);
  const [searchQuery, setSearchQuery] = useState('');
  const [loading, setLoading] = useState(false);
  const [currentPath, setCurrentPath] = useState('/');
  const [selectedFile, setSelectedFile] = useState<FileItem | null>(null);
  const [viewMode, setViewMode] = useState<'grid' | 'list'>('grid');

  useEffect(() => {
    loadFiles();
  }, [currentPath]);

  useEffect(() => {
    filterFiles();
  }, [searchQuery, files]);

  const loadFiles = async () => {
    setLoading(true);
    try {
      // TODO: 从后端 API 加载文件列表
      // const result = await invoke<FileItem[]>('get_files', { path: currentPath });
      // setFiles(result);

      // 模拟数据
      setFiles([
        {
          id: '1',
          name: 'Documents',
          path: '/Documents',
          size: 0,
          type: 'folder',
          modified_at: Date.now() / 1000 - 86400
        },
        {
          id: '2',
          name: 'Images',
          path: '/Images',
          size: 0,
          type: 'folder',
          modified_at: Date.now() / 1000 - 172800
        },
        {
          id: '3',
          name: 'Notes.md',
          path: '/Notes.md',
          size: 1024,
          type: 'file',
          extension: 'md',
          modified_at: Date.now() / 1000 - 3600,
          preview: '# 我的笔记\n\n这是一些重要的笔记内容...'
        },
        {
          id: '4',
          name: 'Project Plan.pdf',
          path: '/Project Plan.pdf',
          size: 2048576,
          type: 'file',
          extension: 'pdf',
          modified_at: Date.now() / 1000 - 7200
        },
        {
          id: '5',
          name: 'Data.xlsx',
          path: '/Data.xlsx',
          size: 524288,
          type: 'file',
          extension: 'xlsx',
          modified_at: Date.now() / 1000 - 86400
        },
        {
          id: '6',
          name: 'image.png',
          path: '/image.png',
          size: 1048576,
          type: 'file',
          extension: 'png',
          modified_at: Date.now() / 1000 - 259200
        }
      ]);
    } catch (error) {
      console.error('Failed to load files:', error);
    } finally {
      setLoading(false);
    }
  };

  const filterFiles = () => {
    if (!searchQuery) {
      setFilteredFiles(files);
      return;
    }

    const query = searchQuery.toLowerCase();
    const filtered = files.filter(file =>
      file.name.toLowerCase().includes(query) ||
      file.path.toLowerCase().includes(query)
    );
    setFilteredFiles(filtered);
  };

  const handleSearch = (e: React.ChangeEvent<HTMLInputElement>) => {
    setSearchQuery(e.target.value);
  };

  const handleFileClick = (file: FileItem) => {
    if (file.type === 'folder') {
      setCurrentPath(file.path);
    } else {
      setSelectedFile(file);
    }
  };

  const handleBack = () => {
    if (currentPath !== '/') {
      const parentPath = currentPath.split('/').slice(0, -1).join('/') || '/';
      setCurrentPath(parentPath);
    }
  };

  const handleClosePreview = () => {
    setSelectedFile(null);
  };

  const formatFileSize = (bytes: number) => {
    if (bytes === 0) return '-';
    const k = 1024;
    const sizes = ['B', 'KB', 'MB', 'GB'];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return Math.round(bytes / Math.pow(k, i) * 100) / 100 + ' ' + sizes[i];
  };

  const getFileIcon = (file: FileItem) => {
    if (file.type === 'folder') return '📁';
    switch (file.extension) {
      case 'md':
        return '📝';
      case 'pdf':
        return '📄';
      case 'xlsx':
      case 'xls':
        return '📊';
      case 'png':
      case 'jpg':
      case 'jpeg':
      case 'gif':
        return '🖼️️';
      case 'txt':
        return '📃';
      default:
        return '📎';
    }
  };

  return (
    <div className="files-page">
      <div className="page-header">
        <h1 className="page-title">📁 文件管理</h1>
        <p className="page-subtitle">浏览和管理您的个人文件</p>
      </div>

      {/* 路径导航 */}
      <div className="path-navigation">
        <button
          className="back-button"
          onClick={handleBack}
          disabled={currentPath === '/'}
        >
          ← 返回
        </button>
        <div className="current-path">
          <span>当前位置: </span>
          <code>{currentPath}</code>
        </div>
      </div>

      {/* 搜索栏 */}
      <div className="search-section">
        <input
          type="text"
          className="search-input"
          placeholder="搜索文件..."
          value={searchQuery}
          onChange={handleSearch}
        />
        <div className="view-mode-toggle">
          <button
            className={`view-button ${viewMode === 'grid' ? 'active' : ''}`}
            onClick={() => setViewMode('grid')}
            title="网格视图"
          >
            ⊞
          </button>
          <button
            className={`view-button ${viewMode === 'list' ? 'active' : ''}`}
            onClick={() => setViewMode('list')}
            title="列表视图"
          >
            ☰
          </button>
        </div>
      </div>

      {/* 加载状态 */}
      {loading && (
        <div className="loading-state">
          <div className="spinner"></div>
          <p>加载文件列表...</p>
        </div>
      )}

      {/* 文件列表 */}
      {!loading && (
        <div className="files-container">
          {filteredFiles.length === 0 ? (
            <div className="empty-state">
              <p>没有找到文件</p>
              {searchQuery && <button onClick={() => setSearchQuery('')}>清除搜索</button>}
            </div>
          ) : viewMode === 'grid' ? (
            <div className="files-grid">
              {filteredFiles.map(file => (
                <div
                  key={file.id}
                  className="file-card"
                  onClick={() => handleFileClick(file)}
                >
                  <div className="file-icon">{getFileIcon(file)}</div>
                  <div className="file-info">
                    <h3 className="file-name">{file.name}</h3>
                    <p className="file-meta">
                      {file.type === 'folder' ? '文件夹' : formatFileSize(file.size)}
                    </p>
                  </div>
                </div>
              ))}
            </div>
          ) : (
            <div className="files-list">
              <table className="files-table">
                <thead>
                  <tr>
                    <th>名称</th>
                    <th>类型</th>
                    <th>大小</th>
                    <th>修改时间</th>
                  </tr>
                </thead>
                <tbody>
                  {filteredFiles.map(file => (
                    <tr
                      key={file.id}
                      className="file-row"
                      onClick={() => handleFileClick(file)}
                    >
                      <td className="file-name-cell">
                        <span className="row-icon">{getFileIcon(file)}</span>
                        <span>{file.name}</span>
                      </td>
                      <td>{file.type === 'folder' ? '文件夹' : file.extension || '-'}</td>
                      <td>{formatFileSize(file.size)}</td>
                      <td>{new Date(file.modified_at * 1000).toLocaleString()}</td>
                    </tr>
                  ))}
                </tbody>
              </table>
            </div>
          )}
        </div>
      )}

      {/* 文件预览面板 */}
      {selectedFile && (
        <div className="preview-panel">
          <div className="preview-header">
            <h2>{selectedFile.name}</h2>
            <button className="close-button" onClick={handleClosePreview}>
              ✕
            </button>
          </div>
          <div className="preview-content">
            <div className="preview-meta">
              <p><strong>路径:</strong> {selectedFile.path}</p>
              <p><strong>大小:</strong> {formatFileSize(selectedFile.size)}</p>
              <p><strong>类型:</strong> {selectedFile.extension || '未知'}</p>
              <p><strong>修改时间:</strong> {new Date(selectedFile.modified_at * 1000).toLocaleString()}</p>
            </div>

            {selectedFile.extension === 'md' && selectedFile.preview && (
              <div className="markdown-preview">
                <h3>内容预览</h3>
                <pre>{selectedFile.preview}</pre>
              </div>
            )}

            {selectedFile.extension?.match(/^(png|jpg|jpeg|gif)$/i) && (
              <div className="image-preview">
                <h3>图片预览</h3>
                <div className="image-placeholder">
                  <span>🖼️️</span>
                  <p>图片预览功能</p>
                  <p className="note">（需要后端支持）</p>
                </div>
              </div>
            )}

            {!selectedFile.preview && selectedFile.extension !== 'md' && (
              <div className="no-preview">
                <p>此文件类型暂不支持预览</p>
                <button onClick={handleClosePreview}>关闭</button>
              </div>
            )}
          </div>
        </div>
      )}

      <style>{`
        .files-page {
          padding: 20px;
        }

        .page-header {
          margin-bottom: 24px;
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

        .path-navigation {
          display: flex;
          align-items: center;
          gap: 16px;
          margin-bottom: 20px;
          padding: 12px;
          background: var(--card-bg);
          border: 1px solid var(--border-color);
          border-radius: 8px;
        }

        .back-button {
          background: var(--primary-color);
          border: none;
          padding: 8px 16px;
          border-radius: 6px;
          color: white;
          font-size: 13px;
          cursor: pointer;
          transition: background 0.2s;
        }

        .back-button:hover:not(:disabled) {
          background: var(--primary-hover);
        }

        .back-button:disabled {
          opacity: 0.5;
          cursor: not-allowed;
        }

        .current-path {
          flex: 1;
          color: var(--text-secondary);
          font-size: 14px;
        }

        .current-path code {
          background: var(--bg-color);
          padding: 4px 8px;
          border-radius: 4px;
          color: var(--text-primary);
        }

        .search-section {
          display: flex;
          gap: 12px;
          margin-bottom: 24px;
        }

        .search-input {
          flex: 1;
          padding: 10px 16px;
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

        .view-mode-toggle {
          display: flex;
          gap: 4px;
        }

        .view-button {
          padding: 10px 12px;
          background: var(--card-bg);
          border: 1px solid var(--border-color);
          color: var(--text-secondary);
          border-radius: 6px;
          cursor: pointer;
          transition: all 0.2s;
        }

        .view-button:hover {
          border-color: var(--primary-color);
        }

        .view-button.active {
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

        /* Grid View */
        .files-grid {
          display: grid;
          grid-template-columns: repeat(auto-fill, minmax(150px, 1fr));
          gap: 20px;
        }

        .file-card {
          background: var(--card-bg);
          border: 1px solid var(--border-color);
          border-radius: 12px;
          padding: 20px;
          text-align: center;
          cursor: pointer;
          transition: all 0.2s;
        }

        .file-card:hover {
          border-color: var(--primary);
          transform: translateY(-4px);
        }

        .file-icon {
          font-size: 48px;
          margin-bottom: 12px;
        }

        .file-info {
          min-height: 40px;
        }

        .file-name {
          font-size: 14px;
          font-weight: 500;
          margin: 0 0 4px 0;
          word-break: break-word;
        }

        .file-meta {
          font-size: 12px;
          color: var(--text-secondary);
          margin: 0;
        }

        /* List View */
        .files-list {
          background: var(--card-bg);
          border: 1px solid var(--border-color);
          border-radius: 8px;
          overflow: hidden;
        }

        .files-table {
          width: 100%;
          border-collapse: collapse;
        }

        .files-table thead {
          background: var(--border-color);
        }

        .files-table th {
          padding: 12px 16px;
          text-align: left;
          font-size: 13px;
          font-weight: 600;
          color: var(--text-secondary);
        }

        .file-row {
          border-bottom: 1px solid var(--border-color);
          cursor: pointer;
          transition: background 0.2s;
        }

        .file-row:hover {
          background: var(--border-color);
        }

        .file-row:last-child {
          border-bottom: none;
        }

        .files-table td {
          padding: 12px 16px;
          font-size: 14px;
        }

        .file-name-cell {
          display: flex;
          align-items: center;
          gap: 8px;
        }

        .row-icon {
          font-size: 20px;
        }

        /* Preview Panel */
        .preview-panel {
          position: fixed;
          top: 0;
          right: 0;
          width: 600px;
          height: 100vh;
          background: var(--sidebar-bg);
          border-left: 1px solid var(--border-color);
          display: flex;
          flex-direction: column;
          z-index: 1000;
        }

        .preview-header {
          display: flex;
          justify-content: space-between;
          align-items: center;
          padding: 24px;
          border-bottom: 1px solid var(--border-color);
        }

        .preview-header h2 {
          font-size: 18px;
          font-weight: 600;
          margin: 0;
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

        .preview-content {
          flex: 1;
          padding: 24px;
          overflow-y: auto;
        }

        .preview-meta {
          background: var(--card-bg);
          border: 1px solid var(--border-color);
          border-radius: 8px;
          padding: 16px;
          margin-bottom: 24px;
        }

        .preview-meta p {
          margin-bottom: 8px;
          font-size: 13px;
          color: var(--text-secondary);
        }

        .preview-meta p:last-child {
          margin-bottom: 0;
        }

        .preview-meta strong {
          font-weight: 600;
          color: var(--text-primary);
        }

        .markdown-preview,
        .image-preview {
          background: var(--card-bg);
          border: 1px solid var(--border-color);
          border-radius: 8px;
          padding: 20px;
        }

        .markdown-preview h3,
        .image-preview h3 {
          font-size: 16px;
          font-weight: 600;
          margin-bottom: 16px;
        }

        .markdown-preview pre {
          background: var(--bg-color);
          padding: 12px;
          border-radius: 6px;
          overflow-x: auto;
          font-size: 13px;
          line-height: 1.6;
          white-space: pre-wrap;
        }

        .image-placeholder {
          text-align: center;
          padding: 40px;
        }

        .image-placeholder span {
          font-size: 64px;
          display: block;
          margin-bottom: 12px;
        }

        .image-placeholder p {
          margin: 8px 0;
          color: var(--text-secondary);
        }

        .image-placeholder .note {
          font-size: 12px;
          opacity: 0.7;
        }

        .no-preview {
          text-align: center;
          padding: 40px;
          color: var(--text-secondary);
        }

        .no-preview button {
          margin-top: 16px;
          background: var(--primary-color);
          border: none;
          padding: 8px 16px;
          border-radius: 6px;
          color: white;
          cursor: pointer;
        }
      `}</style>
    </div>
  );
};
