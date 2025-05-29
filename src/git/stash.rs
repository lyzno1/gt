//! Git stash 操作
//! 
//! 提供 stash 信息的类型定义。

/// Git stash 信息
#[derive(Debug, Clone)]
pub struct Stash {
    /// stash 索引
    pub index: usize,
    /// stash 消息
    pub message: String,
    /// 创建时间
    pub time: i64,
    /// stash ID
    pub id: String,
} 