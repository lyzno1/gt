//! Ship 命令实现
//! 
//! 对应 gw submit，用于"发货"完成的功能到主分支
//! 增强功能：智能 PR 创建、自动合并策略、分支清理、GitHub 集成

use crate::error::{GtResult, GtError};
use crate::git::GitOps;
use crate::config::ConfigManager;
use crate::ui::{print_step, print_success, print_warning, print_info, confirm_action};
use crate::git::network::push_with_retry;
use crate::github::{GithubCli, PullRequestManager, CreatePrOptions, MergePrOptions};

/// 合并策略
#[derive(Debug, Clone, Copy)]
pub enum MergeStrategy {
    /// Rebase 合并（推荐）
    Rebase,
    /// Squash 合并（压缩提交）
    Squash,
    /// 普通合并（保留分支结构）
    Merge,
}

impl Default for MergeStrategy {
    fn default() -> Self {
        Self::Rebase
    }
}

impl std::fmt::Display for MergeStrategy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Rebase => write!(f, "rebase"),
            Self::Squash => write!(f, "squash"),
            Self::Merge => write!(f, "merge"),
        }
    }
}

/// Ship 命令选项
#[derive(Debug, Clone)]
pub struct ShipOptions {
    /// 完成后不切换回主分支
    pub no_switch: bool,
    /// 创建 Pull Request
    pub create_pr: bool,
    /// 自动合并 PR
    pub auto_merge: bool,
    /// 合并策略
    pub merge_strategy: MergeStrategy,
    /// 合并后删除分支
    pub delete_branch: bool,
    /// PR 标题（可选）
    pub pr_title: Option<String>,
    /// PR 描述（可选）
    pub pr_body: Option<String>,
}

impl Default for ShipOptions {
    fn default() -> Self {
        Self {
            no_switch: false,
            create_pr: false,
            auto_merge: false,
            merge_strategy: MergeStrategy::default(),
            delete_branch: false,
            pr_title: None,
            pr_body: None,
        }
    }
}

/// Ship 命令
pub struct ShipCommand {
    options: ShipOptions,
}

impl ShipCommand {
    /// 创建新的 Ship 命令
    pub fn new(
        no_switch: bool, 
        pr: bool, 
        merge_strategy: Option<MergeStrategy>, 
        delete_branch: bool
    ) -> Self {
        let auto_merge = merge_strategy.is_some();
        let create_pr = pr || auto_merge;
        
        let options = ShipOptions {
            no_switch,
            create_pr,
            auto_merge,
            merge_strategy: merge_strategy.unwrap_or_default(),
            delete_branch,
            pr_title: None,
            pr_body: None,
        };
        
        Self { options }
    }
    
    /// 创建带选项的 Ship 命令
    pub fn with_options(options: ShipOptions) -> Self {
        Self { options }
    }
    
    /// 执行命令
    pub async fn execute(self) -> GtResult<()> {
        print_step("开始提交工作成果 (Ship)...");
        
        let git_ops = GitOps::new()?;
        let config_manager = ConfigManager::new(git_ops.repository())?;
        let config = config_manager.repo_config();
        
        // 1. 检查是否在git仓库中
        if !git_ops.is_git_repo() {
            return Err(GtError::NotGitRepository);
        }
        
        // 2. 获取当前分支
        let current_branch = git_ops.current_branch()?;
        let main_branch = &config.main_branch;
        
        // 3. 检查是否在主分支上
        if current_branch == *main_branch {
            self.handle_main_branch_ship(&git_ops, config, main_branch).await?;
        } else {
            self.handle_feature_branch_ship(&git_ops, config, &current_branch, main_branch).await?;
        }
        
        print_success("🚀 工作成果已成功提交！");
        Ok(())
    }
    
