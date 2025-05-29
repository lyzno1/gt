//! SP 命令实现
//! 
//! 对应 gw sp，用于快速保存并推送变更。

use crate::error::{GtError, GtResult};
use crate::commands::SaveCommand;
use crate::git::GitOps;
use crate::config::ConfigManager;
use crate::ui::{print_step, print_success};

/// SP 命令 - 保存并推送变更
pub struct SpCommand {
    message: Option<String>,
    edit: bool,
    files: Vec<String>,
}

impl SpCommand {
    pub fn new(message: Option<String>, edit: bool, files: Vec<String>) -> Self {
        Self { message, edit, files }
    }
    
    pub async fn execute(self) -> GtResult<()> {
        print_step("开始 SP 操作 (保存并推送)...");
        
        let git_ops = GitOps::new()?;
        
        // 1. 检查是否在Git仓库中
        if !git_ops.is_git_repo() {
            return Err(GtError::NotGitRepository);
        }
        
        // 2. 获取当前分支
        let current_branch = git_ops.current_branch()?;
        
        // 3. 步骤 1/2: 执行 save 逻辑 (add + commit)
        print_step("步骤 1/2: 保存当前工作区的所有变更 (gt save)...");
        let save_cmd = SaveCommand::new(self.message.clone(), self.edit, self.files.clone());
        save_cmd.execute().await?;
        print_success("变更已成功保存");
        
        // 4. 步骤 2/2: 推送到远程
        print_step(&format!("步骤 2/2: 推送当前分支 '{}' 到远程 (gt push)...", current_branch));
        
        // 获取配置
        let config_manager = ConfigManager::new(git_ops.repository())?;
        let config = config_manager.repo_config();
        
        // 推送当前分支
        crate::git::network::push_with_retry(
            git_ops.repository(),
            &config.remote_name,
            Some(&current_branch)
        )?;
        
        print_success(&format!("当前分支 '{}' 已成功推送到远程", current_branch));
        print_success("=== 'gt sp' (保存并推送) 操作成功完成 ===");
        
        Ok(())
    }
} 