//! Git 仓库操作
//! 
//! 提供 Git 仓库的核心操作和状态检查。

use crate::error::{GtError, GtResult};
use git2::{Repository as Git2Repo, StatusOptions};
use std::path::{Path, PathBuf};

/// Git 仓库抽象
pub struct Repository {
    inner: Git2Repo,
    path: PathBuf,
}

impl Repository {
    /// 打开当前目录的 Git 仓库
    pub fn open() -> GtResult<Self> {
        Self::open_path(".")
    }
    
    /// 打开指定路径的 Git 仓库
    pub fn open_path<P: AsRef<Path>>(path: P) -> GtResult<Self> {
        let path = path.as_ref();
        let inner = Git2Repo::discover(path)
            .map_err(|_| GtError::NotInGitRepo)?;
        
        let path = inner.workdir()
            .ok_or(GtError::NotInGitRepo)?
            .to_path_buf();
            
        Ok(Self { inner, path })
    }
    
    /// 初始化新的 Git 仓库
    pub fn init<P: AsRef<Path>>(path: P) -> GtResult<Self> {
        let path = path.as_ref();
        let inner = Git2Repo::init(path)?;
        let path = path.to_path_buf();
        
        Ok(Self { inner, path })
    }
    
    /// 获取仓库路径
    pub fn path(&self) -> &Path {
        &self.path
    }
    
    /// 获取内部的 git2 仓库对象
    pub fn inner(&self) -> &Git2Repo {
        &self.inner
    }
    
    /// 获取当前分支名
    pub fn current_branch(&self) -> GtResult<String> {
        let head = self.inner.head()
            .map_err(|_| GtError::CurrentBranchNotFound)?;
            
        if let Some(name) = head.shorthand() {
            Ok(name.to_string())
        } else {
            Err(GtError::CurrentBranchNotFound)
        }
    }
    
    /// 检查是否有未提交的变更
    pub fn has_uncommitted_changes(&self) -> GtResult<bool> {
        let statuses = self.inner.statuses(Some(
            StatusOptions::new()
                .include_untracked(false)
                .include_ignored(false)
        ))?;
        
        Ok(!statuses.is_empty())
    }
    
    /// 检查是否有未追踪的文件
    pub fn has_untracked_files(&self) -> GtResult<bool> {
        let statuses = self.inner.statuses(Some(
            StatusOptions::new()
                .include_untracked(true)
                .include_ignored(false)
        ))?;
        
        for entry in statuses.iter() {
            if entry.status().contains(git2::Status::WT_NEW) {
                return Ok(true);
            }
        }
        
        Ok(false)
    }
    
    /// 检查工作区是否干净
    pub fn is_clean(&self) -> GtResult<bool> {
        Ok(!self.has_uncommitted_changes()? && !self.has_untracked_files()?)
    }
    
    /// 获取仓库状态摘要
    pub fn status_summary(&self) -> GtResult<RepositoryStatus> {
        let statuses = self.inner.statuses(Some(
            StatusOptions::new()
                .include_untracked(true)
                .include_ignored(false)
        ))?;
        
        let mut status = RepositoryStatus::default();
        
        for entry in statuses.iter() {
            let flags = entry.status();
            
            if flags.contains(git2::Status::INDEX_NEW) 
                || flags.contains(git2::Status::INDEX_MODIFIED)
                || flags.contains(git2::Status::INDEX_DELETED)
                || flags.contains(git2::Status::INDEX_RENAMED)
                || flags.contains(git2::Status::INDEX_TYPECHANGE) {
                status.staged += 1;
            }
            
            if flags.contains(git2::Status::WT_MODIFIED)
                || flags.contains(git2::Status::WT_DELETED)
                || flags.contains(git2::Status::WT_TYPECHANGE)
                || flags.contains(git2::Status::WT_RENAMED) {
                status.modified += 1;
            }
            
            if flags.contains(git2::Status::WT_NEW) {
                status.untracked += 1;
            }
        }
        
        Ok(status)
    }
    
