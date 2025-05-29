//! Git 仓库操作
//! 
//! 提供 Git 仓库的核心操作和状态检查。

use crate::error::{GtError, GtResult};
use git2::{Repository as Git2Repo, StatusOptions, BranchType, Signature};
use std::path::{Path, PathBuf};
use std::process::Command;

// 重新导入我们需要的类型
use super::branch::Branch;
use super::commit::Commit;
use super::stash::Stash;
use super::status::WorkingTreeStatus;
use super::remote::Remote;

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
    
    /// 发现并打开 Git 仓库（类似 git2::Repository::discover）
    pub fn discover() -> GtResult<Self> {
        let inner = Git2Repo::discover(".")
            .map_err(|_| GtError::NotInGitRepo)?;
        
        let path = inner.workdir()
            .ok_or(GtError::NotInGitRepo)?
            .to_path_buf();
            
        Ok(Self { inner, path })
    }
    
    /// 检查仓库是否有效
    pub fn is_valid(&self) -> bool {
        !self.inner.is_bare() && self.inner.workdir().is_some()
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
    
    // ===== 分支相关操作 =====
    
    /// 创建新分支
    pub fn create_branch(&self, name: &str, base_branch: Option<&str>) -> GtResult<()> {
        // 验证分支名
        if !Branch::is_valid_name(name) {
            return Err(GtError::InvalidInput { 
                input: format!("无效的分支名: {}", name) 
            });
        }
        
        // 检查分支是否已存在
        if self.branch_exists(name)? {
            return Err(GtError::BranchAlreadyExists { 
                branch: name.to_string() 
            });
        }
        
        // 确定基础提交
        let base_commit = if let Some(base) = base_branch {
            // 基于指定分支
            let base_ref = self.inner.find_branch(base, BranchType::Local)
                .or_else(|_| self.inner.find_branch(base, BranchType::Remote))
                .map_err(|e| GtError::GitOperation { 
                    message: format!("找不到基础分支 '{}': {}", base, e) 
                })?;
            base_ref.get().target().unwrap()
        } else {
            // 基于当前HEAD
            self.inner.head()
                .map_err(|e| GtError::GitOperation { 
                    message: format!("无法获取HEAD: {}", e) 
                })?
                .target().unwrap()
        };
        
        let commit = self.inner.find_commit(base_commit)
            .map_err(|e| GtError::GitOperation { 
                message: format!("找不到基础提交: {}", e) 
            })?;
            
        self.inner.branch(name, &commit, false)
            .map_err(|e| GtError::GitOperation { 
                message: format!("创建分支失败: {}", e) 
            })?;
            
        Ok(())
    }
    
    /// 切换分支
    pub fn checkout_branch(&self, name: &str) -> GtResult<()> {
        // 检查分支是否存在
        if !self.branch_exists(name)? {
            return Err(GtError::BranchNotFound { 
                branch: name.to_string() 
            });
        }
        
        let branch_ref = format!("refs/heads/{}", name);
        let obj = self.inner.revparse_single(&branch_ref)
            .map_err(|e| GtError::GitOperation { 
                message: format!("无法解析分支引用: {}", e) 
            })?;
            
        self.inner.checkout_tree(&obj, None)
            .map_err(|e| GtError::GitOperation { 
                message: format!("检出文件失败: {}", e) 
            })?;
            
        self.inner.set_head(&branch_ref)
            .map_err(|e| GtError::GitOperation { 
                message: format!("更新HEAD失败: {}", e) 
            })?;
            
        Ok(())
    }
    
    /// 创建并切换到新分支
    pub fn create_and_checkout_branch(&self, name: &str, base_branch: Option<&str>) -> GtResult<()> {
        self.create_branch(name, base_branch)?;
        self.checkout_branch(name)?;
        Ok(())
    }
    
    /// 列出所有本地分支
    pub fn list_branches(&self) -> GtResult<Vec<Branch>> {
        let mut branches = Vec::new();
        let current = self.current_branch().unwrap_or_default();
        
        let branch_iter = self.inner.branches(Some(BranchType::Local))
            .map_err(|e| GtError::GitOperation { 
                message: format!("无法列出分支: {}", e) 
            })?;
            
        for branch_result in branch_iter {
            let (branch, _) = branch_result
                .map_err(|e| GtError::GitOperation { 
                    message: format!("读取分支信息失败: {}", e) 
                })?;
                
            if let Some(name) = branch.name()? {
                let branch_info = Branch {
                    name: name.to_string(),
                    is_current: name == current,
                    is_remote: false,
                    upstream: self.get_upstream(name).ok(),
                    ahead: 0, // TODO: 计算ahead/behind
                    behind: 0,
                    last_commit: self.get_last_commit(name).ok(),
                };
                branches.push(branch_info);
            }
        }
        
        Ok(branches)
    }
    
    /// 删除分支
    pub fn delete_branch(&self, name: &str, force: bool) -> GtResult<()> {
        let current = self.current_branch()?;
        if current == name {
            return Err(GtError::GitOperation { 
                message: format!("无法删除当前分支 '{}'", name) 
            });
        }
        
        let mut branch = self.inner.find_branch(name, BranchType::Local)
            .map_err(|_| GtError::BranchNotFound { 
                branch: name.to_string() 
            })?;
            
        // 检查是否已合并(除非强制删除)
        if !force {
            // TODO: 实现合并检查
        }
        
        branch.delete()
            .map_err(|e| GtError::GitOperation { 
                message: format!("删除分支失败: {}", e) 
            })?;
            
        Ok(())
    }
    
    // 分支辅助方法
    fn get_upstream(&self, branch_name: &str) -> GtResult<String> {
        let branch = self.inner.find_branch(branch_name, BranchType::Local)?;
        if let Ok(upstream) = branch.upstream() {
            if let Some(name) = upstream.name()? {
                return Ok(name.to_string());
            }
        }
        Err(GtError::GitOperation { 
            message: "没有上游分支".to_string() 
        })
    }
    
    fn get_last_commit(&self, branch_name: &str) -> GtResult<String> {
        let branch = self.inner.find_branch(branch_name, BranchType::Local)?;
        if let Some(oid) = branch.get().target() {
            return Ok(oid.to_string());
        }
        Err(GtError::GitOperation { 
            message: "无法获取最后提交".to_string() 
        })
    }
    
    // ===== 状态相关操作 =====
    
    /// 检查工作区状态
    pub fn check_status(&self) -> GtResult<WorkingTreeStatus> {
        let statuses = self.inner.statuses(None)?;
        
        let mut status = WorkingTreeStatus {
            has_uncommitted_changes: false,
            has_untracked_files: false,
            has_staged_changes: false,
            modified_files: 0,
            added_files: 0,
            deleted_files: 0,
            untracked_files: 0,
        };
        
        for entry in statuses.iter() {
            let flags = entry.status();
            
            if flags.is_wt_modified() || flags.is_wt_deleted() || flags.is_wt_renamed() {
                status.has_uncommitted_changes = true;
                status.modified_files += 1;
            }
            
            if flags.is_wt_new() {
                status.has_untracked_files = true;
                status.untracked_files += 1;
            }
            
            if flags.is_index_modified() || flags.is_index_new() || flags.is_index_deleted() {
                status.has_staged_changes = true;
                if flags.is_index_new() {
                    status.added_files += 1;
                } else if flags.is_index_deleted() {
                    status.deleted_files += 1;
                }
            }
        }
        
        Ok(status)
    }
    
    // ===== 提交相关操作 =====
    
    /// 添加文件到暂存区
    pub fn add_files(&self, files: &[&str]) -> GtResult<()> {
        let mut index = self.inner.index()?;
        
        for file in files {
            index.add_path(Path::new(file))
                .map_err(|e| GtError::GitOperation {
                    message: format!("无法添加文件 '{}': {}", file, e)
                })?;
        }
        
        index.write()
            .map_err(|e| GtError::GitOperation {
                message: format!("无法写入索引: {}", e)
            })?;
            
        Ok(())
    }
    
    /// 添加所有变更到暂存区
    pub fn add_all(&self) -> GtResult<()> {
        let mut index = self.inner.index()?;
        index.add_all(&["*"], git2::IndexAddOption::DEFAULT, None)
            .map_err(|e| GtError::GitOperation {
                message: format!("无法添加所有文件: {}", e)
            })?;
        
        index.write()
            .map_err(|e| GtError::GitOperation {
                message: format!("无法写入索引: {}", e)
            })?;
            
        Ok(())
    }
    
    /// 创建提交
    pub fn create_commit(&self, message: &str) -> GtResult<()> {
        // 获取当前用户签名
        let signature = self.get_signature()?;
        
        // 获取 HEAD
        let head = self.inner.head()?;
        let parent_commit = head.peel_to_commit()?;
        
        // 获取索引并写入树
        let mut index = self.inner.index()?;
        let tree_id = index.write_tree()?;
        let tree = self.inner.find_tree(tree_id)?;
        
        // 创建提交
        self.inner.commit(
            Some("HEAD"),
            &signature,
            &signature,
            message,
            &tree,
            &[&parent_commit],
        )?;
        
        Ok(())
    }
    
    /// 获取最近的提交
    pub fn get_latest_commit(&self) -> GtResult<Commit> {
        let head = self.inner.head()?;
        let commit = head.peel_to_commit()?;
        
        Ok(self.convert_commit(&commit))
    }
    
    /// 获取用户签名
    fn get_signature(&self) -> GtResult<Signature> {
        let config = self.inner.config()?;
        let name = config.get_string("user.name")
            .map_err(|_| GtError::ConfigError { 
                message: "Git 用户名未配置，请运行 'git config user.name \"你的名字\"'".to_string() 
            })?;
        let email = config.get_string("user.email")
            .map_err(|_| GtError::ConfigError { 
                message: "Git 邮箱未配置，请运行 'git config user.email \"你的邮箱\"'".to_string() 
            })?;
            
        Ok(Signature::now(&name, &email)?)
    }
    
    /// 转换 git2::Commit 为我们的 Commit 类型
    fn convert_commit(&self, commit: &git2::Commit) -> Commit {
        let author = commit.author();
        Commit {
            id: commit.id().to_string(),
            message: commit.message().unwrap_or("").to_string(),
            author: author.name().unwrap_or("").to_string(),
            author_email: author.email().unwrap_or("").to_string(),
            time: author.when().seconds(),
            parents: commit.parent_ids().map(|id| id.to_string()).collect(),
        }
    }
    
    // ===== Stash 相关操作 =====
    
    /// 创建 stash
    pub fn create_stash(&self, message: Option<&str>) -> GtResult<()> {
        let signature = self.get_signature()?;
        let message = message.unwrap_or("WIP");
        
        // 注意：stash_save 需要可变引用，但我们的设计是不可变的
        // 这里我们需要重新思考设计，或者使用 RefCell/Mutex
        // 暂时先返回未实现错误
        Err(GtError::NotImplemented { 
            feature: "stash operations require mutable repository access".to_string() 
        })
    }
    
    /// 应用 stash
    pub fn apply_stash(&self, _index: usize) -> GtResult<()> {
        Err(GtError::NotImplemented { 
            feature: "stash operations require mutable repository access".to_string() 
        })
    }
    
    /// 弹出 stash
    pub fn pop_stash(&self, _index: usize) -> GtResult<()> {
        Err(GtError::NotImplemented { 
            feature: "stash operations require mutable repository access".to_string() 
        })
    }
    
    /// 列出所有 stash
    pub fn list_stashes(&self) -> GtResult<Vec<Stash>> {
        let mut stashes = Vec::new();
        
        // stash_foreach 也需要可变引用
        // 暂时返回空列表
        Ok(stashes)
    }
    
    // ===== 远程操作 =====
    
    /// 添加远程仓库
    pub fn add_remote(&self, name: &str, url: &str) -> GtResult<()> {
        self.inner.remote(name, url)
            .map_err(|e| GtError::GitOperation {
                message: format!("添加远程仓库失败: {}", e)
            })?;
        Ok(())
    }
    
    /// 列出所有远程仓库
    pub fn list_remotes(&self) -> GtResult<Vec<Remote>> {
        let remotes = self.inner.remotes()?;
        let mut result = Vec::new();
        
        for name in remotes.iter() {
            if let Some(name) = name {
                if let Ok(remote) = self.inner.find_remote(name) {
                    let url = remote.url().unwrap_or("").to_string();
                    let push_url = remote.pushurl().map(|s| s.to_string());
                    
                    result.push(Remote {
                        name: name.to_string(),
                        url,
                        push_url,
                    });
                }
            }
        }
        
        Ok(result)
    }
    
    /// 获取指定远程仓库信息
    pub fn get_remote(&self, name: &str) -> GtResult<Remote> {
        let remote = self.inner.find_remote(name)
            .map_err(|_| GtError::RemoteNotFound { remote: name.to_string() })?;
            
        let url = remote.url().unwrap_or("").to_string();
        let push_url = remote.pushurl().map(|s| s.to_string());
        
        Ok(Remote {
            name: name.to_string(),
            url,
            push_url,
        })
    }
    
    /// 推送到远程仓库
    pub fn push(&self, remote: &str, branch: Option<&str>) -> GtResult<()> {
        let current_branch = self.current_branch()?;
        let branch = branch.unwrap_or(&current_branch);
        let refspec = format!("refs/heads/{}:refs/heads/{}", branch, branch);
        
        let mut remote = self.inner.find_remote(remote)
            .map_err(|_| GtError::RemoteNotFound { remote: remote.to_string() })?;
            
        // 推送操作需要回调函数来处理认证
        let mut callbacks = git2::RemoteCallbacks::new();
        callbacks.credentials(|_url, username_from_url, _allowed_types| {
            // 尝试使用 SSH 密钥
            git2::Cred::ssh_key_from_agent(username_from_url.unwrap_or("git"))
        });
        
        let mut push_options = git2::PushOptions::new();
        push_options.remote_callbacks(callbacks);
        
        remote.push(&[&refspec], Some(&mut push_options))
            .map_err(|e| GtError::PushFailed { 
                reason: format!("推送失败: {}", e) 
            })?;
            
        Ok(())
    }
    
    /// 拉取远程仓库（默认使用 rebase）
    pub fn pull(&self, remote: &str, branch: Option<&str>) -> GtResult<()> {
        self.pull_rebase(remote, branch)
    }
    
    /// 拉取并 rebase（推荐方式，保持线性历史）
    pub fn pull_rebase(&self, remote: &str, branch: Option<&str>) -> GtResult<()> {
        // 先 fetch
        self.fetch(remote)?;
        
        // 然后 rebase
        let current_branch = self.current_branch()?;
        let branch = branch.unwrap_or(&current_branch);
        let remote_branch = format!("{}/{}", remote, branch);
        self.rebase(&remote_branch)?;
        
        Ok(())
    }
    
    /// 拉取并 merge（不推荐，会产生 merge commit）
    pub fn pull_merge(&self, remote: &str, branch: Option<&str>) -> GtResult<()> {
        // 先 fetch
        self.fetch(remote)?;
        
        // 然后合并
        let current_branch = self.current_branch()?;
        let branch = branch.unwrap_or(&current_branch);
        let remote_branch = format!("{}/{}", remote, branch);
        self.merge(&remote_branch)?;
        
        Ok(())
    }
    
    /// 从远程仓库抓取
    pub fn fetch(&self, remote: &str) -> GtResult<()> {
        let mut remote = self.inner.find_remote(remote)
            .map_err(|_| GtError::RemoteNotFound { remote: remote.to_string() })?;
            
        let mut callbacks = git2::RemoteCallbacks::new();
        callbacks.credentials(|_url, username_from_url, _allowed_types| {
            git2::Cred::ssh_key_from_agent(username_from_url.unwrap_or("git"))
        });
        
        let mut fetch_options = git2::FetchOptions::new();
        fetch_options.remote_callbacks(callbacks);
        
        remote.fetch(&[] as &[String], Some(&mut fetch_options), None)
            .map_err(|e| GtError::GitOperation {
                message: format!("抓取失败: {}", e)
            })?;
            
        Ok(())
    }
    
    /// 合并分支
    pub fn merge(&self, source: &str) -> GtResult<()> {
        // 解析源分支
        let source_commit = self.inner.revparse_single(source)
            .map_err(|e| GtError::GitOperation {
                message: format!("无法解析源分支 '{}': {}", source, e)
            })?
            .into_commit()
            .map_err(|_| GtError::GitOperation {
                message: format!("'{}' 不是有效的提交", source)
            })?;
            
        // 创建 AnnotatedCommit
        let annotated_commit = self.inner.find_annotated_commit(source_commit.id())?;
            
        // 获取当前提交
        let head = self.inner.head()?;
        let head_commit = head.peel_to_commit()?;
        
        // 检查是否需要合并
        if head_commit.id() == source_commit.id() {
            return Ok(()); // 已经是最新的
        }
        
        // 执行合并分析
        let (analysis, _preference) = self.inner.merge_analysis(&[&annotated_commit])?;
        
        if analysis.is_fast_forward() {
            // 快进合并
            let refname = head.name().unwrap();
            let mut reference = self.inner.find_reference(refname)?;
            reference.set_target(source_commit.id(), "fast-forward merge")?;
            self.inner.set_head(refname)?;
            self.inner.checkout_head(Some(git2::build::CheckoutBuilder::default().force()))?;
        } else if analysis.is_normal() {
            // 三方合并
            let head_tree = head_commit.tree()?;
            let source_tree = source_commit.tree()?;
            let ancestor = self.inner.merge_base(head_commit.id(), source_commit.id())?;
            let ancestor_commit = self.inner.find_commit(ancestor)?;
            let ancestor_tree = ancestor_commit.tree()?;
            
            let mut index = self.inner.merge_trees(&ancestor_tree, &head_tree, &source_tree, None)?;
            
            if index.has_conflicts() {
                return Err(GtError::GitOperation {
                    message: "合并存在冲突，请手动解决".to_string()
                });
            }
            
            let tree_id = index.write_tree_to(&self.inner)?;
            let tree = self.inner.find_tree(tree_id)?;
            let signature = self.get_signature()?;
            let message = format!("Merge {} into {}", source, self.current_branch()?);
            
            self.inner.commit(
                Some("HEAD"),
                &signature,
                &signature,
                &message,
                &tree,
                &[&head_commit, &source_commit],
            )?;
        }
        
        Ok(())
    }
    
    /// 将当前分支 rebase 到目标分支/提交（推荐方式，保持线性历史）
    pub fn rebase(&self, target: &str) -> GtResult<()> {
        // 解析目标提交
        let target_commit = self.inner.revparse_single(target)
            .map_err(|e| GtError::GitOperation {
                message: format!("无法解析目标 '{}': {}", target, e)
            })?
            .into_commit()
            .map_err(|_| GtError::GitOperation {
                message: format!("'{}' 不是有效的提交", target)
            })?;
            
        // 获取当前提交
        let head = self.inner.head()?;
        let head_commit = head.peel_to_commit()?;
        
        // 检查是否需要 rebase
        if head_commit.id() == target_commit.id() {
            return Ok(()); // 已经是最新的
        }
        
        // 检查是否是快进情况
        let merge_base = self.inner.merge_base(head_commit.id(), target_commit.id())?;
        if merge_base == head_commit.id() {
            // 快进：直接移动 HEAD 到目标提交
            let refname = head.name().unwrap();
            let mut reference = self.inner.find_reference(refname)?;
            reference.set_target(target_commit.id(), "fast-forward")?;
            self.inner.set_head(refname)?;
            self.inner.checkout_head(Some(git2::build::CheckoutBuilder::default().force()))?;
            return Ok(());
        }
        
        // 使用外部 git 命令进行 rebase
        let output = Command::new("git")
            .args(&["rebase", target])
            .current_dir(&self.path)
            .output()
            .map_err(|e| GtError::GitOperation {
                message: format!("执行 git rebase 命令失败: {}", e)
            })?;
            
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(GtError::GitOperation {
                message: format!("Rebase 失败: {}", stderr)
            });
        }
        
        Ok(())
    }
    
    // ===== 重置操作 =====
    
    /// 软重置（保留工作区和暂存区）
    pub fn reset_soft(&self, target: &str) -> GtResult<()> {
        let object = self.inner.revparse_single(target)?;
        self.inner.reset(&object, git2::ResetType::Soft, None)
            .map_err(|e| GtError::GitOperation {
                message: format!("软重置失败: {}", e)
            })?;
        Ok(())
    }
    
    /// 混合重置（保留工作区，清空暂存区）
    pub fn reset_mixed(&self, target: &str) -> GtResult<()> {
        let object = self.inner.revparse_single(target)?;
        self.inner.reset(&object, git2::ResetType::Mixed, None)
            .map_err(|e| GtError::GitOperation {
                message: format!("混合重置失败: {}", e)
            })?;
        Ok(())
    }
    
    /// 硬重置（清除工作区和暂存区的所有变更）
    pub fn reset_hard(&self, target: &str) -> GtResult<()> {
        let object = self.inner.revparse_single(target)?;
        self.inner.reset(&object, git2::ResetType::Hard, None)
            .map_err(|e| GtError::GitOperation {
                message: format!("硬重置失败: {}", e)
            })?;
        Ok(())
    }
    
    // ===== 暂存区操作 =====
    
    /// 取消暂存文件
    pub fn unstage_files(&self, files: &[&str]) -> GtResult<()> {
        let head = self.inner.head()?;
        let head_commit = head.peel_to_commit()?;
        let head_tree = head_commit.tree()?;
        
        let mut index = self.inner.index()?;
        
        for file in files {
            let entry = head_tree.get_path(std::path::Path::new(file));
            match entry {
                Ok(entry) => {
                    // 文件在 HEAD 中存在，重置到 HEAD 状态
                    index.add(&git2::IndexEntry {
                        ctime: git2::IndexTime::new(0, 0),
                        mtime: git2::IndexTime::new(0, 0),
                        dev: 0,
                        ino: 0,
                        mode: entry.filemode() as u32,
                        uid: 0,
                        gid: 0,
                        file_size: 0,
                        id: entry.id(),
                        flags: 0,
                        flags_extended: 0,
                        path: file.as_bytes().to_vec(),
                    })?;
                }
                Err(_) => {
                    // 文件在 HEAD 中不存在，从索引中移除
                    index.remove_path(std::path::Path::new(file))?;
                }
            }
        }
        
        index.write()?;
        Ok(())
    }
    
    /// 取消暂存所有文件
    pub fn unstage_all(&self) -> GtResult<()> {
        let head = self.inner.head()?;
        let head_commit = head.peel_to_commit()?;
        let head_tree = head_commit.tree()?;
        
        let mut index = self.inner.index()?;
        index.read_tree(&head_tree)?;
        index.write()?;
        
        Ok(())
    }
    
    // ===== 日志和历史 =====
    
    /// 获取提交历史
    pub fn get_commit_history(&self, count: usize, skip: usize) -> GtResult<Vec<Commit>> {
        let mut revwalk = self.inner.revwalk()?;
        revwalk.push_head()?;
        revwalk.set_sorting(git2::Sort::TIME)?;
        
        let mut commits = Vec::new();
        for (i, oid) in revwalk.enumerate() {
            if i < skip {
                continue;
            }
            if commits.len() >= count {
                break;
            }
            
            let oid = oid?;
            let commit = self.inner.find_commit(oid)?;
            commits.push(self.convert_commit(&commit));
        }
        
        Ok(commits)
    }
    
    /// 获取两个提交之间的差异文件列表
    pub fn get_diff_files(&self, from: &str, to: &str) -> GtResult<Vec<String>> {
        let from_commit = self.inner.revparse_single(from)?.peel_to_commit()?;
        let to_commit = self.inner.revparse_single(to)?.peel_to_commit()?;
        
        let from_tree = from_commit.tree()?;
        let to_tree = to_commit.tree()?;
        
        let diff = self.inner.diff_tree_to_tree(Some(&from_tree), Some(&to_tree), None)?;
        
        let mut files = Vec::new();
        diff.foreach(
            &mut |delta, _progress| {
                if let Some(old_file) = delta.old_file().path() {
                    files.push(old_file.to_string_lossy().to_string());
                }
                if let Some(new_file) = delta.new_file().path() {
                    let path = new_file.to_string_lossy().to_string();
                    if !files.contains(&path) {
                        files.push(path);
                    }
                }
                true
            },
            None,
            None,
            None,
        )?;
        
        Ok(files)
    }
    
    // ===== 标签操作 =====
    
    /// 创建轻量标签
    pub fn create_lightweight_tag(&self, name: &str, target: Option<&str>) -> GtResult<()> {
        let target_oid = if let Some(target) = target {
            self.inner.revparse_single(target)?.id()
        } else {
            self.inner.head()?.target().unwrap()
        };
        
        let target_object = self.inner.find_object(target_oid, None)?;
        self.inner.tag_lightweight(name, &target_object, false)
            .map_err(|e| GtError::GitOperation {
                message: format!("创建标签失败: {}", e)
            })?;
            
        Ok(())
    }
    
    /// 创建带注释的标签
    pub fn create_annotated_tag(&self, name: &str, message: &str, target: Option<&str>) -> GtResult<()> {
        let target_oid = if let Some(target) = target {
            self.inner.revparse_single(target)?.id()
        } else {
            self.inner.head()?.target().unwrap()
        };
        
        let target_object = self.inner.find_object(target_oid, None)?;
        let signature = self.get_signature()?;
        
        self.inner.tag(name, &target_object, &signature, message, false)
            .map_err(|e| GtError::GitOperation {
                message: format!("创建标签失败: {}", e)
            })?;
            
        Ok(())
    }
    
    /// 列出所有标签
    pub fn list_tags(&self) -> GtResult<Vec<String>> {
        let mut tags = Vec::new();
        
        self.inner.tag_foreach(|oid, name| {
            if let Ok(name_str) = std::str::from_utf8(name) {
                if name_str.starts_with("refs/tags/") {
                    tags.push(name_str.strip_prefix("refs/tags/").unwrap().to_string());
                }
            }
            true
        })?;
        
        Ok(tags)
    }
    
    /// 删除标签
    pub fn delete_tag(&self, name: &str) -> GtResult<()> {
        self.inner.tag_delete(name)
            .map_err(|e| GtError::GitOperation {
                message: format!("删除标签失败: {}", e)
            })?;
        Ok(())
    }
    
    // ===== 工作区清理 =====
    
    /// 清理未追踪的文件
    pub fn clean_untracked(&self, directories: bool, force: bool) -> GtResult<Vec<String>> {
        if !force {
            return Err(GtError::GitOperation {
                message: "清理操作需要 force 参数以确认".to_string()
            });
        }
        
        let statuses = self.inner.statuses(Some(
            git2::StatusOptions::new()
                .include_untracked(true)
                .include_ignored(false)
        ))?;
        
        let mut cleaned_files = Vec::new();
        
        for entry in statuses.iter() {
            if entry.status().contains(git2::Status::WT_NEW) {
                if let Some(path) = entry.path() {
                    let full_path = self.path.join(path);
                    
                    if full_path.is_file() {
                        std::fs::remove_file(&full_path)
                            .map_err(|e| GtError::FileSystemError { path: full_path.clone() })?;
                        cleaned_files.push(path.to_string());
                    } else if full_path.is_dir() && directories {
                        std::fs::remove_dir_all(&full_path)
                            .map_err(|e| GtError::FileSystemError { path: full_path.clone() })?;
                        cleaned_files.push(path.to_string());
                    }
                }
            }
        }
        
        Ok(cleaned_files)
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