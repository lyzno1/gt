# GT (Git Toolkit)

一个用 Rust 重写的下一代 Git 工作流工具，旨在提供比传统 `gw` (Git Workflow) 脚本更快、更可靠、更友好的体验。

## 特性

### 🚀 性能
- **原生性能**: 编译为二进制文件，启动快速
- **并发操作**: 使用 async/await 处理 I/O 密集型操作
- **内存安全**: Rust 的所有权系统保证内存安全

### 🛡️ 可靠性
- **类型安全**: 编译时捕获错误，减少运行时问题
- **错误恢复**: 智能的错误处理和恢复策略
- **操作验证**: 执行前验证所有前置条件

### 👥 用户友好
- **直观命令**: 简洁明了的命令行接口
- **友好错误**: 详细的错误描述和解决建议
- **一致体验**: 与原有 `gw` 工具保持习惯一致

## 安装

### 从源码编译
```bash
git clone https://github.com/your-org/gt
cd gt
cargo build --release
cp target/release/gt /usr/local/bin/
```

### 使用包管理器 (计划中)
```bash
# macOS
brew install gt

# Linux
curl -L https://github.com/your-org/gt/releases/latest/download/gt-linux.tar.gz | tar xz
```

## 快速开始

### 基本工作流
```bash
# 1. 开始新功能
gt start feature-branch

# 2. 做一些工作...
echo "新功能" > feature.txt

# 3. 保存工作
gt save -m "添加新功能"

# 4. 同步分支
gt update

# 5. 提交成果
gt ship --pr

# 6. 清理分支 (可选)
gt clean feature-branch
```

### 配置设置
```bash
# 查看配置
gt config show

# 设置 GitHub Token
gt config set github.token your-token

# 从 gw 迁移配置
gt config migrate
```

## 命令参考

### 核心命令

| 命令 | 对应 gw | 描述 |
|------|---------|------|
| `gt start <branch>` | `gw start` | 开始新的功能分支 |
| `gt save [options]` | `gw save` | 保存当前工作 (add + commit) |
| `gt update` | `gw update` | 同步当前分支 |
| `gt ship [options]` | `gw submit` | 提交工作成果 |
| `gt clean <branch>` | `gw rm` | 清理分支 |

### 辅助命令

| 命令 | 描述 |
|------|------|
| `gt status` | 显示仓库状态 |
| `gt init [path]` | 初始化 Git 仓库 |
| `gt config` | 配置管理 |

### 全局选项

| 选项 | 描述 |
|------|------|
| `-v, --verbose` | 启用详细输出 |
| `-n, --dry-run` | 预演模式，不执行实际操作 |
| `-y, --yes` | 非交互模式，自动确认所有提示 |

## 详细用法

### gt start - 开始新分支
```bash
# 从 main 分支创建新分支
gt start feature-branch

# 从指定分支创建
gt start feature-branch --base develop

# 仅使用本地分支，不拉取远程
gt start feature-branch --local
```

### gt save - 保存工作
```bash
# 提交所有更改
gt save

# 指定提交信息
gt save -m "修复重要bug"

# 强制使用编辑器
gt save --edit

# 只提交指定文件
gt save src/main.rs src/lib.rs
```

### gt ship - 提交成果
```bash
# 创建 Pull Request
gt ship --pr

# 自动合并 (rebase)
gt ship --auto-merge

# 自动合并 (squash)
gt ship --squash

# 合并后删除分支
gt ship --delete-branch

# 不切换回主分支
gt ship --no-switch
```

## 架构

GT 采用分层架构设计：

```
┌─────────────────┐
│   CLI Layer     │  命令行接口和路由
├─────────────────┤
│ Application     │  命令实现和业务逻辑
├─────────────────┤
│   Domain        │  Git 操作和工作流
├─────────────────┤
│ Infrastructure  │  GitHub API、配置等
└─────────────────┘
```

详细信息请参考 [架构文档](ARCHITECTURE.md)。

## 配置

GT 使用 TOML 格式的配置文件，位于：
- macOS: `~/Library/Application Support/gt/config.toml`
- Linux: `~/.config/gt/config.toml`
- Windows: `%APPDATA%\gt\config.toml`

### 配置示例
```toml
[git]
default_remote = "origin"
default_base = "main"
auto_push = true

[github]
token = "your-github-token"
default_org = "your-org"

[ui]
verbose = false
confirm_destructive = true
```

## 从 gw 迁移

GT 提供了自动迁移工具：

```bash
# 自动检测并迁移 gw 配置
gt config migrate

# 查看迁移的配置
gt config show
```

### 命令映射
- `gw start` → `gt start`
- `gw save` → `gt save`
- `gw update` → `gt update`
- `gw submit` → `gt ship`
- `gw rm` → `gt clean`

## 开发

### 环境要求
- Rust 1.70+
- Git 2.0+

### 构建
```bash
cargo build
```

### 测试
```bash
cargo test
```

### 运行
```bash
cargo run -- start test-branch
```

## 贡献

欢迎贡献！请参考以下步骤：

1. Fork 这个仓库
2. 创建特性分支 (`gt start feature/amazing-feature`)
3. 提交更改 (`gt save -m "添加一些惊人功能"`)
4. 推送分支 (`gt update`)
5. 创建 Pull Request (`gt ship --pr`)

## 许可证

本项目采用 MIT 许可证 - 查看 [LICENSE](LICENSE) 文件了解详情。

## 路线图

### v0.1.0 (当前)
- [x] 基础架构和 CLI
- [x] 错误处理系统
- [x] Git 抽象层
- [ ] 核心命令实现

### v0.2.0
- [ ] GitHub 集成
- [ ] 配置迁移工具
- [ ] 高级工作流

### v0.3.0
- [ ] 性能优化
- [ ] 扩展插件系统
- [ ] 完整测试覆盖

### v1.0.0
- [ ] 生产就绪
- [ ] 完整文档
- [ ] 多平台分发

## 支持

- 📖 [文档](docs/)
- 🐛 [问题报告](https://github.com/your-org/gt/issues)
- 💬 [讨论](https://github.com/your-org/gt/discussions)
- 📧 [邮件支持](mailto:support@your-org.com)

---

**GT - 让 Git 工作流更简单、更快速、更可靠！** 🚀 # 测试自动合并修复
