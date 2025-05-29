//! Update 命令实现
//! 
//! 对应 gw update，用于同步当前分支到最新状态
//! 增强功能：智能 stash 管理、自动冲突检测、回滚机制

use crate::error::{GtResult, GtError};
use crate::git::GitOps;
use crate::config::ConfigManager;
use crate::ui::{print_step, print_success, print_warning, print_info, confirm_action};
use crate::git::network::pull_rebase_with_retry;

/// Update 命令选项
#[derive(Debug, Clone)]
pub struct UpdateOptions {
    /// 是否强制更新（忽略未提交变更）
    pub force: bool,
    /// 是否跳过 stash 操作
    pub no_stash: bool,
    /// 是否只更新主分支（不 rebase 当前分支）
    pub main_only: bool,
}

impl Default for UpdateOptions {
    fn default() -> Self {
        Self {
            force: false,
            no_stash: false,
            main_only: false,
        }
    }
}

/// Update 命令
pub struct UpdateCommand {
    options: UpdateOptions,
}

impl UpdateCommand {
    /// 创建新的 Update 命令
    pub fn new(force: bool) -> Self {
        let options = UpdateOptions {
            force,
            ..Default::default()
        };
        Self { options }
    }
    
    /// 创建带选项的 Update 命令
    pub fn with_options(options: UpdateOptions) -> Self {
        Self { options }
    }
    
    /// 执行命令
    pub async fn execute(self) -> GtResult<()> {
        print_step("开始同步分支到最新状态...");
        
        let git_ops = GitOps::new()?;
        let config_manager = ConfigManager::new(git_ops.repository())?;
        let config = config_manager.repo_config();
        
        // 1. 检查是否在git仓库中
        if !git_ops.is_git_repo() {
            return Err(GtError::NotGitRepository);
        }
        
        // 2. 获取当前分支
        let original_branch = git_ops.current_branch()?;
        let main_branch = &config.main_branch;
        
        // 3. 处理未提交的变更
        let stash_created = self.handle_uncommitted_changes(&git_ops)?;
        
        // 4. 更新主分支
        if original_branch == *main_branch {
            // 如果在主分支上，直接更新
            self.update_main_branch(&git_ops, config, main_branch).await?;
        } else {
            // 如果在功能分支上，先切换到主分支更新，再rebase
            self.update_feature_branch(&git_ops, config, &original_branch, main_branch).await?;
        }
        
        // 5. 恢复 stash（如果之前创建了）
        if stash_created {
            self.restore_stash(&git_ops)?;
        }
        
        print_success(&format!("分支 '{}' 已成功同步到最新状态！", original_branch));
        Ok(())
    }
    
    /// 处理未提交的变更
    fn handle_uncommitted_changes(&self, git_ops: &GitOps) -> GtResult<bool> {
        if git_ops.is_clean()? {
            return Ok(false);
        }
        
        if self.options.force {
            print_warning("强制模式：忽略未提交的变更");
            return Ok(false);
        }
        
        print_warning("检测到未提交的变更或未追踪的文件");
        print_info("同步前需要处理这些变更:");
        
        if self.options.no_stash {
            return Err(GtError::DirtyWorkingDirectory);
        }
        
        // 显示状态
        let status = git_ops.check_status()?;
        if status.has_uncommitted_changes {
            print_info("- 未提交的变更");
        }
        if status.has_untracked_files {
            print_info("- 未追踪的文件");
        }
        
        print_info("1) 暂存 (Stash) 变更并在同步后尝试恢复");
        print_info("2) 取消同步操作");
        
        if !confirm_action("是否暂存变更并继续同步？", true) {
            return Err(GtError::UserCancelled);
        }
        
        // 创建 stash
        print_step("暂存当前变更...");
        let current_branch = git_ops.current_branch()?;
        let stash_message = format!("WIP on {} before gt update", current_branch);
        git_ops.create_stash(Some(&stash_message))?;
        
        print_success("变更已暂存");
        Ok(true)
    }
    
