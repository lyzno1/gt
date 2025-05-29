//! Git reset 操作抽象
//! 
//! 提供重置操作的类型定义和便捷函数

use crate::error::GtResult;
use crate::git::Repository;

/// Reset 操作的类型
#[derive(Debug, Clone)]
pub enum ResetType {
    /// 软重置（保留工作区和暂存区）
    Soft,
    /// 混合重置（保留工作区，清空暂存区）
    Mixed,
    /// 硬重置（清除所有变更）
    Hard,
}

/// Reset 操作的结果
#[derive(Debug)]
pub struct ResetResult {
    /// 重置的目标
    pub target: String,
    /// 重置类型
    pub reset_type: ResetType,
    /// 是否显示详细信息
    pub verbose: bool,
}

/// 便捷函数：软重置到指定提交
pub fn reset_soft(repo: &Repository, target: &str, verbose: bool) -> GtResult<ResetResult> {
    if verbose {
        println!("🔄 软重置到 {}...", target);
    }
    
    repo.reset_soft(target)?;
    
    if verbose {
        println!("✅ 软重置完成（工作区和暂存区已保留）");
    }
    
    Ok(ResetResult {
        target: target.to_string(),
        reset_type: ResetType::Soft,
        verbose,
    })
}

/// 便捷函数：混合重置到指定提交
pub fn reset_mixed(repo: &Repository, target: &str, verbose: bool) -> GtResult<ResetResult> {
    if verbose {
        println!("🔄 混合重置到 {}...", target);
    }
    
    repo.reset_mixed(target)?;
    
    if verbose {
        println!("✅ 混合重置完成（工作区已保留，暂存区已清空）");
    }
    
    Ok(ResetResult {
        target: target.to_string(),
        reset_type: ResetType::Mixed,
        verbose,
    })
}

/// 便捷函数：硬重置到指定提交
pub fn reset_hard(repo: &Repository, target: &str, verbose: bool) -> GtResult<ResetResult> {
    if verbose {
        println!("⚠️  硬重置到 {}（将丢失所有未提交的变更）...", target);
    }
    
    repo.reset_hard(target)?;
    
    if verbose {
        println!("✅ 硬重置完成（所有未提交的变更已丢失）");
    }
    
    Ok(ResetResult {
        target: target.to_string(),
        reset_type: ResetType::Hard,
        verbose,
    })
} 