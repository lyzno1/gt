//! Start 命令实现
//! 
//! 对应 gw start，用于开始新的功能分支
//! 增强功能：智能分支命名、自动配置、状态检查

use crate::error::{GtResult, GtError};
use crate::git::{Repository, GitOps};
use crate::config::{ConfigManager, RepoConfig};
use crate::ui::{print_step, print_success, print_warning, confirm_action};
use crate::git::network::pull_rebase_with_retry;

/// Start 命令选项
#[derive(Debug, Clone)]
pub struct StartOptions {
    /// 分支名称
    pub branch: String,
    /// 基础分支（默认为主分支）
    pub base: Option<String>,
    /// 是否只在本地创建（不推送）
    pub local: bool,
    /// 是否强制创建（覆盖已存在的分支）
    pub force: bool,
    /// 是否跳过更新基础分支
    pub skip_update: bool,
    /// 自定义描述
    pub description: Option<String>,
    /// 预演模式（不执行实际操作）
    pub dry_run: bool,
}

impl Default for StartOptions {
    fn default() -> Self {
        Self {
            branch: String::new(),
            base: None,
            local: false,
            force: false,
            skip_update: false,
            description: None,
            dry_run: false,
        }
    }
}

/// Start 命令
pub struct StartCommand {
    options: StartOptions,
}

impl StartCommand {
    /// 创建新的 Start 命令
    pub fn new(options: StartOptions) -> Self {
        Self { options }
    }
    
    /// 便捷构造函数
    pub fn with_branch(branch: String) -> Self {
        let options = StartOptions {
            branch,
            ..Default::default()
        };
        Self::new(options)
    }
    
    /// 执行命令
    pub async fn execute(mut self) -> GtResult<()> {
        print_step(&format!("开始创建功能分支 '{}'", self.options.branch));
        
        let git_ops = GitOps::new()?;
        let config_manager = ConfigManager::new(git_ops.repository())?;
        let config = config_manager.repo_config();
        
        // 1. 验证输入
        self.validate_input()?;
        
        // 2. 检查工作区状态
        self.check_working_directory(&git_ops)?;
        
        // 3. 确定基础分支并更新
        let base_branch = self.determine_base_branch(config)?;
        if !self.options.skip_update {
            self.update_base_branch(&git_ops, config, &base_branch).await?;
        }
        
        // 4. 检查分支是否已存在
        self.handle_existing_branch(&git_ops)?;
        
        // 5. 创建并切换到新分支
        self.create_and_checkout_branch(&git_ops, &base_branch)?;
        
        // 6. 显示成功信息和后续建议
        self.show_success_info();
        
        Ok(())
    }
    
    /// 验证输入参数
    fn validate_input(&mut self) -> GtResult<()> {
        // 验证分支名称
        if self.options.branch.is_empty() {
            return Err(GtError::InvalidInput {
                input: "分支名称不能为空".to_string()
            });
        }
        
        // 规范化分支名称
        self.options.branch = self.normalize_branch_name(&self.options.branch);
        
        // 验证分支名称格式
        if !self.is_valid_branch_name(&self.options.branch) {
            return Err(GtError::InvalidBranchName {
                name: self.options.branch.clone()
            });
        }
        
        Ok(())
    }
    
    /// 规范化分支名称
    fn normalize_branch_name(&self, name: &str) -> String {
        name.trim()
            .replace(' ', "-")
            .replace("_", "-")
            .to_lowercase()
    }
    
