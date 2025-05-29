# 贡献指南

感谢您对 GT (Git Toolkit) 项目的兴趣！

## 开发环境设置

1. 安装 Rust (推荐使用 rustup)
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2. 克隆项目
```bash
git clone https://github.com/lyzno1/gt.git
cd gt
```

3. 安装开发依赖
```bash
rustup component add clippy rustfmt
cargo install cargo-audit cargo-watch
```

4. 运行测试
```bash
cargo test
```

## 开发流程

### 代码风格

- 使用 `cargo fmt` 格式化代码
- 使用 `cargo clippy` 检查代码质量
- 遵循 Rust 官方风格指南

### 提交规范

我们使用 [Conventional Commits](https://www.conventionalcommits.org/) 规范：

```
type(scope): subject

body

footer
```

类型：
- `feat`: 新功能
- `fix`: Bug 修复
- `docs`: 文档更新
- `style`: 代码格式化
- `refactor`: 代码重构
- `test`: 测试相关
- `chore`: 构建或辅助工具的变动

### 分支策略

- `main`: 主分支，包含稳定的发布版本
- `develop`: 开发分支，包含最新的开发进度
- `feature/*`: 功能分支
- `hotfix/*`: 热修复分支

### Pull Request 流程

1. Fork 项目
2. 创建功能分支：`git checkout -b feature/my-feature`
3. 提交更改：`git commit -am 'feat: add some feature'`
4. 推送到分支：`git push origin feature/my-feature`
5. 创建 Pull Request

### 测试

- 为新功能编写测试
- 确保所有测试通过：`cargo test`
- 测试覆盖率应保持在 80% 以上

### 文档

- 为公共 API 编写文档注释
- 更新 README.md（如果需要）
- 添加使用示例

## 项目架构

请参考 [ARCHITECTURE.md](ARCHITECTURE.md) 了解项目架构。

## 发布流程

1. 更新版本号：`Cargo.toml` 中的 `version`
2. 更新 CHANGELOG.md
3. 创建 tag：`git tag v0.x.0`
4. 推送 tag：`git push origin v0.x.0`
5. GitHub Actions 会自动构建和发布

## 问题报告

请使用 GitHub Issues 报告问题，包含：

- 操作系统和版本
- Rust 版本
- GT 版本
- 重现步骤
- 期望行为
- 实际行为

## 联系方式

- GitHub Issues: 项目问题讨论
- GitHub Discussions: 功能建议和讨论 