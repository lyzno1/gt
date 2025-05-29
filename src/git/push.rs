//! Git push 操作抽象
//! 
//! 提供推送到远程仓库的类型定义和便捷函数

use crate::error::{GtResult, GtError};
use crate::git::Repository;

/// Push 操作的选项
#[derive(Debug, Clone)]
pub struct PushOptions {
    /// 是否强制推送
    pub force: bool,
    /// 是否设置上游分支
    pub set_upstream: bool,
    /// 是否显示详细信息
    pub verbose: bool,
}

impl Default for PushOptions {
    fn default() -> Self {
        Self {
            force: false,
            set_upstream: false,
            verbose: false,
        }
    }
}

/// Push 操作的结果
#[derive(Debug)]
pub struct PushResult {
    /// 推送的远程仓库
    pub remote: String,
    /// 推送的分支
    pub branch: String,
    /// 使用的选项
    pub options: PushOptions,
}

/// 便捷函数：推送当前分支到 origin
pub fn push_current(repo: &Repository, verbose: bool) -> GtResult<PushResult> {
    let branch = repo.current_branch()?;
    let remote = "origin";
    
    if verbose {
        println!("🚀 推送 {} 到 {}...", branch, remote);
    }
    
    repo.push(remote, Some(&branch))?;
    
    if verbose {
        println!("✅ 推送完成");
    }
    
    Ok(PushResult {
        remote: remote.to_string(),
        branch,
        options: PushOptions { verbose, ..Default::default() },
    })
}

/// 便捷函数：首次推送（设置上游）
pub fn push_set_upstream(repo: &Repository, remote: &str, branch: &str, verbose: bool) -> GtResult<PushResult> {
    if verbose {
        println!("🚀 首次推送 {} 到 {} 并设置上游...", branch, remote);
    }
    
    repo.push(remote, Some(branch))?;
    
    if verbose {
        println!("✅ 推送完成并已设置上游分支");
    }
    
    Ok(PushResult {
        remote: remote.to_string(),
        branch: branch.to_string(),
        options: PushOptions { set_upstream: true, verbose, ..Default::default() },
    })
} 