    /// 验证分支名称是否有效
    fn is_valid_branch_name(&self, name: &str) -> bool {
        !name.is_empty() 
            && !name.starts_with('-')
            && !name.ends_with('-')
            && !name.contains("..")
            && !name.contains("//")
            && name.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '/' || c == '_')
    }
    
    /// 检查工作区状态
    fn check_working_directory(&self, git_ops: &GitOps) -> GtResult<()> {
        if !git_ops.is_clean()? {
            let has_uncommitted = git_ops.has_uncommitted_changes()?;
            let has_untracked = git_ops.has_untracked_files()?;
            
            print_warning("工作区不干净，存在未处理的变更");
            
            if has_uncommitted {
                print_warning("发现未提交的变更");
            }
            
            if has_untracked {
                print_warning("发现未追踪的文件");
            }
            
            if !confirm_action("是否要在当前状态下继续创建分支？", false) {
                return Err(GtError::UserCancelled);
            }
        }
        
        Ok(())
    }
    
    /// 确定基础分支
    fn determine_base_branch(&self, config: &RepoConfig) -> GtResult<String> {
        if let Some(ref base) = self.options.base {
            Ok(base.clone())
        } else {
            Ok(config.main_branch.clone())
        }
    }
    
    /// 更新基础分支
    async fn update_base_branch(&self, git_ops: &GitOps, config: &RepoConfig, base_branch: &str) -> GtResult<()> {
        if self.options.dry_run {
            print_step(&format!("🔍 [预演] 更新基础分支 '{}' 到最新状态", base_branch));
            print_success(&format!("🔍 [预演] 基础分支 '{}' 已更新到最新状态", base_branch));
            return Ok(());
        }
        
        let current_branch = git_ops.current_branch()?;
        
        // 如果当前不在基础分支上，需要切换
        if current_branch != base_branch {
            print_step(&format!("切换到基础分支 '{}'", base_branch));
            git_ops.checkout_branch(base_branch)?;
        }
        
        // 拉取最新更新
        print_step(&format!("更新基础分支 '{}' 到最新状态", base_branch));
        pull_rebase_with_retry(
            git_ops.repository(), 
            &config.remote_name, 
            Some(base_branch)
        )?;
        
        print_success(&format!("基础分支 '{}' 已更新到最新状态", base_branch));
        Ok(())
    }
    
    /// 处理已存在的分支
    fn handle_existing_branch(&self, git_ops: &GitOps) -> GtResult<()> {
        let branches = git_ops.list_branches()?;
        let branch_exists = branches.iter().any(|b| b.name == self.options.branch);
        
        if branch_exists {
            if self.options.force {
                print_warning(&format!("分支 '{}' 已存在，将被强制重新创建", self.options.branch));
                git_ops.delete_branch(&self.options.branch, true)?;
            } else {
                return Err(GtError::BranchAlreadyExists {
                    branch: self.options.branch.clone()
                });
            }
        }
        
        Ok(())
    }
    
    /// 创建并切换到新分支
    fn create_and_checkout_branch(&self, git_ops: &GitOps, base_branch: &str) -> GtResult<()> {
        print_step(&format!(
            "基于 '{}' 创建新分支 '{}'", 
            base_branch, 
            self.options.branch
        ));
        
        git_ops.create_and_checkout_branch(&self.options.branch, Some(base_branch))?;
        
        print_success(&format!("已创建并切换到分支 '{}'", self.options.branch));
        Ok(())
    }
    
    /// 显示成功信息和建议
    fn show_success_info(&self) {
        print_success(&format!("🎉 功能分支 '{}' 创建成功！", self.options.branch));
        
        if let Some(ref desc) = self.options.description {
            println!("  📝 描述: {}", desc);
        }
        
        println!("\n📋 后续操作建议:");
        println!("  gt save \"初始提交\"     # 保存第一次提交");
        println!("  gt save                # 交互式提交");
        println!("  gt sync                # 同步最新更新");
        println!("  gt ship --pr           # 创建 PR 并提交");
        println!("  gt ship -a             # 创建 PR 并自动合并");
    }
}

/// 便捷函数：快速开始新分支
pub async fn start_branch(branch_name: String, base: Option<String>) -> GtResult<()> {
    let options = StartOptions {
        branch: branch_name,
        base,
        ..Default::default()
    };
    
    let cmd = StartCommand::new(options);
    cmd.execute().await
}

/// 便捷函数：本地分支开发
pub async fn start_local_branch(branch_name: String) -> GtResult<()> {
    let options = StartOptions {
        branch: branch_name,
        local: true,
        ..Default::default()
    };
    
    let cmd = StartCommand::new(options);
    cmd.execute().await
} 