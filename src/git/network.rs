//! Git 网络操作模块
//! 
//! 提供带重试机制的网络操作，对应 gw 的 git_network_ops.sh

use crate::error::{GtResult, GtError};
use crate::git::Repository;
use crate::ui::{print_step, print_warning, print_error, print_success};
use std::time::Duration;
use std::thread;

/// 网络操作配置
#[derive(Debug, Clone)]
pub struct NetworkConfig {
    pub max_attempts: usize,
    pub delay_seconds: u64,
    pub verbose: bool,
}

impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
            max_attempts: 50,
            delay_seconds: 1,
            verbose: true,
        }
    }
}

/// 网络操作管理器
pub struct NetworkOps {
    config: NetworkConfig,
}

impl NetworkOps {
    /// 创建网络操作管理器
    pub fn new(config: NetworkConfig) -> Self {
        Self { config }
    }
    
    /// 带重试的推送操作
    pub fn push_with_retry(
        &self,
        repo: &Repository,
        remote: &str,
        branch: Option<&str>,
    ) -> GtResult<()> {
        let current_branch = repo.current_branch()?;
        let branch_name = branch.unwrap_or(&current_branch);
        
        if self.config.verbose {
            print_step(&format!("推送分支 '{}' 到远程 '{}'...", branch_name, remote));
        }
        
        for attempt in 1..=self.config.max_attempts {
            match repo.push(remote, Some(branch_name)) {
                Ok(_) => {
                    if self.config.verbose {
                        print_success("推送成功");
                    }
                    return Ok(());
                }
                Err(e) => {
                    if attempt >= self.config.max_attempts {
                        print_error(&format!("推送失败，已达到最大重试次数 ({})", self.config.max_attempts));
                        return Err(e);
                    }
                    
                    if self.config.verbose {
                        print_warning(&format!("推送失败 (尝试 {}/{}): {}", attempt, self.config.max_attempts, e));
                        print_step(&format!("等待 {} 秒后重试...", self.config.delay_seconds));
                    }
                    
                    thread::sleep(Duration::from_secs(self.config.delay_seconds));
                }
            }
        }
        
        unreachable!()
    }
    
    /// 带重试的拉取操作（使用 rebase）
    pub fn pull_with_retry(
        &self,
        repo: &Repository,
        remote: &str,
        branch: Option<&str>,
        use_rebase: bool,
    ) -> GtResult<()> {
        let current_branch = repo.current_branch()?;
        let branch_name = branch.unwrap_or(&current_branch);
        
        if self.config.verbose {
            let method = if use_rebase { "rebase" } else { "merge" };
            print_step(&format!("从远程 '{}' 拉取分支 '{}' (使用 {})...", remote, branch_name, method));
        }
        
        for attempt in 1..=self.config.max_attempts {
            let result = if use_rebase {
                repo.pull_rebase(remote, Some(branch_name))
            } else {
                repo.pull_merge(remote, Some(branch_name))
            };
            
            match result {
                Ok(_) => {
                    if self.config.verbose {
                        print_success("拉取成功");
                    }
                    return Ok(());
                }
                Err(e) => {
                    if attempt >= self.config.max_attempts {
                        print_error(&format!("拉取失败，已达到最大重试次数 ({})", self.config.max_attempts));
                        return Err(e);
                    }
                    
                    if self.config.verbose {
                        print_warning(&format!("拉取失败 (尝试 {}/{}): {}", attempt, self.config.max_attempts, e));
                        print_step(&format!("等待 {} 秒后重试...", self.config.delay_seconds));
                    }
                    
                    thread::sleep(Duration::from_secs(self.config.delay_seconds));
                }
            }
        }
        
        unreachable!()
    }
    
    /// 带重试的抓取操作
    pub fn fetch_with_retry(
        &self,
        repo: &Repository,
        remote: &str,
    ) -> GtResult<()> {
        if self.config.verbose {
            print_step(&format!("从远程 '{}' 抓取更新...", remote));
        }
        
        for attempt in 1..=self.config.max_attempts {
            match repo.fetch(remote) {
                Ok(_) => {
                    if self.config.verbose {
                        print_success("抓取成功");
                    }
                    return Ok(());
                }
                Err(e) => {
                    if attempt >= self.config.max_attempts {
                        print_error(&format!("抓取失败，已达到最大重试次数 ({})", self.config.max_attempts));
                        return Err(e);
                    }
                    
                    if self.config.verbose {
                        print_warning(&format!("抓取失败 (尝试 {}/{}): {}", attempt, self.config.max_attempts, e));
                        print_step(&format!("等待 {} 秒后重试...", self.config.delay_seconds));
                    }
                    
                    thread::sleep(Duration::from_secs(self.config.delay_seconds));
                }
            }
        }
        
        unreachable!()
    }
    
    /// 检查网络连接
    pub fn check_connectivity(&self, repo: &Repository, remote: &str) -> GtResult<bool> {
        if self.config.verbose {
            print_step(&format!("检查与远程 '{}' 的连接...", remote));
        }
        
        // 简单的连接检查：尝试列出远程引用
        match repo.fetch(remote) {
            Ok(_) => {
                if self.config.verbose {
                    print_success("网络连接正常");
                }
                Ok(true)
            }
            Err(e) => {
                if self.config.verbose {
                    print_warning(&format!("网络连接检查失败: {}", e));
                }
                Ok(false)
            }
        }
    }
    
    /// 智能推送：先检查连接，再推送
    pub fn smart_push(
        &self,
        repo: &Repository,
        remote: &str,
        branch: Option<&str>,
    ) -> GtResult<()> {
        // 先检查连接
        if !self.check_connectivity(repo, remote)? {
            return Err(GtError::NetworkError {
                message: format!("无法连接到远程仓库 '{}'", remote)
            });
        }
        
        // 再推送
        self.push_with_retry(repo, remote, branch)
    }
    
    /// 智能拉取：先检查连接，再拉取
    pub fn smart_pull(
        &self,
        repo: &Repository,
        remote: &str,
        branch: Option<&str>,
        use_rebase: bool,
    ) -> GtResult<()> {
        // 先检查连接
        if !self.check_connectivity(repo, remote)? {
            return Err(GtError::NetworkError {
                message: format!("无法连接到远程仓库 '{}'", remote)
            });
        }
        
        // 再拉取
        self.pull_with_retry(repo, remote, branch, use_rebase)
    }
}

/// 便捷函数：使用默认配置进行推送
pub fn push_with_retry(repo: &Repository, remote: &str, branch: Option<&str>) -> GtResult<()> {
    let ops = NetworkOps::new(NetworkConfig::default());
    ops.push_with_retry(repo, remote, branch)
}

/// 便捷函数：使用默认配置进行拉取（rebase）
pub fn pull_rebase_with_retry(repo: &Repository, remote: &str, branch: Option<&str>) -> GtResult<()> {
    let ops = NetworkOps::new(NetworkConfig::default());
    ops.pull_with_retry(repo, remote, branch, true)
}

/// 便捷函数：使用默认配置进行抓取
pub fn fetch_with_retry(repo: &Repository, remote: &str) -> GtResult<()> {
    let ops = NetworkOps::new(NetworkConfig::default());
    ops.fetch_with_retry(repo, remote)
} 