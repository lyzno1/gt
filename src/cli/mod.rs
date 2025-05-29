//! 命令行接口模块
//! 
//! 处理命令行参数解析和命令路由。

pub mod args;
pub mod router;

pub use args::Cli;
pub use router::CommandRouter; 