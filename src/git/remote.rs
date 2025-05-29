//! Git 远程操作
//! 
//! 提供远程仓库信息的类型定义。

/// Git 远程仓库信息
#[derive(Debug, Clone)]
pub struct Remote {
    /// 远程名称
    pub name: String,
    /// 远程 URL
    pub url: String,
    /// 推送 URL (如果与 fetch URL 不同)
    pub push_url: Option<String>,
} 