//! Ship 命令实现
//! 
//! 对应 gw submit，用于提交工作成果。

use crate::error::{GtResult, GtError};

/// 合并策略
#[derive(Debug, Clone, Copy)]
pub enum MergeStrategy {
    Rebase,
    Squash,
    Merge,
}

/// Ship 命令
pub struct ShipCommand {
    no_switch: bool,
    pr: bool,
    merge_strategy: Option<MergeStrategy>,
    delete_branch: bool,
}

impl ShipCommand {
    /// 创建新的 Ship 命令
    pub fn new(
        no_switch: bool, 
        pr: bool, 
        merge_strategy: Option<MergeStrategy>, 
        delete_branch: bool
    ) -> Self {
        Self { no_switch, pr, merge_strategy, delete_branch }
    }
    
    /// 执行命令
    pub async fn execute(self) -> GtResult<()> {
        // TODO: 实现 ship 命令逻辑
        Err(GtError::NotImplemented { 
            feature: "ship command".to_string() 
        })
    }
} 