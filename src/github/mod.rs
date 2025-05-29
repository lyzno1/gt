//! GitHub CLI 抽象模块
//! 
//! 提供 GitHub CLI (gh) 的抽象接口，支持 PR 创建、合并等操作

pub mod cli;
pub mod pr;

// 重新导出核心类型
pub use cli::{GithubCli, GithubAuth};
pub use pr::{
    PullRequest, PullRequestManager, MergeStrategy, 
    CreatePrOptions, MergePrOptions
}; 