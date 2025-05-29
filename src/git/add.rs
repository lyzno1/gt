//! Git add 操作抽象
//! 
//! 提供暂存区添加文件的类型定义和便捷函数

use crate::error::GtResult;
use crate::git::Repository;

/// Add 操作的选项
#[derive(Debug, Clone)]
pub struct AddOptions {
    /// 是否添加所有文件（包括删除的文件）
    pub all: bool,
    /// 是否强制添加忽略的文件
    pub force: bool,
    /// 是否更新已跟踪的文件
    pub update: bool,
    /// 是否显示详细信息
    pub verbose: bool,
}

impl Default for AddOptions {
    fn default() -> Self {
        Self {
            all: false,
            force: false,
            update: false,
            verbose: false,
        }
    }
}

/// Add 操作的结果
#[derive(Debug)]
pub struct AddResult {
    /// 已添加的文件列表
    pub added_files: Vec<String>,
    /// 使用的选项
    pub options: AddOptions,
}

impl AddResult {
    /// 获取添加的文件数量
    pub fn file_count(&self) -> usize {
        self.added_files.len()
    }
    
    /// 是否添加了文件
    pub fn has_files(&self) -> bool {
        !self.added_files.is_empty()
    }
}

/// 便捷函数：添加所有文件
pub fn add_all(repo: &Repository, verbose: bool) -> GtResult<AddResult> {
    repo.add_all()?;
    
    if verbose {
        let status = repo.check_status()?;
        println!("✅ 已添加所有变更到暂存区");
        println!("   修改文件: {}", status.modified_files);
        println!("   新增文件: {}", status.added_files);
        println!("   删除文件: {}", status.deleted_files);
    }
    
    Ok(AddResult {
        added_files: vec!["所有变更文件".to_string()],
        options: AddOptions { all: true, verbose, ..Default::default() },
    })
}

/// 便捷函数：添加指定文件
pub fn add_files<I, S>(repo: &Repository, files: I, verbose: bool) -> GtResult<AddResult>
where
    I: IntoIterator<Item = S>,
    S: Into<String>,
{
    let file_list: Vec<String> = files.into_iter().map(|s| s.into()).collect();
    let paths: Vec<&str> = file_list.iter().map(|s| s.as_str()).collect();
    
    repo.add_files(&paths)?;
    
    if verbose {
        println!("✅ 已添加 {} 个文件到暂存区:", file_list.len());
        for file in &file_list {
            println!("   + {}", file);
        }
    }
    
    Ok(AddResult {
        added_files: file_list,
        options: AddOptions { verbose, ..Default::default() },
    })
} 