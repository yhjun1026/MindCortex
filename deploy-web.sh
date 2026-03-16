#!/bin/bash

# MindCortex v0.2.0 Web 部署脚本

echo "🚀 MindCortex v0.2.0 Web 部署开始..."

# 项目根目录
PROJECT_DIR="/Users/yanghongjun/.openclaw/workspace/MindCortex"
BUILD_DIR="$PROJECT_DIR/dist"
WEB_DIR="$PROJECT_DIR/web-deploy"

echo "📍 项目目录: $PROJECT_DIR"
echo "📁 构建目录: $BUILD_DIR"
echo "🌐 Web 部署目录: $WEB_DIR"

# 检查 Node.js 环境
if ! command -v node &> /dev/null; then
    echo "❌ 错误: 未安装 Node.js，请先安装"
    exit 1
fi

echo "✅ Node.js 版本: $(node --version)"
echo "✅ npm 版本: $(npm --version)"

# 停止当前运行的服务
echo "🛑 停止现有服务..."
lsof -ti :8080 | grep 'node\|npm' | xargs kill -9 2>/dev/null || true

# 构建 Web 版本
echo "📦 构建 Web 版本..."
cd "$PROJECT_DIR" || exit 1

# 清理旧的构建
echo "🧹 清理旧的构建文件..."
rm -rf "$BUILD_DIR"

# 安装依赖（如果需要）
echo "📦 检查依赖..."
if [ ! -d "node_modules" ]; then
    echo "📥 安装依赖..."
    npm install
fi

# 构建
echo "🔨 构建项目..."
npm run build

if [ $? -ne 0 ]; then
    echo "❌ 构建失败"
    exit 1
fi

echo "✅ 构建成功！"

# 创建 Web 部署目录
echo "📂 创建 Web 部署目录..."
mkdir -p "$WEB_DIR"

# 复制静态文件
echo "📋 复制静态文件到 Web 部署目录..."
cp -r "$BUILD_DIR"/* "$WEB_DIR/"

# 创建简单的 Web 服务器脚本
echo "📝 创建 Web 服务器脚本..."
cat > "$WEB_DIR/start-server.sh" << 'EOF'
#!/bin/bash

# MindCortex Web 服务器启动脚本

echo "🚀 MindCortex Web 服务器启动中..."
echo "📍 访问地址: http://localhost:3000"
echo "按 Ctrl+C 停止服务器"

# 使用 http-server 提供静态文件
npx http-server -p 3000 -o -c-1 --silent
EOF

chmod +x "$WEB_DIR/start-server.sh"

# 创建启动说明
echo "📝 创建启动说明..."
cat > "$WEB_DIR/README.md" << 'EOF'
# MindCortex v0.2.0 Web 版本

## 📦 启动方式

### 方式 1: 使用内置脚本（推荐）
```bash
./start-server.sh
```

访问地址: http://localhost:3000

### 方式 2: 使用 npx http-server
```bash
npx http-server -p 3000 -o -c-1 --silent
```

### 方式 3: 使用 Python SimpleHTTPServer
```bash
python3 -m http.server 3000
```

### 方式 4: 使用 Node.js http-server
```bash
npx http-server build/ -p 3000 -o -c-1 --silent
```

## 🌐 部署到云服务

### Vercel 部署
```bash
npm install -g vercel
cd build
vercel deploy
```

### Netlify 部署
```bash
npm install -g netlify-cli
netlify deploy --dir=build --prod
```

### 部署到 GitHub Pages
```bash
cd build
git init
git add .
git commit -m "Initial commit"
git push origin main
```

## 🔧 本地端口

- 开发服务器: http://localhost:1420
- Web 服务器: http://localhost:3000

## 📂 技术栈

- 前端: React + Vite
- 构建: Tauri (Web 模式)
- 服务器: Node.js http-server / Vercel / Netlify

## 🎉 注意事项

1. 这是 MindCortex 的 Web 版本，某些功能可能受限于浏览器环境
2. 文件访问功能在 Web 版本中可能需要额外的权限配置
3. Agent 连接器需要后端服务支持
4. 向量数据库（ChromaDB）需要服务端支持

## 📞 获取帮助

如有问题，请访问项目文档或联系开发团队。
EOF

# 创建 Vercel 配置
echo "📸 创建 Vercel 配置..."
cat > "$WEB_DIR/vercel.json" << 'EOF'
{
  "version": 2,
  "builds": [
    {
      "src": "package.json",
      "use": "@vercel/static-build-plugin"
    }
  ]
}
EOF

# 创建 Netlify 配置
echo "📸 创建 Netlify 配置..."
cat > "$WEB_DIR/netlify.toml" << 'EOF
[build]
  command = "vite build"
  publish = "dist"

[[redirects]]
  from = "/*"
  to = "/index.html"
  status = 200
  force = true
EOF

# 创建 Dockerfile（可选）
echo "📸创建 Dockerfile..."
cat > "$WEB_DIR/Dockerfile" << 'EOF'
FROM nginx:alpine

WORKDIR /usr/share/nginx/html

COPY dist/* /usr/share/nginx/html/

EXPOSE 80

CMD ["nginx", "-g", "daemon off;"]
EOF

echo "✅ Web 版本准备完成！"
echo ""
echo "📂 文件位置:"
echo "   构建文件: $BUILD_DIR"
echo "   部署文件: $WEB_DIR"
echo ""
echo "🚀 启动 Web 服务器:"
echo "   cd $WEB_DIR && ./start-server.sh"
echo ""
echo "🌐 部署到云服务:"
echo "   Vercel: cd $WEB_DIR/dist && npx vercel deploy"
echo "   Netlify: cd $WEB_DIR && npx netlify deploy --dir=dist --prod"
echo ""
echo "🎯 访问地址: http://localhost:3000"
echo ""
echo "✅ 部署完成！"
