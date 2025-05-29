pub mod repository;
pub mod branch;
pub mod commit;
pub mod remote;
pub mod stash;
pub mod status;
pub mod add;
pub mod fetch;
pub mod push;
pub mod reset;
pub mod network;

// 重新导出核心类型
pub use repository::Repository;
pub use branch::Branch;
pub use commit::Commit;
pub use remote::Remote;
pub use stash::Stash;
pub use status::WorkingTreeStatus;

// 重新导出操作结果类型
pub use add::{AddOptions, AddResult};
pub use fetch::{FetchOptions, FetchResult};
pub use push::{PushOptions, PushResult};
pub use reset::{ResetType, ResetResult};

// 重新导出网络操作
pub use network::{NetworkConfig, NetworkOps, push_with_retry, pull_rebase_with_retry, fetch_with_retry};

use crate::error::{GtResult, GtError};

/// Git操作的统一接口
/// 这是主要的入口点，提供所有 Git 相关功能
pub struct GitOps {
    repo: Repository,
}

impl GitOps {
    /// 初始化Git操作实例
    pub fn new() -> GtResult<Self> {
        let repo = Repository::discover()?;
        Ok(Self { repo })
    }
    
    /// 从指定路径创建
    pub fn from_path<P: AsRef<std::path::Path>>(path: P) -> GtResult<Self> {
        let repo = Repository::open_path(path)?;
        Ok(Self { repo })
    }
    
    /// 检查是否在Git仓库中
    pub fn is_git_repo(&self) -> bool {
        self.repo.is_valid()
    }
    
    /// 获取仓库引用
    pub fn repository(&self) -> &Repository {
        &self.repo
    }
    
    // 分支相关操作
    /// 获取当前分支名
    pub fn current_branch(&self) -> GtResult<String> {
        self.repo.current_branch()
    }
    
    /// 创建新分支
    pub fn create_branch(&self, name: &str, base_branch: Option<&str>) -> GtResult<()> {
        self.repo.create_branch(name, base_branch)
    }
    
    /// 切换分支
    pub fn checkout_branch(&self, name: &str) -> GtResult<()> {
        self.repo.checkout_branch(name)
    }
    
    /// 创建并切换到新分支
    pub fn create_and_checkout_branch(&self, name: &str, base_branch: Option<&str>) -> GtResult<()> {
        self.repo.create_and_checkout_branch(name, base_branch)
    }
    
    /// 列出所有本地分支
    pub fn list_branches(&self) -> GtResult<Vec<Branch>> {
        self.repo.list_branches()
    }
    
    /// 删除分支
    pub fn delete_branch(&self, name: &str, force: bool) -> GtResult<()> {
        self.repo.delete_branch(name, force)
    }
    
    // 状态相关操作
    /// 检查工作区状态
    pub fn check_status(&self) -> GtResult<WorkingTreeStatus> {
        self.repo.check_status()
    }
    
    /// 检查是否有未提交的变更
    pub fn has_uncommitted_changes(&self) -> GtResult<bool> {
        self.repo.has_uncommitted_changes()
    }
    
    /// 检查是否有未追踪的文件
    pub fn has_untracked_files(&self) -> GtResult<bool> {
        self.repo.has_untracked_files()
    }
    
    /// 检查工作区是否干净
    pub fn is_clean(&self) -> GtResult<bool> {
        self.repo.is_clean()
    }
    
    // 提交相关操作
    /// 添加文件到暂存区
    pub fn add_files(&self, files: &[&str]) -> GtResult<()> {
        self.repo.add_files(files)
    }
    
    /// 添加所有变更到暂存区
    pub fn add_all(&self) -> GtResult<()> {
        self.repo.add_all()
    }
    
    /// 创建提交
    pub fn create_commit(&self, message: &str) -> GtResult<()> {
        self.repo.create_commit(message)
    }
    
    /// 获取最近的提交
    pub fn get_latest_commit(&self) -> GtResult<Commit> {
        self.repo.get_latest_commit()
    }
    
    // Stash 相关操作
    /// 创建 stash
    pub fn create_stash(&self, message: Option<&str>) -> GtResult<()> {
        self.repo.create_stash(message)
    }
    
