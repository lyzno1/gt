# GT (Git Toolkit) 架构文档

## 概述

GT 是一个用 Rust 重写的下一代 Git 工作流工具，旨在替代现有的 `gw` (Git Workflow) Bash 脚本工具。本文档详细说明了 GT 的架构设计理念、目录结构和工作原理。

## 设计理念

### 1. 模块化架构
- **单一职责原则**: 每个模块只负责一个特定的功能领域
- **松耦合**: 模块间通过明确的接口进行交互
- **可扩展性**: 新功能可以轻松添加而不影响现有代码

### 2. 类型安全
- **强类型系统**: 利用 Rust 的类型系统防止运行时错误
- **错误处理**: 使用 `Result<T, E>` 类型进行显式错误处理
- **零成本抽象**: 在编译时解决问题，运行时无额外开销

### 3. 用户体验优先
- **直观的命令**: 简洁明了的命令行接口
- **友好的错误信息**: 详细的错误描述和恢复建议
- **一致性**: 与原有 `gw` 工具保持使用习惯的一致性

### 4. 性能与可靠性
- **内存安全**: Rust 的所有权系统保证内存安全
- **并发安全**: 使用 async/await 处理 I/O 密集型操作
- **原生性能**: 编译为原生二进制文件，启动快速

## 项目结构

```
gt/
├── Cargo.toml              # 项目配置和依赖管理
├── ARCHITECTURE.md         # 本架构文档
├── README.md              # 项目说明
├── src/                   # 源代码目录
│   ├── main.rs           # 程序入口点
│   ├── lib.rs            # 库入口，重新导出核心类型
│   │
│   ├── cli/              # 命令行接口层
│   │   ├── mod.rs        # CLI 模块声明
│   │   ├── args.rs       # 命令行参数定义 (使用 clap)
│   │   └── router.rs     # 命令路由器，分发命令到处理器
│   │
│   ├── commands/         # 命令实现层
│   │   ├── mod.rs        # 命令模块声明和重新导出
│   │   ├── start.rs      # start 命令 (创建新分支)
│   │   ├── save.rs       # save 命令 (保存工作)
│   │   ├── update.rs     # update 命令 (同步分支)
│   │   ├── ship.rs       # ship 命令 (提交成果)
│   │   ├── clean.rs      # clean 命令 (清理分支)
│   │   ├── status.rs     # status 命令 (显示状态)
│   │   ├── init.rs       # init 命令 (初始化仓库)
│   │   └── config.rs     # config 命令 (配置管理)
│   │
│   ├── git/              # Git 操作抽象层
│   │   ├── mod.rs        # Git 模块声明
│   │   ├── repository.rs # 仓库抽象和状态管理
│   │   ├── branch.rs     # 分支操作
│   │   ├── commit.rs     # 提交操作
│   │   ├── remote.rs     # 远程操作
│   │   └── stash.rs      # 暂存操作
│   │
│   ├── error/            # 错误处理系统
│   │   ├── mod.rs        # 错误模块声明
│   │   ├── types.rs      # 错误类型定义
│   │   ├── handler.rs    # 错误处理器
│   │   └── recovery.rs   # 错误恢复策略
│   │
│   ├── github.rs         # GitHub API 集成
│   ├── workflow.rs       # 工作流引擎
│   ├── config.rs         # 配置管理
│   ├── ui.rs            # 用户界面和交互
│   └── utils.rs         # 通用工具函数
│
└── target/              # 编译输出目录
    └── debug/gt         # 可执行文件
```

## 架构层次

### 1. 表示层 (Presentation Layer)
**位置**: `src/main.rs`, `src/cli/`

**职责**:
- 解析命令行参数
- 提供用户友好的错误信息
- 处理用户交互

**组件**:
- `main.rs`: 程序入口点，负责启动异步运行时并调用 CLI
- `cli/args.rs`: 使用 clap 库定义完整的命令行参数结构
- `cli/router.rs`: 命令路由器，将解析后的命令分发到对应的处理器

**设计理念**: 
- 将用户界面与业务逻辑分离
- 提供类型安全的参数解析
- 统一的命令处理入口

### 2. 应用层 (Application Layer)
**位置**: `src/commands/`

**职责**:
- 实现具体的业务命令
- 协调各个领域服务
- 处理工作流逻辑

**组件**:
- 每个命令都是一个独立的结构体，实现 `execute()` 方法
- 命令参数通过构造函数传入，保证数据完整性
- 异步执行模型，支持并发操作

**设计理念**:
- 命令模式 (Command Pattern)
- 每个命令都是无状态的，可以独立测试
- 统一的执行接口

### 3. 领域层 (Domain Layer)
**位置**: `src/git/`, `src/workflow.rs`

**职责**:
- 封装 Git 操作的复杂性
- 提供高级的工作流抽象
- 维护业务规则和约束

**组件**:
- `git/repository.rs`: Git 仓库的核心抽象，提供状态查询和基本操作
- `git/branch.rs`: 分支管理操作
- `git/commit.rs`: 提交相关操作
- `git/remote.rs`: 远程仓库操作
- `git/stash.rs`: 工作区暂存操作
- `workflow.rs`: 工作流引擎，编排复杂的 Git 操作序列