    /// 获取与远程的同步状态
    pub fn sync_status(&self, remote_name: &str) -> GtResult<SyncStatus> {
        let current_branch = self.current_branch()?;
        let local_ref = format!("refs/heads/{}", current_branch);
        let remote_ref = format!("refs/remotes/{}/{}", remote_name, current_branch);
        
        let local_oid = self.inner.refname_to_id(&local_ref).ok();
        let remote_oid = self.inner.refname_to_id(&remote_ref).ok();
        
        match (local_oid, remote_oid) {
            (Some(local), Some(remote)) => {
                if local == remote {
                    Ok(SyncStatus::UpToDate)
                } else {
                    let (ahead, behind) = self.inner.graph_ahead_behind(local, remote)?;
                    Ok(SyncStatus::Diverged { ahead, behind })
                }
            }
            (Some(_), None) => Ok(SyncStatus::LocalOnly),
            (None, Some(_)) => Ok(SyncStatus::RemoteOnly),
            (None, None) => Ok(SyncStatus::NoTracking),
        }
    }
    
    /// 检查分支是否存在
    pub fn branch_exists(&self, name: &str) -> GtResult<bool> {
        match self.inner.find_branch(name, git2::BranchType::Local) {
            Ok(_) => Ok(true),
            Err(e) if e.code() == git2::ErrorCode::NotFound => Ok(false),
            Err(e) => Err(GtError::GitError(e)),
        }
    }
    
    /// 检查远程是否存在
    pub fn remote_exists(&self, name: &str) -> GtResult<bool> {
        match self.inner.find_remote(name) {
            Ok(_) => Ok(true),
            Err(e) if e.code() == git2::ErrorCode::NotFound => Ok(false),
            Err(e) => Err(GtError::GitError(e)),
        }
    }
}

/// 仓库状态摘要
#[derive(Debug, Default, Clone)]
pub struct RepositoryStatus {
    /// 已暂存的文件数
    pub staged: usize,
    /// 已修改的文件数
    pub modified: usize,
    /// 未追踪的文件数
    pub untracked: usize,
}

impl RepositoryStatus {
    /// 检查是否有变更
    pub fn has_changes(&self) -> bool {
        self.staged > 0 || self.modified > 0 || self.untracked > 0
    }
    
    /// 检查是否干净
    pub fn is_clean(&self) -> bool {
        !self.has_changes()
    }
}

/// 与远程的同步状态
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SyncStatus {
    /// 与远程同步
    UpToDate,
    /// 本地领先
    Ahead(usize),
    /// 本地落后
    Behind(usize),
    /// 分叉状态
    Diverged { ahead: usize, behind: usize },
    /// 仅本地存在
    LocalOnly,
    /// 仅远程存在
    RemoteOnly,
    /// 无跟踪分支
    NoTracking,
}

impl SyncStatus {
    /// 获取状态描述
    pub fn description(&self) -> String {
        match self {
            Self::UpToDate => "与远程同步".to_string(),
            Self::Ahead(n) => format!("领先远程 {} 个提交", n),
            Self::Behind(n) => format!("落后远程 {} 个提交", n),
            Self::Diverged { ahead, behind } => {
                format!("领先 {} 个提交，落后 {} 个提交", ahead, behind)
            }
            Self::LocalOnly => "仅本地存在".to_string(),
            Self::RemoteOnly => "仅远程存在".to_string(),
            Self::NoTracking => "无跟踪分支".to_string(),
        }
    }
    
    /// 检查是否需要推送
    pub fn needs_push(&self) -> bool {
        match self {
            Self::Ahead(_) => true,
            Self::Diverged { ahead, .. } => *ahead > 0,
            Self::LocalOnly => true,
            _ => false,
        }
    }
    
    /// 检查是否需要拉取
    pub fn needs_pull(&self) -> bool {
        match self {
            Self::Behind(_) => true,
            Self::Diverged { behind, .. } => *behind > 0,
            _ => false,
        }
    }
} 