    /// 应用 stash
    pub fn apply_stash(&self, index: usize) -> GtResult<()> {
        self.repo.apply_stash(index)
    }
    
    /// 弹出 stash
    pub fn pop_stash(&self, index: usize) -> GtResult<()> {
        self.repo.pop_stash(index)
    }
    
    /// 列出所有 stash
    pub fn list_stashes(&self) -> GtResult<Vec<Stash>> {
        self.repo.list_stashes()
    }
    
    // ===== 远程操作 =====
    
    /// 添加远程仓库
    pub fn add_remote(&self, name: &str, url: &str) -> GtResult<()> {
        self.repo.add_remote(name, url)
    }
    
    /// 列出所有远程仓库
    pub fn list_remotes(&self) -> GtResult<Vec<Remote>> {
        self.repo.list_remotes()
    }
    
    /// 获取指定远程仓库信息
    pub fn get_remote(&self, name: &str) -> GtResult<Remote> {
        self.repo.get_remote(name)
    }
    
    /// 推送到远程仓库
    pub fn push(&self, remote: &str, branch: Option<&str>) -> GtResult<()> {
        self.repo.push(remote, branch)
    }
    
    /// 拉取远程仓库
    pub fn pull(&self, remote: &str, branch: Option<&str>) -> GtResult<()> {
        self.repo.pull(remote, branch)
    }
    
    /// 从远程仓库抓取
    pub fn fetch(&self, remote: &str) -> GtResult<()> {
        self.repo.fetch(remote)
    }
    
    /// 合并分支
    pub fn merge(&self, source: &str) -> GtResult<()> {
        self.repo.merge(source)
    }
    
    // ===== 重置操作 =====
    
    /// 软重置（保留工作区和暂存区）
    pub fn reset_soft(&self, target: &str) -> GtResult<()> {
        self.repo.reset_soft(target)
    }
    
    /// 混合重置（保留工作区，清空暂存区）
    pub fn reset_mixed(&self, target: &str) -> GtResult<()> {
        self.repo.reset_mixed(target)
    }
    
    /// 硬重置（清除工作区和暂存区的所有变更）
    pub fn reset_hard(&self, target: &str) -> GtResult<()> {
        self.repo.reset_hard(target)
    }
    
    // ===== 暂存区操作 =====
    
    /// 取消暂存文件
    pub fn unstage_files(&self, files: &[&str]) -> GtResult<()> {
        self.repo.unstage_files(files)
    }
    
    /// 取消暂存所有文件
    pub fn unstage_all(&self) -> GtResult<()> {
        self.repo.unstage_all()
    }
    
    // ===== 日志和历史 =====
    
    /// 获取提交历史
    pub fn get_commit_history(&self, count: usize, skip: usize) -> GtResult<Vec<Commit>> {
        self.repo.get_commit_history(count, skip)
    }
    
    /// 获取两个提交之间的差异文件列表
    pub fn get_diff_files(&self, from: &str, to: &str) -> GtResult<Vec<String>> {
        self.repo.get_diff_files(from, to)
    }
    
    // ===== 标签操作 =====
    
    /// 创建轻量标签
    pub fn create_lightweight_tag(&self, name: &str, target: Option<&str>) -> GtResult<()> {
        self.repo.create_lightweight_tag(name, target)
    }
    
    /// 创建带注释的标签
    pub fn create_annotated_tag(&self, name: &str, message: &str, target: Option<&str>) -> GtResult<()> {
        self.repo.create_annotated_tag(name, message, target)
    }
    
    /// 列出所有标签
    pub fn list_tags(&self) -> GtResult<Vec<String>> {
        self.repo.list_tags()
    }
    
    /// 删除标签
    pub fn delete_tag(&self, name: &str) -> GtResult<()> {
        self.repo.delete_tag(name)
    }
    
    // ===== 工作区清理 =====
    
    /// 清理未追踪的文件
    pub fn clean_untracked(&self, directories: bool, force: bool) -> GtResult<Vec<String>> {
        self.repo.clean_untracked(directories, force)
    }
} 