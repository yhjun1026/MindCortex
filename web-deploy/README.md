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
