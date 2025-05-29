//! GT (Git Toolkit) - 下一代 Git 工作流工具
//! 
//! GT 是一个用 Rust 编写的高性能 Git 工作流工具，旨在简化常见的 Git 操作
//! 并提供更好的用户体验。
//! 
//! # 核心特性
//! 
//! - **流程驱动**: 命令对应开发意图，而非 Git 技术细节
//! - **智能默认**: 减少决策负担，提供最佳实践默认值
//! - **安全第一**: 防止误操作，提供清晰反馈
//! - **高性能**: 基于 Rust 和 libgit2，启动快速，操作高效
//! 
//! # 基本用法
//! 
//! ```bash
//! gt start feature/auth    # 开始新功能开发
//! gt save "实现登录逻辑"    # 保存进度
//! gt sync                  # 同步更新
//! gt ship                  # 提交工作成果
//! ```

pub mod cli;
pub mod commands;
pub mod git;
pub mod github;
pub mod workflow;
pub mod config;
pub mod ui;
pub mod error;
pub mod utils;

// 重新导出核心类型
pub use error::{GtError, GtResult};
pub use config::GtConfig;
pub use git::Repository; 