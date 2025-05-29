//! 工具函数
//! 
//! 提供通用的工具函数和辅助功能。

use crate::error::{GtResult, GtError};

/// 检查命令是否存在
pub fn command_exists(cmd: &str) -> bool {
    std::process::Command::new("which")
        .arg(cmd)
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

/// 获取当前时间戳
pub fn current_timestamp() -> String {
    chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC").to_string()
}

/// 验证分支名称
pub fn validate_branch_name(name: &str) -> GtResult<()> {
    if name.is_empty() {
        return Err(GtError::InvalidBranchName { 
            name: name.to_string() 
        });
    }
    
    // Git 分支名称基本验证
    if name.contains("..") || name.starts_with('-') || name.ends_with('.') {
        return Err(GtError::InvalidBranchName { 
            name: name.to_string() 
        });
    }
    
    Ok(())
} 