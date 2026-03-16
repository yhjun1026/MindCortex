import React, { useState, useEffect } from 'react';

interface MarkdownEditorProps {
  value?: string;
  onChange?: (value: string) => void;
  placeholder?: string;
  readOnly?: boolean;
  height?: string;
}

export const MarkdownEditor: React.FC<MarkdownEditorProps> = ({
  value = '',
  onChange,
  placeholder = '开始输入 Markdown...',
  readOnly = false,
  height = '400px'
}) => {
  const [content, setContent] = useState(value);
  const [previewMode, setPreviewMode] = useState<'split' | 'edit' | 'preview'>('split');

  useEffect(() => {
    setContent(value);
  }, [value]);

  const handleContentChange = (e: React.ChangeEvent<HTMLTextAreaElement>) => {
    const newContent = e.target.value;
    setContent(newContent);
    if (onChange) {
      onChange(newContent);
    }
  };

  // 简单的 Markdown 解析器（仅支持基本语法）
  const parseMarkdown = (markdown: string): string => {
    let html = markdown;

    // 转义 HTML 特殊字符
    html = html.replace(/&/g, '&amp;');
    html = html.replace(/</g, '&lt;');
    html = html.replace(/>/g, '&gt;');

    // 代码块
    html = html.replace(/```(\w*)\n([\s\S]*?)```/g, '<pre><code class="language-$1">$2</code></pre>');

    // 行内代码
    html = html.replace(/`([^`]+)`/g, '<code>$1</code>');

    // 标题
    html = html.replace(/^### (.*$)/gm, '<h3>$1</h3>');
    html = html.replace(/^## (.*$)/gm, '<h2>$1</h2>');
    html = html.replace(/^# (.*$)/gm, '<h1>$1</h1>');

    // 粗体和斜体
    html = html.replace(/\*\*\*(.+?)\*\*\*/g, '<strong><em>$1</em></strong>');
    html = html.replace(/\*\*(.+?)\*\*/g, '<strong>$1</strong>');
    html = html.replace(/\*(.+?)\*/g, '<em>$1</em>');

    // 删除线
    html = html.replace(/~~(.+?)~~/g, '<del>$1</del>');

    // 分隔线
    html = html.replace(/^---$/gm, '<hr />');

    // 引用
    html = html.replace(/^> (.*$)/gm, '<blockquote>$1</blockquote>');

    // 无序列表
    html = html.replace(/^\* (.*$)/gm, '<li>$1</li>');
    html = html.replace(/^- (.*$)/gm, '<li>$1</li>');
    html = html.replace(/<\/li>\n<li>/g, '</li><li>');
    html = html.replace(/(<li>.*<\/li>)/s, '<ul>$1</ul>');

    // 有序列表
    html = html.replace(/^\d+\. (.*$)/gm, '<li>$1</li>');

    // 链接
    html = html.replace(/\[([^\]]+)\]\(([^)]+)\)/g, '<a href="$2" target="_blank">$1</a>');

    // 图片
    html = html.replace(/!\[([^\]]*)\]\(([^)]+)\)/g, '<img src="$2" alt="$1" />');

    // 换行
    html = html.replace(/\n/g, '<br />');

    return html;
  };

  const renderedPreview = parseMarkdown(content);

  return (
    <div className="markdown-editor">
      <div className="editor-toolbar">
        <div className="toolbar-group">
          <button
            className={`toolbar-button ${previewMode === 'edit' ? 'active' : ''}`}
            onClick={() => setPreviewMode('edit')}
            title="编辑模式"
          >
            ✏️ 编辑
          </button>
          <button
            className={`toolbar-button ${previewMode === 'split' ? 'active' : ''}`}
            onClick={() => setPreviewMode('split')}
            title="分屏模式"
          >
            ⬜ 分屏
          </button>
          <button
            className={`toolbar-button ${previewMode === 'preview' ? 'active' : ''}`}
            onClick={() => setPreviewMode('preview')}
            title="预览模式"
          >
            👁️ 预览
          </button>
        </div>

        {!readOnly && (
          <div className="toolbar-group">
            <button
              className="toolbar-button"
              onClick={() => {
                const textarea = document.querySelector('.markdown-textarea') as HTMLTextAreaElement;
                if (textarea) {
                  const start = textarea.selectionStart;
                  const end = textarea.selectionEnd;
                  const text = textarea.value;
                  const newText = text.substring(0, start) + '**粗体**' + text.substring(end);
                  setContent(newText);
                  if (onChange) onChange(newText);
                  setTimeout(() => {
                    textarea.focus();
                    textarea.selectionStart = textarea.selectionEnd = start + 2;
                  }, 0);
                }
              }}
              title="粗体"
            >
              **B**
            </button>
            <button
              className="toolbar-button"
              onClick={() => {
                const textarea = document.querySelector('.markdown-textarea') as HTMLTextAreaElement;
                if (textarea) {
                  const start = textarea.selectionStart;
                  const end = textarea.selectionEnd;
                  const text = textarea.value;
                  const newText = text.substring(0, start) + '*斜体*' + text.substring(end);
                  setContent(newText);
                  if (onChange) onChange(newText);
                  setTimeout(() => {
                    textarea.focus();
                    textarea.selectionStart = textarea.selectionEnd = start + 1;
                  }, 0);
                }
              }}
              title="斜体"
            >
              *I*
            </button>
            <button
              className="toolbar-button"
              onClick={() => {
                const textarea = document.querySelector('.markdown-textarea') as HTMLTextAreaElement;
                if (textarea) {
                  const start = textarea.selectionStart;
                  const end = textarea.selectionEnd;
                  const text = textarea.value;
                  const selectedText = text.substring(start, end);
                  const newText = text.substring(0, start) + `[\`${selectedText || '链接文本'}\`](url)` + text.substring(end);
                  setContent(newText);
                  if (onChange) onChange(newText);
                }
              }}
              title="链接"
            >
              🔗
            </button>
            <button
              className="toolbar-button"
              onClick={() => {
                const textarea = document.querySelector('.markdown-textarea') as HTMLTextAreaElement;
                if (textarea) {
                  const start = textarea.selectionStart;
                  const end = textarea.selectionEnd;
                  const text = textarea.value;
                  const selectedText = text.substring(start, end);
                  const newText = text.substring(0, start) + `\`\`\`\n${selectedText || '代码'}\n\`\`\`` + text.substring(end);
                  setContent(newText);
                  if (onChange) onChange(newText);
                }
              }}
              title="代码块"
            >
              &lt;/&gt;
            </button>
            <button
              className="toolbar-button"
              onClick={() => {
                const textarea = document.querySelector('.markdown-textarea') as HTMLTextAreaElement;
                if (textarea) {
                  const start = textarea.selectionStart;
                  const end = textarea.selectionEnd;
                  const text = textarea.value;
                  const newText = text.substring(0, start) + '\n---\n' + text.substring(end);
                  setContent(newText);
                  if (onChange) onChange(newText);
                }
              }}
              title="分隔线"
            >
              —
            </button>
          </div>
        )}
      </div>

      <div
        className={`editor-content editor-mode-${previewMode}`}
        style={{ height }}
      >
        {(previewMode === 'edit' || previewMode === 'split') && (
          <div className="editor-pane edit-pane">
            <textarea
              className="markdown-textarea"
              value={content}
              onChange={handleContentChange}
              placeholder={placeholder}
              readOnly={readOnly}
            />
          </div>
        )}

        {(previewMode === 'preview' || previewMode === 'split') && (
          <div className="editor-pane preview-pane">
            <div
              className="markdown-preview"
              dangerouslySetInnerHTML={{ __html: renderedPreview || '<p class="empty">预览区域</p>' }}
            />
          </div>
        )}
      </div>

      <style>{`
        .markdown-editor {
          display: flex;
          flex-direction: column;
          border: 1px solid var(--border-color);
          border-radius: 8px;
          overflow: hidden;
          background: var(--bg-color);
        }

        .editor-toolbar {
          display: flex;
          justify-content: space-between;
          align-items: center;
          padding: 8px 12px;
          background: var(--card-bg);
          border-bottom: 1px solid var(--border-color);
          flex-wrap: wrap;
          gap: 8px;
        }

        .toolbar-group {
          display: flex;
          gap: 4px;
        }

        .toolbar-button {
          background: transparent;
          border: 1px solid transparent;
          padding: 6px 10px;
          border-radius: 4px;
          font-size: 13px;
          cursor: pointer;
          transition: all 0.2s;
          color: var(--text-secondary);
        }

        .toolbar-button:hover {
          border-color: var(--border-color);
          color: var(--text-primary);
        }

        .toolbar-button.active {
          background: var(--primary-color);
          border-color: var(--primary-color);
          color: white;
        }

        .editor-content {
          display: flex;
          overflow: hidden;
        }

        .editor-mode-edit,
        .editor-mode-preview {
          flex: 1;
        }

        .editor-mode-split .editor-pane {
          flex: 1;
        }

        .editor-pane {
          overflow: auto;
        }

        .edit-pane {
          border-right: 1px solid var(--border-color);
        }

        .markdown-textarea {
          width: 100%;
          height: 100%;
          padding: 16px;
          background: var(--bg-color);
          border: none;
          color: var(--text-primary);
          font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', 'Consolas', monospace;
          font-size: 14px;
          line-height: 1.6;
          resize: none;
          outline: none;
        }

        .markdown-textarea::placeholder {
          color: var(--text-secondary);
          opacity: 0.5;
        }

        .markdown-preview {
          padding: 16px;
          color: var(--text-primary);
          font-size: 15px;
          line-height: 1.8;
          overflow-wrap: break-word;
        }

        .markdown-preview.empty {
          color: var(--text-secondary);
          font-style: italic;
        }

        /* Preview Styles */
        .markdown-preview h1,
        .markdown-preview h2,
        .markdown-preview h3,
        .markdown-preview h4,
        .markdown-preview h5,
        .markdown-preview h6 {
          margin: 24px 0 16px 0;
          font-weight: 600;
          line-height: 1.3;
        }

        .markdown-preview h1 {
          font-size: 28px;
          border-bottom: 2px solid var(--border-color);
          padding-bottom: 8px;
        }

        .markdown-preview h2 {
          font-size: 24px;
          border-bottom: 1px solid var(--border-color);
          padding-bottom: 6px;
        }

        .markdown-preview h3 {
          font-size: 20px;
        }

        .markdown-preview p {
          margin: 12px 0;
        }

        .markdown-preview code {
          background: var(--border-color);
          padding: 2px 6px;
          border-radius: 4px;
          font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', 'Consolas', monospace;
          font-size: 13px;
        }

        .markdown-preview pre {
          background: var(--card-bg);
          border: 1px solid var(--border-color);
          border-radius: 8px;
          padding: 16px;
          margin: 16px 0;
          overflow-x: auto;
        }

        .markdown-preview pre code {
          background: transparent;
          padding: 0;
          border-radius: 0;
          font-size: 14px;
          line-height: 1.6;
        }

        .markdown-preview blockquote {
          border-left: 4px solid var(--primary-color);
          padding-left: 16px;
          margin: 16px 0;
          color: var(--text-secondary);
          font-style: italic;
        }

        .markdown-preview ul,
        .markdown-preview ol {
          padding-left: 24px;
          margin: 12px 0;
        }

        .markdown-preview li {
          margin: 6px 0;
          line-height: 1.6;
        }

        .markdown-preview a {
          color: var(--primary-color);
          text-decoration: underline;
          cursor: pointer;
        }

        .markdown-preview a:hover {
          color: var(--primary-hover);
        }

        .markdown-preview img {
          max-width: 100%;
          height: auto;
          border-radius: 8px;
          margin: 16px 0;
        }

        .markdown-preview hr {
          border: none;
          border-top: 2px solid var(--border-color);
          margin: 24px 0;
        }

        .markdown-preview del {
          color: var(--text-secondary);
          text-decoration: line-through;
        }

        .markdown-preview strong {
          font-weight: 600;
        }

        .markdown-preview em {
          font-style: italic;
        }
      `}</style>
    </div>
  );
};
