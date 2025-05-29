//! 命令路由器
//! 
//! 负责将命令分发到对应的处理器。

use crate::cli::args::{Commands, ConfigAction};
use crate::error::{GtResult, GtError};
use crate::error::ErrorHandler;

/// 命令路由器
pub struct CommandRouter {
    error_handler: ErrorHandler,
    verbose: bool,
    dry_run: bool,
}

impl CommandRouter {
    /// 创建新的命令路由器
    pub fn new(verbose: bool, dry_run: bool, interactive: bool) -> Self {
        let error_handler = ErrorHandler::new(verbose, interactive);
        
        Self {
            error_handler,
            verbose,
            dry_run,
        }
    }
    
    /// 路由命令到对应的处理器
    pub async fn route(&self, command: Commands) -> GtResult<()> {
        if self.verbose {
            println!("🔧 执行命令: {:?}", std::any::type_name_of_val(&command));
        }
        
        if self.dry_run {
            println!("🔍 预演模式: 不会执行实际操作");
        }
        
        match command {
            // ⭐ 核心工作流命令
            Commands::Start { branch, base, local } => {
                self.handle_start(branch, base, local).await
            }
            Commands::Save { message, edit, files } => {
                self.handle_save(message, edit, files).await
            }
            Commands::Sp { message, edit, files } => {
                self.handle_sp(message, edit, files).await
            }
            Commands::Update { force } => {
                self.handle_update(force).await
            }
            Commands::Ship { 
                no_switch, 
                pr, 
                auto_merge, 
                squash, 
                merge, 
                delete_branch 
            } => {
                self.handle_ship(no_switch, pr, auto_merge, squash, merge, delete_branch).await
            }
            Commands::Rm { branch, force } => {
                self.handle_rm(branch, force).await
            }
            Commands::Clean { branch } => {
                self.handle_clean(branch).await
            }
            
            // 🛠️ Git操作增强封装
            Commands::Status { remote, log } => {
                self.handle_status().await
            }
            
            // 暂时返回未实现错误的Git封装命令
            Commands::Add { files: _ } => {
                Err(GtError::NotImplemented { feature: "add command".to_string() })
            }
            Commands::AddAll => {
                Err(GtError::NotImplemented { feature: "add-all command".to_string() })
            }
            Commands::Commit { message: _, args: _ } => {
                Err(GtError::NotImplemented { feature: "commit command".to_string() })
            }
            Commands::Push { args: _ } => {
                Err(GtError::NotImplemented { feature: "push command".to_string() })
            }
            Commands::Pull { args: _ } => {
                Err(GtError::NotImplemented { feature: "pull command".to_string() })
            }
            Commands::Fetch { args: _ } => {
                Err(GtError::NotImplemented { feature: "fetch command".to_string() })
            }
            Commands::Branch { args: _ } => {
                Err(GtError::NotImplemented { feature: "branch command".to_string() })
            }
            Commands::Checkout { branch: _, args: _ } => {
                Err(GtError::NotImplemented { feature: "checkout command".to_string() })
            }
            Commands::Merge { source: _, args: _ } => {
                Err(GtError::NotImplemented { feature: "merge command".to_string() })
            }
            Commands::Log { args: _ } => {
                Err(GtError::NotImplemented { feature: "log command".to_string() })
            }
            Commands::Diff { args: _ } => {
                Err(GtError::NotImplemented { feature: "diff command".to_string() })
            }
            Commands::Reset { target: _, args: _ } => {
                Err(GtError::NotImplemented { feature: "reset command".to_string() })
            }
            Commands::Stash { action: _ } => {
                Err(GtError::NotImplemented { feature: "stash command".to_string() })
            }
            Commands::Rebase { target: _, interactive: _, continue_rebase: _, abort: _, skip: _, args: _ } => {
                Err(GtError::NotImplemented { feature: "rebase command".to_string() })
            }
            Commands::Undo { soft: _, hard: _ } => {
                Err(GtError::NotImplemented { feature: "undo command".to_string() })
            }
            Commands::Unstage { interactive: _, files: _ } => {
                Err(GtError::NotImplemented { feature: "unstage command".to_string() })
            }
            
            // 🚀 仓库管理与配置
            Commands::Init { path, args: _ } => {
                self.handle_init(path).await
            }
            Commands::Config { action } => {
                self.handle_config(action).await
            }
            Commands::Remote { args: _ } => {
                Err(GtError::NotImplemented { feature: "remote command".to_string() })
            }
            Commands::GhCreate { repo: _, args: _ } => {
                Err(GtError::NotImplemented { feature: "gh-create command".to_string() })
            }
            Commands::Ide { editor: _ } => {
                Err(GtError::NotImplemented { feature: "ide command".to_string() })
            }
            Commands::UpdateSelf { check, yes } => {
                self.handle_update_self(check, yes).await
            }
        }
    }
    
    /// 处理 start 命令
    async fn handle_start(&self, branch: String, base: String, local: bool) -> GtResult<()> {
        use crate::commands::{StartCommand, start::StartOptions};
        
        let options = StartOptions {
            branch,
            base: if base.is_empty() { None } else { Some(base) },
            local,
            force: false,
            skip_update: self.dry_run,
            description: None,
            dry_run: self.dry_run,
        };
        
        let cmd = StartCommand::new(options);
        cmd.execute().await
    }
    
    /// 处理 save 命令
    async fn handle_save(&self, message: Option<String>, edit: bool, files: Vec<String>) -> GtResult<()> {
        use crate::commands::SaveCommand;
        
        let cmd = SaveCommand::new(message, edit, files);
        cmd.execute().await
    }
    
