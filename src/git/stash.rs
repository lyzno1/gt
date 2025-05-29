//! Git stash 操作
//! 
//! 提供工作区暂存和恢复操作。

use crate::error::{GtResult, GtError};

/// Stash 操作
pub struct StashManager {
    // TODO: 添加字段
}

impl StashManager {
    /// 创建新的 stash 管理器
    pub fn new() -> Self {
        Self {
            // TODO: 初始化字段
        }
    }
    
    /// 暂存当前工作区
    pub async fn stash(&self, _message: Option<&str>) -> GtResult<()> {
        // TODO: 实现 stash 逻辑
        Err(GtError::NotImplemented { 
            feature: "stash".to_string() 
        })
    }
} 