use crate::error::{GtResult, GtError};

/// 分支信息
#[derive(Debug, Clone)]
pub struct Branch {
    pub name: String,
    pub is_current: bool,
    pub is_remote: bool,
    pub upstream: Option<String>,
    pub ahead: usize,
    pub behind: usize,
    pub last_commit: Option<String>,
}

impl Branch {
    /// 检查分支名是否有效
    pub fn is_valid_name(name: &str) -> bool {
        !name.is_empty() 
        && !name.contains("..")
        && !name.starts_with('/')
        && !name.ends_with('/')
        && !name.contains(' ')
        && !name.contains('~')
        && !name.contains('^')
        && !name.contains(':')
        && !name.contains('?')
        && !name.contains('*')
        && !name.contains('[')
    }
} 