    /// 处理在主分支上的 ship 操作
    async fn handle_main_branch_ship(
        &self, 
        git_ops: &GitOps, 
        config: &crate::config::RepoConfig, 
        main_branch: &str
    ) -> GtResult<()> {
        print_warning(&format!("您当前在主分支 ({})，ship 命令通常用于功能分支", main_branch));
        
        if !confirm_action("是否仍要继续推送主分支？", false) {
            return Err(GtError::UserCancelled);
        }
        
        // 检查未提交变更
        self.check_uncommitted_changes(&git_ops).await?;
        
        // 推送主分支
        print_step(&format!("推送主分支 '{}' 到远程...", main_branch));
        push_with_retry(git_ops.repository(), &config.remote_name, Some(main_branch))?;
        
        print_success(&format!("主分支 '{}' 已推送到远程", main_branch));
        Ok(())
    }
    
    /// 处理功能分支的 ship 操作
    async fn handle_feature_branch_ship(
        &self,
        git_ops: &GitOps,
        config: &crate::config::RepoConfig,
        current_branch: &str,
        main_branch: &str,
    ) -> GtResult<()> {
        print_info(&format!("准备提交功能分支 '{}' 的工作成果", current_branch));
        
        // 1. 检查未提交变更
        self.check_uncommitted_changes(&git_ops).await?;
        
        // 2. 推送当前分支
        self.push_current_branch(&git_ops, config, current_branch).await?;
        
        // 3. 创建 Pull Request（如果需要）
        let pr_url = if self.options.create_pr {
            Some(self.create_pull_request(&git_ops, config, current_branch, main_branch).await?)
        } else {
            None
        };
        
        // 4. 自动合并 PR（如果需要）
        if self.options.auto_merge && pr_url.is_some() {
            self.auto_merge_pr(&pr_url.unwrap()).await?;
        }
        
        // 5. 切换回主分支（如果需要）
        if !self.options.no_switch {
            self.switch_to_main_branch(&git_ops, config, main_branch).await?;
        }
        
        // 6. 删除功能分支（如果需要）
        if self.options.delete_branch && !self.options.no_switch {
            self.cleanup_feature_branch(&git_ops, current_branch)?;
        }
        
        Ok(())
    }
    
    /// 检查未提交的变更
    async fn check_uncommitted_changes(&self, git_ops: &GitOps) -> GtResult<()> {
        if git_ops.is_clean()? {
            print_info("工作区干净，没有未提交的变更");
            return Ok(());
        }
        
        print_warning("检测到未提交的变更或未追踪的文件");
        print_info("在提交前需要处理这些变更:");
        
        let status = git_ops.check_status()?;
        if status.has_uncommitted_changes {
            print_info("- 未提交的变更");
        }
        if status.has_untracked_files {
            print_info("- 未追踪的文件");
        }
        
        print_info("1) 处理并提交变更");
        print_info("2) 暂存变更（不推荐，推送后 PR 中不包含）");
        print_info("3) 取消 ship 操作");
        
        // 简化处理：要求用户手动提交
        if confirm_action("是否使用 'gt save' 保存这些变更？", true) {
            // 调用 save 命令
            let save_cmd = crate::commands::SaveCommand::new(None, false, Vec::new());
            save_cmd.execute().await?;
            print_success("变更已保存");
        } else {
            return Err(GtError::UserCancelled);
        }
        
        Ok(())
    }
    
    /// 推送当前分支
    async fn push_current_branch(
        &self,
        git_ops: &GitOps,
        config: &crate::config::RepoConfig,
        current_branch: &str,
    ) -> GtResult<()> {
        print_step(&format!("推送分支 '{}' 到远程...", current_branch));
        
        push_with_retry(git_ops.repository(), &config.remote_name, Some(current_branch))?;
        
        print_success(&format!("分支 '{}' 已推送到远程", current_branch));
        Ok(())
    }
    
