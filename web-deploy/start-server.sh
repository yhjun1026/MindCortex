#!/bin/bash

# MindCortex Web 服务器启动脚本

echo "🚀 MindCortex Web 服务器启动中..."
echo "📍 访问地址: http://localhost:3000"
echo "按 Ctrl+C 停止服务器"

# 使用 http-server 提供静态文件
npx http-server -p 3000 -o -c-1 --silent
