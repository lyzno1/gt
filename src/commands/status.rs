//! Status 命令实现
//! 
//! 显示仓库状态信息。

use crate::error::{GtResult, GtError};

/// Status 命令
pub struct StatusCommand {
    remote: bool,
    log: bool,
}

impl StatusCommand {
    /// 创建新的 Status 命令
    pub fn new(remote: bool, log: bool) -> Self {
        Self { remote, log }
    }
    
    /// 执行命令
    pub async fn execute(self) -> GtResult<()> {
        // TODO: 实现 status 命令逻辑
        Err(GtError::NotImplemented { 
            feature: "status command".to_string() 
        })
    }
} 