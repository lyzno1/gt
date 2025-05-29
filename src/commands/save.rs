//! Save 命令实现
//! 
//! 对应 gw save，用于保存当前工作 (add + commit)
//! 增强功能：智能文件选择、交互式提交、编辑器集成

use crate::error::{GtResult, GtError};
use crate::git::GitOps;
use crate::ui::{print_step, print_success, print_warning, print_info, confirm_action, prompt_input};

/// Save 命令选项
#[derive(Debug, Clone)]
pub struct SaveOptions {
    /// 提交信息
    pub message: Option<String>,
    /// 强制使用编辑器
    pub edit: bool,
    /// 指定要添加的文件
    pub files: Vec<String>,
    /// 是否添加所有文件
    pub add_all: bool,
}

impl Default for SaveOptions {
    fn default() -> Self {
        Self {
            message: None,
            edit: false,
            files: Vec::new(),
            add_all: true,
        }
    }
}

/// Save 命令
pub struct SaveCommand {
    options: SaveOptions,
}

impl SaveCommand {
    /// 创建新的 Save 命令
    pub fn new(message: Option<String>, edit: bool, files: Vec<String>) -> Self {
        let add_all = files.is_empty();
        let options = SaveOptions {
            message,
            edit,
            files,
            add_all,
        };
        Self { options }
    }
    
    /// 执行命令
    pub async fn execute(self) -> GtResult<()> {
        print_step("开始保存当前工作...");
        
        let git_ops = GitOps::new()?;
        
        // 1. 检查是否在git仓库中
        if !git_ops.is_git_repo() {
            return Err(GtError::NotGitRepository);
        }
        
        // 2. 添加文件到暂存区
        self.add_files(&git_ops)?;
        
        // 3. 检查是否有暂存的变更
        let status = git_ops.check_status()?;
        if !status.has_staged_changes {
            if self.options.add_all {
                print_warning("没有检测到需要保存的变更");
            } else {
                print_warning("指定的文件没有变更或未能添加到暂存区");
            }
            return Ok(());
        }
        
        // 4. 提交变更
        self.commit_changes(&git_ops)?;
        
        print_success("变更已成功保存！");
        Ok(())
    }
    
    /// 添加文件到暂存区
    fn add_files(&self, git_ops: &GitOps) -> GtResult<()> {
        if self.options.add_all {
            print_step("添加所有变更到暂存区...");
            git_ops.add_all()?;
        } else if !self.options.files.is_empty() {
            print_step(&format!("添加指定文件到暂存区: {}", self.options.files.join(", ")));
            let file_refs: Vec<&str> = self.options.files.iter().map(|s| s.as_str()).collect();
            git_ops.add_files(&file_refs)?;
        } else {
            // 简化版本：如果没有指定文件，默认添加所有变更
            print_step("没有指定文件，添加所有变更到暂存区...");
            git_ops.add_all()?;
        }
        
        Ok(())
    }
    
    /// 提交变更
    fn commit_changes(&self, git_ops: &GitOps) -> GtResult<()> {
        let message = if let Some(ref msg) = self.options.message {
            if self.options.edit {
                // 有消息但要求编辑，使用交互式编辑
                self.get_commit_message_interactive(Some(msg.clone()))?
            } else {
                // 直接使用提供的消息
                print_step("使用提供的提交信息进行提交...");
                msg.clone()
            }
        } else if self.options.edit {
            // 没有消息但要求编辑器，使用交互式编辑
            self.get_commit_message_interactive(None)?
        } else {
            // 交互式输入提交信息
            self.get_commit_message_interactive(None)?
        };
        
        // 执行提交
        print_step("提交变更...");
        git_ops.create_commit(&message)?;
        
        Ok(())
    }
    
    /// 获取提交信息（交互式）
    fn get_commit_message_interactive(&self, initial_message: Option<String>) -> GtResult<String> {
        if let Some(msg) = initial_message {
            print_info(&format!("当前提交信息: {}", msg));
            if confirm_action("是否使用当前信息？", true) {
                return Ok(msg);
            }
        }
        
        print_info("请输入提交信息 (空行结束):");
        
        let mut lines = Vec::new();
        loop {
            let line = prompt_input("", None);
            if line.is_empty() {
                break;
            }
            lines.push(line);
        }
        
        if lines.is_empty() {
            return Err(GtError::EmptyCommitMessage);
        }
        
        let message = lines.join("\n");
        Ok(message)
    }
}

/// 便捷函数：快速保存所有变更
pub async fn save_all(message: Option<String>) -> GtResult<()> {
    let cmd = SaveCommand::new(message, false, Vec::new());
    cmd.execute().await
}

/// 便捷函数：交互式保存
pub async fn save_interactive() -> GtResult<()> {
    let cmd = SaveCommand::new(None, false, Vec::new());
    cmd.execute().await
}

/// 便捷函数：使用编辑器保存
pub async fn save_with_editor() -> GtResult<()> {
    let cmd = SaveCommand::new(None, true, Vec::new());
    cmd.execute().await
} 