**设计理念**:
- 领域驱动设计 (DDD)
- 将底层 Git 操作抽象为高级概念
- 提供类型安全的接口

### 4. 基础设施层 (Infrastructure Layer)
**位置**: `src/github.rs`, `src/config.rs`, `src/ui.rs`, `src/utils.rs`

**职责**:
- 外部系统集成
- 配置管理
- 通用工具和服务

**组件**:
- `github.rs`: GitHub API 集成，处理 Pull Request 等操作
- `config.rs`: 配置文件管理，支持从 gw 迁移配置
- `ui.rs`: 用户界面组件，进度条、确认对话框等
- `utils.rs`: 通用工具函数和验证逻辑

**设计理念**:
- 插件化架构
- 可配置和可扩展
- 与外部系统的松耦合

### 5. 横切关注点 (Cross-cutting Concerns)
**位置**: `src/error/`

**职责**:
- 统一的错误处理
- 日志记录
- 性能监控

**组件**:
- `error/types.rs`: 定义所有可能的错误类型
- `error/handler.rs`: 错误处理器，提供用户友好的错误信息
- `error/recovery.rs`: 错误恢复策略

## 数据流

### 典型的命令执行流程

```
用户输入命令
     ↓
main.rs (程序入口)
     ↓
cli/args.rs (解析参数)
     ↓
cli/router.rs (路由命令)
     ↓
commands/*.rs (执行具体命令)
     ↓
git/*.rs (Git 操作)
     ↓
外部系统 (Git, GitHub API)
```

### 错误处理流程

```
任何层级的错误
     ↓
error/types.rs (错误类型化)
     ↓
error/handler.rs (错误处理)
     ↓
用户友好的错误信息
```

## 核心类型和接口

### 1. 错误处理
```rust
// 统一的结果类型
type GtResult<T> = Result<T, GtError>;

// 主要错误类型
enum GtError {
    NotInGitRepo,
    BranchNotFound { branch: String },
    UncommittedChanges,
    NetworkTimeout { attempts: u32 },
    // ... 其他错误类型
}
```

### 2. 命令接口
```rust
// 所有命令都实现这个模式
struct SomeCommand {
    // 命令参数
}

impl SomeCommand {
    fn new(params) -> Self { ... }
    async fn execute(self) -> GtResult<()> { ... }
}
```

### 3. Git 抽象
```rust
struct Repository {
    // Git 仓库的高级抽象
}

impl Repository {
    fn open() -> GtResult<Self> { ... }
    fn current_branch(&self) -> GtResult<String> { ... }
    fn status_summary(&self) -> GtResult<RepositoryStatus> { ... }
    // ... 其他操作
}
```

## 依赖管理

### 核心依赖
- **clap**: 命令行参数解析
- **git2**: Git 操作的底层库
- **tokio**: 异步运行时
- **thiserror**: 错误类型定义
- **octocrab**: GitHub API 客户端

### 设计原则
- 最小化依赖数量
- 选择成熟稳定的库
- 避免功能重复的依赖

## 扩展性设计

### 1. 添加新命令
1. 在 `src/commands/` 下创建新的命令文件
2. 在 `src/cli/args.rs` 中添加命令参数定义
3. 在 `src/cli/router.rs` 中添加路由处理
4. 在 `src/commands/mod.rs` 中导出新命令

### 2. 添加新的 Git 操作
1. 在 `src/git/` 下创建新的操作模块
2. 在 `src/git/mod.rs` 中导出新模块
3. 在相关命令中使用新操作

### 3. 添加新的错误类型
1. 在 `src/error/types.rs` 中定义新错误
2. 在相关模块中使用新错误类型

## 性能考虑

### 1. 编译时优化
- 使用泛型和 trait 实现零成本抽象
- 条件编译减少二进制文件大小
- 优化的依赖配置

### 2. 运行时性能
- 异步 I/O 避免阻塞
- 懒加载减少启动时间
- 内存池重用减少分配

### 3. 用户体验
- 快速的命令响应
- 渐进式输出显示进度
- 智能缓存减少重复操作

## 测试策略

### 1. 单元测试
- 每个模块的核心逻辑
- 错误处理路径
- 边界条件测试

### 2. 集成测试
- 命令端到端测试
- Git 操作集成测试
- 错误恢复测试

### 3. 系统测试
- 真实 Git 仓库测试
- GitHub API 集成测试
- 性能基准测试

## 部署和分发

### 1. 构建配置
- 针对不同平台的优化配置
- 静态链接减少依赖
- 压缩二进制文件

### 2. 安装方式
- 直接下载二进制文件
- 包管理器分发 (homebrew, apt 等)
- 从源码编译

### 3. 配置迁移
- 自动检测现有 `gw` 配置
- 提供迁移工具和向导
- 向后兼容性支持

---

这个架构设计确保了 GT 项目的：
- **可维护性**: 清晰的模块分离和职责划分
- **可扩展性**: 容易添加新功能和修改现有功能
- **可测试性**: 每个组件都可以独立测试
- **用户友好性**: 直观的命令和友好的错误信息
- **性能**: 高效的异步执行和优化的编译配置 