//! 用户界面
//! 
//! 提供终端用户界面和交互功能。

use crate::error::{GtResult, GtError};

/// UI 管理器
pub struct UiManager {
    // TODO: 添加字段
}

impl UiManager {
    /// 创建新的 UI 管理器
    pub fn new() -> Self {
        Self {
            // TODO: 初始化字段
        }
    }
    
    /// 显示进度
    pub async fn show_progress(&self, _message: &str) -> GtResult<()> {
        // TODO: 实现进度显示逻辑
        Err(GtError::NotImplemented { 
            feature: "progress display".to_string() 
        })
    }
    
    /// 用户确认
    pub async fn confirm(&self, _message: &str) -> GtResult<bool> {
        // TODO: 实现用户确认逻辑
        Err(GtError::NotImplemented { 
            feature: "user confirmation".to_string() 
        })
    }
} 