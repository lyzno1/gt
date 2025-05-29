//! 错误恢复策略
//! 
//! 定义不同错误的恢复策略和自动处理机制。

use super::types::GtError;

/// 错误恢复策略
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecoveryStrategy {
    /// 提示用户选择操作
    Prompt,
    /// 自动重试
    AutoRetry,
    /// 忽略错误继续执行
    Ignore,
    /// 中止操作
    Abort,
}

impl RecoveryStrategy {
    /// 根据错误类型确定恢复策略
    pub fn for_error(error: &GtError) -> Self {
        match error {
            // 需要用户确认的错误
            GtError::UncommittedChanges | GtError::UntrackedFiles => Self::Prompt,
            
            // 可以自动重试的错误
            GtError::NetworkTimeout { .. } | GtError::RemoteError { .. } => Self::AutoRetry,
            
            // 用户取消操作
            GtError::UserCancelled => Self::Abort,
            
            // 严重错误，需要中止
            GtError::NotInGitRepo
            | GtError::GitError(_)
            | GtError::IoError { .. }
            | GtError::PermissionDenied { .. } => Self::Abort,
            
            // 其他错误提示用户
            _ => Self::Prompt,
        }
    }
    
    /// 检查是否应该自动处理
    pub fn is_automatic(self) -> bool {
        matches!(self, Self::AutoRetry | Self::Ignore)
    }
    
    /// 检查是否需要用户交互
    pub fn requires_interaction(self) -> bool {
        matches!(self, Self::Prompt)
    }
} 