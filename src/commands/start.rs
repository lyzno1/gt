//! Start 命令实现
//! 
//! 对应 gw start，用于开始新的功能分支。

use crate::error::{GtResult, GtError};

/// Start 命令
pub struct StartCommand {
    branch: String,
    base: String,
    local: bool,
}

impl StartCommand {
    /// 创建新的 Start 命令
    pub fn new(branch: String, base: String, local: bool) -> Self {
        Self { branch, base, local }
    }
    
    /// 执行命令
    pub async fn execute(self) -> GtResult<()> {
        // TODO: 实现 start 命令逻辑
        Err(GtError::NotImplemented { 
            feature: "start command".to_string() 
        })
    }
} 