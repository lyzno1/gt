//! Git 提交操作
//! 
//! 提供提交信息的类型定义。

use std::fmt;

/// Git 提交信息
#[derive(Debug, Clone)]
pub struct Commit {
    /// 提交 ID
    pub id: String,
    /// 提交消息
    pub message: String,
    /// 作者
    pub author: String,
    /// 作者邮箱
    pub author_email: String,
    /// 提交时间
    pub time: i64,
    /// 父提交 ID 列表
    pub parents: Vec<String>,
}

impl fmt::Display for Commit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", &self.id[..8], self.message.lines().next().unwrap_or(""))
    }
} 