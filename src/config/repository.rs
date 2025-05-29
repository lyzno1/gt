//! 仓库配置模块
//! 
//! 管理远程仓库、主分支等配置，对应 gw 的 config_vars.sh

use crate::error::{GtResult, GtError};
use crate::git::Repository;
use std::env;

/// 仓库配置
#[derive(Debug, Clone)]
pub struct RepoConfig {
    /// 默认远程仓库名
    pub remote_name: String,
    /// 主分支名
    pub main_branch: String,
    /// 最大重试次数
    pub max_attempts: usize,
    /// 重试延迟（秒）
    pub delay_seconds: u64,
}

impl Default for RepoConfig {
    fn default() -> Self {
        Self {
            remote_name: "origin".to_string(),
            main_branch: "main".to_string(),
            max_attempts: 50,
            delay_seconds: 1,
        }
    }
}

impl RepoConfig {
    /// 从环境变量和 Git 仓库创建配置
    pub fn from_env_and_repo(repo: &Repository) -> GtResult<Self> {
        let mut config = Self::default();
        
        // 从环境变量读取配置
        if let Ok(remote) = env::var("REMOTE_NAME") {
            config.remote_name = remote;
        }
        
        if let Ok(attempts) = env::var("MAX_ATTEMPTS") {
            if let Ok(attempts) = attempts.parse() {
                config.max_attempts = attempts;
            }
        }
        
        if let Ok(delay) = env::var("DELAY_SECONDS") {
            if let Ok(delay) = delay.parse() {
                config.delay_seconds = delay;
            }
        }
        
        // 检测实际的主分支名
        config.main_branch = Self::detect_main_branch(repo)?;
        
        Ok(config)
    }
    
    /// 检测主分支名（master 或 main）
    fn detect_main_branch(repo: &Repository) -> GtResult<String> {
        // 检查本地分支
        if repo.branch_exists("master")? {
            return Ok("master".to_string());
        }
        
        if repo.branch_exists("main")? {
            return Ok("main".to_string());
        }
        
        // 检查远程分支
        let remotes = repo.list_remotes()?;
        for remote in &remotes {
            // 检查 origin/master
            if Self::remote_branch_exists(repo, &remote.name, "master")? {
                return Ok("master".to_string());
            }
            
            // 检查 origin/main
            if Self::remote_branch_exists(repo, &remote.name, "main")? {
                return Ok("main".to_string());
            }
        }
        
        // 默认使用环境变量或 main
        let default_branch = env::var("DEFAULT_MAIN_BRANCH")
            .unwrap_or_else(|_| "main".to_string());
            
        Ok(default_branch)
    }
    
    /// 检查远程分支是否存在
    fn remote_branch_exists(repo: &Repository, remote: &str, branch: &str) -> GtResult<bool> {
        // 这里需要检查远程分支，现在先简单返回 false
        // TODO: 实现远程分支检查
        let _ = (repo, remote, branch);
        Ok(false)
    }
    
    /// 验证配置是否有效
    pub fn validate(&self, repo: &Repository) -> GtResult<()> {
        // 检查远程仓库是否存在
        if !repo.remote_exists(&self.remote_name)? {
            return Err(GtError::ConfigError {
                message: format!("远程仓库 '{}' 不存在", self.remote_name)
            });
        }
        
        Ok(())
    }
    
    /// 获取主分支的完整远程引用
    pub fn main_branch_remote_ref(&self) -> String {
        format!("{}/{}", self.remote_name, self.main_branch)
    }
}

/// 全局配置管理器
pub struct ConfigManager {
    repo_config: RepoConfig,
}

impl ConfigManager {
    /// 创建配置管理器
    pub fn new(repo: &Repository) -> GtResult<Self> {
        let repo_config = RepoConfig::from_env_and_repo(repo)?;
        repo_config.validate(repo)?;
        
        Ok(Self { repo_config })
    }
    
    /// 获取仓库配置
    pub fn repo_config(&self) -> &RepoConfig {
        &self.repo_config
    }
    
    /// 更新主分支配置
    pub fn update_main_branch(&mut self, branch: String) {
        self.repo_config.main_branch = branch;
    }
    
    /// 更新远程仓库配置
    pub fn update_remote_name(&mut self, remote: String) {
        self.repo_config.remote_name = remote;
    }
} 