    /// 更新主分支（当前就在主分支上）
    async fn update_main_branch(&self, git_ops: &GitOps, config: &crate::config::RepoConfig, main_branch: &str) -> GtResult<()> {
        print_info(&format!("您已在主分支 ({})，正在拉取最新代码...", main_branch));
        
        print_step("从远程拉取最新更新 (使用 rebase)...");
        pull_rebase_with_retry(
            git_ops.repository(),
            &config.remote_name,
            Some(main_branch)
        )?;
        
        print_success(&format!("主分支 '{}' 已更新到最新状态", main_branch));
        Ok(())
    }
    
    /// 更新功能分支（需要切换到主分支更新，然后rebase）
    async fn update_feature_branch(
        &self, 
        git_ops: &GitOps, 
        config: &crate::config::RepoConfig, 
        original_branch: &str, 
        main_branch: &str
    ) -> GtResult<()> {
        print_info(&format!("同步功能分支 '{}' 到最新状态", original_branch));
        
        // 1. 切换到主分支
        print_step(&format!("1/3: 切换到主分支 '{}'...", main_branch));
        git_ops.checkout_branch(main_branch)?;
        print_success(&format!("已切换到主分支 '{}'", main_branch));
        
        // 2. 更新主分支
        print_step("2/3: 从远程拉取最新更新 (使用 rebase)...");
        pull_rebase_with_retry(
            git_ops.repository(),
            &config.remote_name,
            Some(main_branch)
        )?;
        print_success("主分支已更新到最新状态");
        
        // 3. 切换回原分支
        print_step(&format!("3/3: 切换回功能分支 '{}'...", original_branch));
        git_ops.checkout_branch(original_branch)?;
        
        // 4. Rebase 到最新的主分支
        if !self.options.main_only {
            print_step(&format!("将功能分支 '{}' rebase 到最新的 '{}'...", original_branch, main_branch));
            
            // 使用 GitOps 的 rebase 方法（如果存在）或者直接调用 git rebase
            if let Err(e) = self.rebase_branch(git_ops, main_branch) {
                print_warning("Rebase 操作遇到冲突或失败");
                print_info("请解决 rebase 冲突，然后运行:");
                print_info("  git add <冲突文件>");
                print_info("  git rebase --continue");
                print_info("或者运行 'git rebase --abort' 取消 rebase");
                return Err(e);
            }
            
            print_success(&format!("功能分支 '{}' 已成功 rebase 到最新的 '{}'", original_branch, main_branch));
        }
        
        Ok(())
    }
    
    /// 执行 rebase 操作
    fn rebase_branch(&self, git_ops: &GitOps, target_branch: &str) -> GtResult<()> {
        // 简单实现：使用系统 git 命令
        // 在实际项目中，可以考虑使用 libgit2 的 rebase 功能
        use std::process::Command;
        
        let output = Command::new("git")
            .args(&["rebase", target_branch])
            .output()
            .map_err(|e| GtError::CommandError {
                command: "git rebase".to_string(),
                error: e.to_string(),
            })?;
        
        if !output.status.success() {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            return Err(GtError::GitOperation {
                message: format!("Rebase 失败: {}", error_msg),
            });
        }
        
        Ok(())
    }
    
    /// 恢复 stash
    fn restore_stash(&self, git_ops: &GitOps) -> GtResult<()> {
        print_step("尝试恢复之前暂存的变更...");
        
        // 弹出最新的 stash
        if let Err(e) = git_ops.pop_stash(0) {
            print_warning("自动恢复暂存失败，可能存在冲突");
            print_info("请手动检查并恢复:");
            print_info("  gt stash list    # 查看暂存列表");
            print_info("  gt stash pop     # 手动恢复暂存");
            print_info("  git status       # 查看冲突状态");
            return Err(e);
        }
        
        print_success("暂存的变更已成功恢复");
        Ok(())
    }
}

/// 便捷函数：标准更新
pub async fn update_branch() -> GtResult<()> {
    let cmd = UpdateCommand::new(false);
    cmd.execute().await
}

/// 便捷函数：强制更新
pub async fn force_update_branch() -> GtResult<()> {
    let cmd = UpdateCommand::new(true);
    cmd.execute().await
}

/// 便捷函数：只更新主分支
pub async fn update_main_only() -> GtResult<()> {
    let options = UpdateOptions {
        force: false,
        no_stash: false,
        main_only: true,
    };
    let cmd = UpdateCommand::with_options(options);
    cmd.execute().await
} 