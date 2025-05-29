//! Save 命令实现
//! 
//! 对应 gw save，用于保存当前工作 (add + commit)。

use crate::error::{GtResult, GtError};

/// Save 命令
pub struct SaveCommand {
    message: Option<String>,
    edit: bool,
    files: Vec<String>,
}

impl SaveCommand {
    /// 创建新的 Save 命令
    pub fn new(message: Option<String>, edit: bool, files: Vec<String>) -> Self {
        Self { message, edit, files }
    }
    
    /// 执行命令
    pub async fn execute(self) -> GtResult<()> {
        // TODO: 实现 save 命令逻辑
        Err(GtError::NotImplemented { 
            feature: "save command".to_string() 
        })
    }
} 