# MindCortex Release Artifacts

This directory contains release artifacts for MindCortex.

## v0.2.1 - 2026-03-16

### macOS (Apple Silicon, ARM64)

- **DMG Installer**: `mindcortex-0.2.1-aarch64.dmg` (5.3 MB)
  - Double-click to mount and install
  - Drag the app to /Applications

- **Application Bundle**: `mindcortex-temp.app` (13 MB)
  - Direct application bundle
  - Can be copied to /Applications directly

### Installation

**Option 1: Using DMG**
1. Download `mindcortex-0.2.1-aarch64.dmg`
2. Double-click the DMG file to mount it
3. Drag `mindcortex-temp.app` to your Applications folder
4. Launch from Applications

**Option 2: Using App Bundle**
1. Download or copy `mindcortex-temp.app`
2. Move to /Applications
3. Launch from Applications

### System Requirements

- macOS 11.0 (Big Sur) or later
- Apple Silicon (M1/M2/M3) or Intel Mac with Rosetta 2
- 100 MB free disk space

### Features

- Hybrid search (keyword + semantic)
- Knowledge graph visualization
- RAG-based natural language queries
- Code assistant with multiple LLM providers
- VSCode extension integration
- Session management and analytics

### Changelog

**v0.2.1 (2026-03-16)**

- ✅ Complete Phase 1: Performance optimization (concurrent search, caching, index warmup)
- ✅ Complete Phase 2: VSCode plugin integration
- ✅ Complete Phase 3: Knowledge graph visualization
- ✅ Complete Phase 4: Natural language query (RAG framework)
- ✅ Fix all TypeScript and Rust compilation errors
- ✅ Build macOS desktop application
- ✅ Create DMG installer
- ✅ Web deployment ready

### Support

- **GitHub**: https://github.com/yhjun1026/MindCortex
- **Issues**: https://github.com/yhjun1026/MindCortex/issues
- **Documentation**: See README.md in project root

### Notes

- This is the first stable release (v0.2.1)
- VSCode extension is included in the repository
- Web version is deployed at http://localhost:8080/
- Full feature documentation available in COMPLETION_REPORT_V0.2.1.md