    /// 创建 Pull Request
    async fn create_pull_request(
        &self,
        _git_ops: &GitOps,
        _config: &crate::config::RepoConfig,
        current_branch: &str,
        main_branch: &str,
    ) -> GtResult<String> {
        print_step("创建 Pull Request...");
        
        // 检查 GitHub CLI 是否可用
        let github_cli = GithubCli::new(false);
        if !github_cli.is_available() {
            print_warning("GitHub CLI (gh) 不可用，请手动在 GitHub 上创建 Pull Request");
            return Ok(format!("https://github.com/your-org/repo/compare/{}...{}", main_branch, current_branch));
        }
        
        // 创建 PR 选项 - 使用正确的字段名称
        let mut pr_options = CreatePrOptions::new(
            current_branch.to_string(),
            main_branch.to_string()
        );
        
        // 设置可选字段
        if let Some(ref title) = self.options.pr_title {
            pr_options = pr_options.with_title(title.clone());
        }
        
        if let Some(ref body) = self.options.pr_body {
            pr_options = pr_options.with_body(body.clone());
        }
        
        // 创建 PR - 移除 await，因为这不是异步方法
        let pr_manager = PullRequestManager::new(github_cli);
        let pr = pr_manager.create_pr(pr_options)?;
        
        print_success(&format!("Pull Request 已创建: {}", pr.url));
        Ok(pr.url)
    }
    
    /// 自动合并 PR
    async fn auto_merge_pr(&self, pr_url: &str) -> GtResult<()> {
        print_step(&format!("自动合并 PR (策略: {})...", self.options.merge_strategy));
        
        let github_cli = GithubCli::new(false);
        let pr_manager = PullRequestManager::new(github_cli);
        
        // 创建合并选项 - 使用正确的字段名称
        let mut merge_options = MergePrOptions::new(self.options.merge_strategy.into());
        
        if self.options.delete_branch {
            merge_options = merge_options.delete_branch();
        }
        
        merge_options = merge_options.enable_auto_merge();
        
        // 合并 PR - 移除 await，使用正确的方法名
        pr_manager.merge_pr(pr_url, merge_options)?;
        
        print_success("Pull Request 已自动合并");
        Ok(())
    }
    
    /// 切换回主分支
    async fn switch_to_main_branch(
        &self,
        git_ops: &GitOps,
        config: &crate::config::RepoConfig,
        main_branch: &str,
    ) -> GtResult<()> {
        print_step(&format!("切换回主分支 '{}'...", main_branch));
        
        git_ops.checkout_branch(main_branch)?;
        
        // 拉取最新更新
        print_step("拉取主分支最新更新...");
        crate::git::network::pull_rebase_with_retry(
            git_ops.repository(),
            &config.remote_name,
            Some(main_branch)
        )?;
        
        print_success(&format!("已切换到主分支 '{}' 并更新到最新状态", main_branch));
        Ok(())
    }
    
    /// 清理功能分支
    fn cleanup_feature_branch(&self, git_ops: &GitOps, branch_name: &str) -> GtResult<()> {
        print_step(&format!("删除本地功能分支 '{}'...", branch_name));
        
        git_ops.delete_branch(branch_name, false)?;
        
        print_success(&format!("本地分支 '{}' 已删除", branch_name));
        Ok(())
    }
}

// 为 MergeStrategy 实现转换到 GitHub API 格式
impl From<MergeStrategy> for crate::github::pr::MergeStrategy {
    fn from(strategy: MergeStrategy) -> Self {
        match strategy {
            MergeStrategy::Rebase => Self::Rebase,
            MergeStrategy::Squash => Self::Squash,
            MergeStrategy::Merge => Self::Merge,
        }
    }
}

/// 便捷函数：简单推送
pub async fn ship_simple() -> GtResult<()> {
    let cmd = ShipCommand::new(false, false, None, false);
    cmd.execute().await
}

/// 便捷函数：创建 PR
pub async fn ship_with_pr() -> GtResult<()> {
    let cmd = ShipCommand::new(false, true, None, false);
    cmd.execute().await
}

/// 便捷函数：自动合并
pub async fn ship_auto_merge() -> GtResult<()> {
    let cmd = ShipCommand::new(false, true, Some(MergeStrategy::Rebase), true);
    cmd.execute().await
} 