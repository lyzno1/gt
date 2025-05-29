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
            Commands::Start { branch, base, local } => {
                self.handle_start(branch, base, local).await
            }
            Commands::Save { message, edit, files } => {
                self.handle_save(message, edit, files).await
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
            Commands::Clean { branch, force } => {
                self.handle_clean(branch, force).await
            }
            Commands::Status { remote, log } => {
                self.handle_status(remote, log).await
            }
            Commands::Init { path } => {
                self.handle_init(path).await
            }
            Commands::Config { action } => {
                self.handle_config(action).await
            }
        }
    }
    
    /// 处理 start 命令
    async fn handle_start(&self, branch: String, base: String, local: bool) -> GtResult<()> {
        use crate::commands::StartCommand;
        
        let cmd = StartCommand::new(branch, base, local);
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
    async fn handle_clean(&self, branch: String, force: bool) -> GtResult<()> {
        use crate::commands::CleanCommand;
        
        let cmd = CleanCommand::new(branch, force);
        cmd.execute().await
    }
    
    /// 处理 status 命令
    async fn handle_status(&self, remote: bool, log: bool) -> GtResult<()> {
        use crate::commands::StatusCommand;
        
        let cmd = StatusCommand::new(remote, log);
        cmd.execute().await
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
} 