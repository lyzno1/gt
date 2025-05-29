//! Git 状态检查模块
//! 
//! 提供工作区状态信息的类型定义。

/// 工作区状态
#[derive(Debug, Clone)]
pub struct WorkingTreeStatus {
    /// 是否有未提交的变更
    pub has_uncommitted_changes: bool,
    /// 是否有未追踪的文件
    pub has_untracked_files: bool,
    /// 是否有暂存的变更
    pub has_staged_changes: bool,
    /// 修改的文件数量
    pub modified_files: usize,
    /// 新增的文件数量
    pub added_files: usize,
    /// 删除的文件数量
    pub deleted_files: usize,
    /// 未追踪的文件数量
    pub untracked_files: usize,
}

impl WorkingTreeStatus {
    /// 工作区是否干净
    pub fn is_clean(&self) -> bool {
        !self.has_uncommitted_changes && !self.has_untracked_files && !self.has_staged_changes
    }
    
    /// 是否需要提交
    pub fn needs_commit(&self) -> bool {
        self.has_staged_changes
    }
    
    /// 是否有未处理的变更
    pub fn has_pending_changes(&self) -> bool {
        self.has_uncommitted_changes || self.has_untracked_files
    }
} 