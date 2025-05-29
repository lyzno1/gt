//! 错误类型定义
//! 
//! 定义了 gt 工具中所有可能的错误类型，提供丰富的错误信息。

use thiserror::Error;
use std::path::PathBuf;

/// gt 工具的主要错误类型
#[derive(Error, Debug)]
pub enum GtError {
    /// 不在 Git 仓库中
    #[error("当前目录不是 Git 仓库")]
    NotInGitRepo,
    
    /// 分支相关错误
    #[error("分支 '{branch}' 不存在")]
    BranchNotFound { branch: String },
    
    #[error("分支 '{branch}' 已存在")]
    BranchAlreadyExists { branch: String },
    
    #[error("无法获取当前分支信息")]
    CurrentBranchNotFound,
    
    /// 工作区状态错误
    #[error("有未提交的变更，请先处理")]
    UncommittedChanges,
    
    #[error("有未追踪的文件，请先处理")]
    UntrackedFiles,
    
    #[error("工作区不干净，无法执行操作")]
    DirtyWorkingDirectory,
    
    /// 远程操作错误
    #[error("远程仓库 '{remote}' 不存在")]
    RemoteNotFound { remote: String },
    
    #[error("远程操作失败: {message}")]
    RemoteError { message: String },
    
    #[error("推送失败: {reason}")]
    PushFailed { reason: String },
    
    #[error("拉取失败: {reason}")]
    PullFailed { reason: String },
    
    #[error("网络连接失败，已重试 {attempts} 次")]
    NetworkTimeout { attempts: u32 },
    
    /// GitHub 相关错误
    #[error("GitHub API 错误: {0}")]
    GitHubError(#[from] octocrab::Error),
    
    #[error("GitHub 认证失败")]
    GitHubAuthError,
    
    #[error("Pull Request 创建失败: {reason}")]
    PullRequestError { reason: String },
    
    /// 配置错误
    #[error("配置错误: {message}")]
    ConfigError { message: String },
    
    #[error("配置文件不存在: {path}")]
    ConfigFileNotFound { path: PathBuf },
    
    #[error("配置文件格式错误: {reason}")]
    ConfigParseError { reason: String },
    
    /// 用户交互错误
    #[error("用户取消操作")]
    UserCancelled,
    
    #[error("无效的用户输入: {input}")]
    InvalidInput { input: String },
    
    /// 工作流错误
    #[error("工作流验证失败: {reason}")]
    WorkflowValidationError { reason: String },
    
    #[error("工作流步骤 '{step}' 执行失败: {reason}")]
    WorkflowStepError { step: String, reason: String },
    
    #[error("前置条件不满足: {condition}")]
    PreconditionFailed { condition: String },
    
    /// 文件系统错误
    #[error("文件操作失败: {path}")]
    FileSystemError { path: PathBuf },
    
    #[error("权限不足: {operation}")]
    PermissionDenied { operation: String },
    
    /// Git 底层错误
    #[error("Git 操作失败: {0}")]
    GitError(#[from] git2::Error),
    
    /// IO 错误
    #[error("IO 错误: {0}")]
    IoError(#[from] std::io::Error),
    
    /// 序列化错误
    #[error("序列化错误: {0}")]
    SerializationError(#[from] toml::de::Error),
    
    /// 其他错误
    #[error("内部错误: {message}")]
    InternalError { message: String },
    
    #[error("功能未实现: {feature}")]
    NotImplemented { feature: String },
    
    /// 无效的分支名称
    #[error("无效的分支名称: {name}")]
    InvalidBranchName { name: String },
}

/// gt 工具的结果类型
pub type GtResult<T> = Result<T, GtError>;

impl GtError {
    /// 创建配置错误
    pub fn config_error(message: impl Into<String>) -> Self {
        Self::ConfigError {
            message: message.into(),
        }
    }
    
    /// 创建工作流错误
    pub fn workflow_error(reason: impl Into<String>) -> Self {
        Self::WorkflowValidationError {
            reason: reason.into(),
        }
    }
    
    /// 创建内部错误
    pub fn internal_error(message: impl Into<String>) -> Self {
        Self::InternalError {
            message: message.into(),
        }
    }
    
    /// 检查是否为用户可恢复的错误
    pub fn is_recoverable(&self) -> bool {
        matches!(
            self,
            Self::UncommittedChanges
                | Self::UntrackedFiles
                | Self::NetworkTimeout { .. }
                | Self::UserCancelled
                | Self::InvalidInput { .. }
        )
    }
    
    /// 检查是否需要用户确认
    pub fn requires_confirmation(&self) -> bool {
        matches!(
            self,
            Self::UncommittedChanges | Self::UntrackedFiles | Self::DirtyWorkingDirectory
        )
    }
    
    /// 获取错误的严重程度
    pub fn severity(&self) -> ErrorSeverity {
        match self {
            Self::UserCancelled | Self::InvalidInput { .. } => ErrorSeverity::Info,
            Self::UncommittedChanges | Self::UntrackedFiles => ErrorSeverity::Warning,
            Self::NetworkTimeout { .. } | Self::RemoteError { .. } => ErrorSeverity::Error,
            Self::GitError(_) | Self::IoError(_) => ErrorSeverity::Critical,
            _ => ErrorSeverity::Error,
        }
    }
}

/// 错误严重程度
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorSeverity {
    Info,
    Warning,
    Error,
    Critical,
}

impl ErrorSeverity {
    /// 获取对应的颜色
    pub fn color(&self) -> colored::Color {
        match self {
            Self::Info => colored::Color::Blue,
            Self::Warning => colored::Color::Yellow,
            Self::Error => colored::Color::Red,
            Self::Critical => colored::Color::Magenta,
        }
    }
    
    /// 获取对应的图标
    pub fn icon(&self) -> &'static str {
        match self {
            Self::Info => "ℹ️",
            Self::Warning => "⚠️",
            Self::Error => "❌",
            Self::Critical => "💥",
        }
    }
} 