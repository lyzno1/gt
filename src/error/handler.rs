//! 错误处理器
//! 
//! 提供统一的错误处理和用户反馈机制。

use super::types::{GtError, GtResult, ErrorSeverity};
use super::recovery::RecoveryStrategy;
use colored::Colorize;
use std::io::{self, Write};

/// 错误处理器
pub struct ErrorHandler {
    verbose: bool,
    interactive: bool,
}

impl ErrorHandler {
    /// 创建新的错误处理器
    pub fn new(verbose: bool, interactive: bool) -> Self {
        Self {
            verbose,
            interactive,
        }
    }
    
    /// 处理错误并返回是否应该继续执行
    pub fn handle_error(&self, error: &GtError) -> bool {
        self.display_error(error);
        
        if error.is_recoverable() && self.interactive {
            self.try_recovery(error)
        } else {
            false
        }
    }
    
    /// 显示错误信息
    pub fn display_error(&self, error: &GtError) {
        let severity = error.severity();
        let icon = severity.icon();
        let color = severity.color();
        
        eprintln!(
            "{} {}",
            icon,
            format!("{}", error).color(color).bold()
        );
        
        if self.verbose {
            self.display_verbose_error(error);
        }
        
        // 提供解决建议
        if let Some(suggestion) = self.get_suggestion(error) {
            eprintln!("{} {}", "💡".blue(), suggestion.blue());
        }
    }
    
    /// 显示详细错误信息
    fn display_verbose_error(&self, error: &GtError) {
        match error {
            GtError::GitError(git_err) => {
                eprintln!("  Git 错误详情: {}", git_err);
            }
            GtError::IoError { operation, error } => {
                eprintln!("  IO 错误详情: {} - {}", operation, error);
            }
            GtError::NetworkTimeout { attempts } => {
                eprintln!("  网络重试次数: {}", attempts);
            }
            _ => {}
        }
    }
    
    /// 获取错误解决建议
    fn get_suggestion(&self, error: &GtError) -> Option<String> {
        match error {
            GtError::NotInGitRepo => {
                Some("请在 Git 仓库目录中运行此命令，或使用 'gt init' 初始化仓库".to_string())
            }
            GtError::UncommittedChanges => {
                Some("使用 'gt save' 提交变更，或使用 'gt stash' 暂存变更".to_string())
            }
            GtError::UntrackedFiles => {
                Some("使用 'gt save' 添加并提交文件，或将文件添加到 .gitignore".to_string())
            }
            GtError::BranchNotFound { branch } => {
                Some(format!("使用 'gt start {}' 创建分支", branch))
            }
            GtError::RemoteNotFound { remote } => {
                Some(format!("使用 'gt remote add {} <url>' 添加远程仓库", remote))
            }
            GtError::NetworkTimeout { .. } => {
                Some("检查网络连接，或稍后重试".to_string())
            }
            GtError::GitHubAuthError => {
                Some("使用 'gh auth login' 登录 GitHub，或检查访问令牌".to_string())
            }
            _ => None,
        }
    }
    
    /// 尝试错误恢复
    fn try_recovery(&self, error: &GtError) -> bool {
        let strategy = RecoveryStrategy::for_error(error);
        
        match strategy {
            RecoveryStrategy::Prompt => self.prompt_user_action(error),
            RecoveryStrategy::AutoRetry => {
                println!("🔄 自动重试中...");
                true
            }
            RecoveryStrategy::Ignore => {
                println!("⏭️  跳过此错误");
                true
            }
            RecoveryStrategy::Abort => false,
        }
    }
    
    /// 提示用户操作
    fn prompt_user_action(&self, error: &GtError) -> bool {
        match error {
            GtError::UncommittedChanges => {
                self.prompt_uncommitted_changes()
            }
            GtError::UntrackedFiles => {
                self.prompt_untracked_files()
            }
            _ => {
                self.prompt_continue()
            }
        }
    }
    
    /// 提示处理未提交变更
    fn prompt_uncommitted_changes(&self) -> bool {
        use dialoguer::Select;
        
        let choices = vec![
            "提交变更",
            "暂存变更",
            "取消操作",
        ];
        
        match Select::new()
            .with_prompt("如何处理未提交的变更？")
            .items(&choices)
            .default(0)
            .interact()
        {
            Ok(0) => {
                println!("📝 请使用 'gt save' 提交变更后重试");
                false
            }
            Ok(1) => {
                println!("📦 请使用 'gt stash' 暂存变更后重试");
                false
            }
            _ => false,
        }
    }
    
    /// 提示处理未追踪文件
    fn prompt_untracked_files(&self) -> bool {
        use dialoguer::Confirm;
        
        match Confirm::new()
            .with_prompt("是否要添加未追踪的文件？")
            .default(true)
            .interact()
        {
            Ok(true) => {
                println!("📁 请使用 'gt save' 添加并提交文件后重试");
                false
            }
            Ok(false) => {
                println!("⏭️  忽略未追踪文件，继续执行");
                true
            }
            Err(_) => false,
        }
    }
    
    /// 提示是否继续
    fn prompt_continue(&self) -> bool {
        use dialoguer::Confirm;
        
        match Confirm::new()
            .with_prompt("是否要继续执行？")
            .default(false)
            .interact()
        {
            Ok(result) => result,
            Err(_) => false,
        }
    }
}

impl Default for ErrorHandler {
    fn default() -> Self {
        Self::new(false, true)
    }
} 