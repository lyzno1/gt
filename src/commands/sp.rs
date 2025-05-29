//! SP 命令实现
//! 
//! 对应 gw sp，用于快速保存并推送变更。

use crate::error::{GtError, GtResult};

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
        // TODO: 实现 sp 命令逻辑
        // 1. 检查是否在Git仓库中
        // 2. 执行 save 逻辑 (add + commit)
        // 3. 执行 push 逻辑
        Err(GtError::NotImplemented {
            feature: "sp command".to_string()
        })
    }
} 