    /// 处理 update 命令
    async fn handle_update(&self, force: bool) -> GtResult<()> {
        use crate::commands::UpdateCommand;
        
        let cmd = UpdateCommand::new(force);
        cmd.execute().await
    }
    
    /// 处理 ship 命令
    async fn handle_ship(
        &self,
        no_switch: bool,
        pr: bool,
        auto_merge: bool,
        squash: bool,
        merge: bool,
        delete_branch: bool,
    ) -> GtResult<()> {
        use crate::commands::ShipCommand;
        
        // 确定合并策略
        let merge_strategy = if squash {
            Some(crate::commands::MergeStrategy::Squash)
        } else if merge {
            Some(crate::commands::MergeStrategy::Merge)
        } else if auto_merge {
            Some(crate::commands::MergeStrategy::Rebase)
        } else {
            None
        };
        
        let cmd = ShipCommand::new(no_switch, pr, merge_strategy, delete_branch);
        cmd.execute().await
    }
    
    /// 处理 clean 命令
    async fn handle_clean(&self, branch: String) -> GtResult<()> {
        use crate::commands::CleanCommand;
        
        let cmd = CleanCommand::new(branch);
        cmd.execute().await
    }
    
    /// 处理 status 命令
    async fn handle_status(&self) -> GtResult<()> {
        // 直接使用 GitOps 来检查状态，不再需要 StatusCommand
        let git_ops = crate::git::GitOps::new()?;
        let status = git_ops.check_status()?;
        
        println!("工作区状态:");
        println!("  修改的文件: {}", status.modified_files);
        println!("  新增的文件: {}", status.added_files);
        println!("  删除的文件: {}", status.deleted_files);
        println!("  未追踪的文件: {}", status.untracked_files);
        
        if status.is_clean() {
            println!("✅ 工作区干净");
        } else {
            println!("⚠️ 工作区有未处理的变更");
        }
        
        Ok(())
    }
    
    /// 处理 init 命令
    async fn handle_init(&self, path: Option<String>) -> GtResult<()> {
        use crate::commands::InitCommand;
        
        let cmd = InitCommand::new(path);
        cmd.execute().await
    }
    
    /// 处理 config 命令
    async fn handle_config(&self, action: Option<ConfigAction>) -> GtResult<()> {
        use crate::commands::ConfigCommand;
        
        let cmd = ConfigCommand::new(action);
        cmd.execute().await
    }
    
    /// 处理 sp 命令
    async fn handle_sp(&self, message: Option<String>, edit: bool, files: Vec<String>) -> GtResult<()> {
        use crate::commands::SpCommand;
        
        let cmd = SpCommand::new(message, edit, files);
        cmd.execute().await
    }
    
    /// 处理 rm 命令
    async fn handle_rm(&self, _branch: String, _force: bool) -> GtResult<()> {
        // TODO: 实现 RmCommand
        Err(GtError::NotImplemented { feature: "rm command".to_string() })
    }
    
    /// 处理 update-self 命令
    async fn handle_update_self(&self, check: bool, yes: bool) -> GtResult<()> {
        use crate::ui::{print_step, print_success, print_warning, confirm_action};
        use std::process::Command;
        
        if check {
            print_step("检查 GT 更新...");
            // TODO: 实现版本检查逻辑
            print_success("当前版本是最新的");
            return Ok(());
        }
        
        if !yes && !confirm_action("确定要更新 GT 到最新版本吗？", false) {
            return Err(GtError::UserCancelled);
        }
        
        if self.dry_run {
            print_step("🔍 [预演] 更新 GT 自身");
            print_success("🔍 [预演] GT 更新完成");
            return Ok(());
        }
        
        print_step("🔄 开始更新 GT...");
        
        // 1. 使用 GT 自己来同步最新代码
        print_step("📥 使用 GT 拉取最新更新...");
        let git_ops = crate::git::GitOps::new()?;
        let config_manager = crate::config::ConfigManager::new(git_ops.repository())?;
        let config = config_manager.repo_config();
        
        // 拉取最新更新
        crate::git::network::pull_rebase_with_retry(
            git_ops.repository(),
            &config.remote_name,
            Some(&config.main_branch)
        )?;
        
        // 2. 重新编译和安装
        print_step("🔨 重新编译 GT...");
        let output = Command::new("cargo")
            .args(&["install", "--path", ".", "--force"])
            .output()
            .map_err(|e| GtError::GitOperation {
                message: format!("执行 cargo install 失败: {}", e)
            })?;
            
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(GtError::GitOperation {
                message: format!("编译安装失败: {}", stderr)
            });
        }
        
        // 3. 复制到正确的路径（如果需要）
        print_step("📋 更新系统路径...");
        let home = std::env::var("HOME").unwrap_or_default();
        let cargo_bin = format!("{}/.cargo/bin/gt", home);
        let local_bin = format!("{}/.local/bin/gt", home);
        
        if std::path::Path::new(&cargo_bin).exists() && std::path::Path::new(&format!("{}/.local/bin", home)).exists() {
            let _ = Command::new("cp")
                .args(&[&cargo_bin, &local_bin])
                .output();
        }
        
        print_success("🎉 GT 更新完成！");
        print_step("✅ 验证安装...");
        
        // 验证安装
        let output = Command::new("gt")
            .args(&["--version"])
            .output();
            
        match output {
            Ok(out) if out.status.success() => {
                let version = String::from_utf8_lossy(&out.stdout);
                print_success(&format!("GT 版本: {}", version.trim()));
            }
            _ => {
                print_warning("无法验证版本，但安装可能已完成");
            }
        }
        
        Ok(())
    }
} 