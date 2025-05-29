//! Clean 命令实现
//! 
//! 对应 gw rm，用于清理分支。

use crate::error::{GtResult, GtError};

/// Clean 命令
pub struct CleanCommand {
    branch: String,
    force: bool,
}

impl CleanCommand {
    /// 创建新的 Clean 命令
    pub fn new(branch: String, force: bool) -> Self {
        Self { branch, force }
    }
    
    /// 执行命令
    pub async fn execute(self) -> GtResult<()> {
        // TODO: 实现 clean 命令逻辑
        Err(GtError::NotImplemented { 
            feature: "clean command".to_string() 
        })
    }
} 