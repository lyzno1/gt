//! Git fetch 操作抽象
//! 
//! 提供从远程仓库获取更新的类型定义和便捷函数

use crate::error::{GtResult, GtError};
use crate::git::Repository;

/// Fetch 操作的选项
#[derive(Debug, Clone)]
pub struct FetchOptions {
    /// 是否获取所有远程
    pub all: bool,
    /// 是否获取标签
    pub tags: bool,
    /// 是否强制更新引用
    pub force: bool,
    /// 是否显示详细信息
    pub verbose: bool,
    /// 是否删除本地不存在远程的引用
    pub prune: bool,
}

impl Default for FetchOptions {
    fn default() -> Self {
        Self {
            all: false,
            tags: false,
            force: false,
            verbose: false,
            prune: false,
        }
    }
}

/// Fetch 操作的结果
#[derive(Debug)]
pub struct FetchResult {
    /// 已获取的远程仓库列表
    pub fetched_remotes: Vec<String>,
    /// 更新的引用列表
    pub updated_refs: Vec<String>,
    /// 使用的选项
    pub options: FetchOptions,
}

impl FetchResult {
    /// 获取已获取的远程数量
    pub fn remote_count(&self) -> usize {
        self.fetched_remotes.len()
    }
    
    /// 是否成功获取
    pub fn is_success(&self) -> bool {
        !self.fetched_remotes.is_empty()
    }
}

/// 便捷函数：获取默认远程
pub fn fetch_origin(repo: &Repository, verbose: bool) -> GtResult<FetchResult> {
    let remote_name = "origin";
    
    if verbose {
        println!("🌐 从 {} 获取更新...", remote_name);
    }
    
    if !repo.remote_exists(remote_name)? {
        return Err(GtError::RemoteNotFound {
            remote: remote_name.to_string()
        });
    }
    
    repo.fetch(remote_name)?;
    
    if verbose {
        println!("✅ 从 {} 获取完成", remote_name);
    }
    
    Ok(FetchResult {
        fetched_remotes: vec![remote_name.to_string()],
        updated_refs: Vec::new(),
        options: FetchOptions { verbose, ..Default::default() },
    })
}

/// 便捷函数：获取所有远程
pub fn fetch_all(repo: &Repository, verbose: bool) -> GtResult<FetchResult> {
    let remotes = repo.list_remotes()?;
    let mut fetched_remotes = Vec::new();
    
    if verbose {
        println!("🌐 获取所有远程仓库...");
    }
    
    for remote in remotes {
        if verbose {
            println!("   正在获取 {}...", remote.name);
        }
        
        match repo.fetch(&remote.name) {
            Ok(_) => {
                fetched_remotes.push(remote.name.clone());
                if verbose {
                    println!("   ✅ {} 获取成功", remote.name);
                }
            }
            Err(e) => {
                if verbose {
                    println!("   ❌ {} 获取失败: {}", remote.name, e);
                }
                return Err(e);
            }
        }
    }
    
    Ok(FetchResult {
        fetched_remotes,
        updated_refs: Vec::new(),
        options: FetchOptions { all: true, verbose, ..Default::default() },
    })
}

/// 便捷函数：获取指定远程
pub fn fetch_remote(repo: &Repository, remote: &str, verbose: bool) -> GtResult<FetchResult> {
    if verbose {
        println!("🌐 从 {} 获取更新...", remote);
    }
    
    if !repo.remote_exists(remote)? {
        return Err(GtError::RemoteNotFound {
            remote: remote.to_string()
        });
    }
    
    repo.fetch(remote)?;
    
    if verbose {
        println!("✅ 从 {} 获取完成", remote);
    }
    
    Ok(FetchResult {
        fetched_remotes: vec![remote.to_string()],
        updated_refs: Vec::new(),
        options: FetchOptions { verbose, ..Default::default() },
    })
} 