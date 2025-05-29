//! 命令模块
//! 
//! 包含所有命令的实现。

// 核心工作流命令
pub mod start;
pub mod save;
pub mod sp;
pub mod update;
pub mod ship;
// pub mod rm;  // TODO: 待实现
pub mod clean;

// Git操作增强封装 - TODO: 待实现
pub mod status;
// pub mod add;
// pub mod commit;
// pub mod push;
// pub mod pull;
// pub mod fetch;
// pub mod branch;
// pub mod checkout;
// pub mod merge;
// pub mod log;
// pub mod diff;
// pub mod reset;
// pub mod stash;
// pub mod rebase;
// pub mod undo;
// pub mod unstage;

// 仓库管理与配置
pub mod init;
pub mod config;
// pub mod remote;
// pub mod gh_create;
// pub mod ide;

// 兼容性别名 - TODO: 待实现
// pub mod push_aliases;

// 重新导出主要类型
pub use start::StartCommand;
pub use save::SaveCommand;
pub use sp::SpCommand;
pub use update::UpdateCommand;
pub use ship::{ShipCommand, MergeStrategy};
// pub use rm::RmCommand;
pub use clean::CleanCommand;
pub use status::StatusCommand;
pub use init::InitCommand;
pub use config::ConfigCommand; 