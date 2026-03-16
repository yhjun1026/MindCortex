/**
 * MindCortex VSCode 扩展类型定义
 */

/**
 * 搜索结果类型
 */
export interface SearchResult {
    id: string;
    title: string;
    content: string;
    type: 'code' | 'doc' | 'snippet';
    language?: string;
    filePath?: string;
    score: number;
    metadata?: Record<string, any>;
}

/**
 * 搜索选项
 */
export interface SearchOptions {
    query: string;
    filters?: {
        code?: boolean;
        docs?: boolean;
        snippets?: boolean;
    };
    maxResults?: number;
}

/**
 * 代码片段
 */
export interface CodeSnippet {
    id: string;
    title: string;
    description: string;
    code: string;
    language: string;
    tags: string[];
    createdAt: number;
    usageCount: number;
}

/**
 * 最佳实践
 */
export interface BestPractice {
    id: string;
    title: string;
    description: string;
    language: string;
    category: string;
    content: string;
    examples: string[];
}

/**
 * 问题解决方案
 */
export interface Solution {
    id: string;
    title: string;
    problem: string;
    solution: string;
    language: string;
    tags: string[];
    relatedSnippets: string[];
}

/**
 * 会话
 */
export interface Session {
    id: string;
    name: string;
    description?: string;
    language?: string;
    createdAt: number;
    lastUsedAt: number;
    tags: string[];
    metadata: Record<string, any>;
}

/**
 * Webview 消息类型
 */
export interface WebviewMessage {
    type: 'search' | 'assistant' | 'graph' | 'ready' | 'initialized';
    [key: string]: any;
}

/**
 * 搜索消息
 */
export interface SearchMessage extends WebviewMessage {
    type: 'search';
    query: string;
    filters?: {
        code?: boolean;
        docs?: boolean;
        snippets?: boolean;
    };
}

/**
 * 代码助手消息
 */
export interface AssistantMessage extends WebviewMessage {
    type: 'assistant';
    action: string;
    [key: string]: any;
}

/**
 * 知识图谱消息
 */
export interface GraphMessage extends WebviewMessage {
    type: 'graph';
    action: string;
    [key: string]: any;
}

/**
 * 扩展配置
 */
export interface ExtensionConfig {
    apiEndpoint: string;
    autoIndex: boolean;
    indexInterval: number;
    enableCodeAssistant: boolean;
    maxSearchResults: number;
    debug: boolean;
}

/**
 * 知识图谱节点
 */
export interface GraphNode {
    id: string;
    label: string;
    type: string;
    properties: Record<string, any>;
    x?: number;
    y?: number;
}

/**
 * 知识图谱边
 */
export interface GraphEdge {
    id: string;
    source: string;
    target: string;
    label?: string;
    weight?: number;
    properties?: Record<string, any>;
}

/**
 * 知识图谱数据
 */
export interface GraphData {
    nodes: GraphNode[];
    edges: GraphEdge[];
}
