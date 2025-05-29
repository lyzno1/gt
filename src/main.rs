//! GT (Git Toolkit) - 下一代 Git 工作流工具
//!
//! 这是一个用 Rust 重写的 Git 工作流工具，旨在提供比传统 shell 脚本
//! 更快、更可靠、更友好的体验。

use clap::Parser;
use gt::cli::Cli;
use gt::error::{GtError, GtResult};
use std::process;
use tracing::{error, info};

fn setup_logging() -> GtResult<()> {
    use tracing_subscriber::{filter::EnvFilter, fmt, prelude::*};

    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("gt=info"));

    tracing_subscriber::registry()
        .with(fmt::layer().with_target(false).with_thread_ids(false))
        .with(filter)
        .init();

    Ok(())
}

fn print_version_info() {
    println!("GT (Git Toolkit) {}", env!("CARGO_PKG_VERSION"));
    println!("Built with Rust {}", env!("CARGO_PKG_RUST_VERSION"));
}

#[tokio::main]
async fn main() {
    // 设置日志
    if let Err(e) = setup_logging() {
        eprintln!("Failed to setup logging: {}", e);
        process::exit(1);
    }

    // 解析命令行参数
    let cli = Cli::parse();
    
    info!("GT starting with args: {:?}", std::env::args().collect::<Vec<_>>());

    // 执行命令
    match cli.execute().await {
        Ok(()) => {
            info!("Command completed successfully");
        }
        Err(e) => {
            error!("Command failed: {}", e);
            handle_error(&e);
            process::exit(1);
        }
    }
}

fn handle_error(error: &GtError) {
    use colored::*;

    match error {
        GtError::NotImplemented { feature } => {
            eprintln!("{} {}", "Not Implemented:".red().bold(), feature);
            eprintln!("{}", "This feature is still under development.".yellow());
            eprintln!("{}", "Check https://github.com/lyzno1/gt for updates.".blue());
        }
        GtError::NotInGitRepo => {
            eprintln!("{}", "Error: Not in a Git repository".red().bold());
            eprintln!("{}", "Please run this command from within a Git repository.".yellow());
        }
        GtError::BranchNotFound { branch } => {
            eprintln!("{} {}", "Error: Branch not found:".red().bold(), branch);
        }
        GtError::UncommittedChanges => {
            eprintln!("{}", "Error: You have uncommitted changes".red().bold());
            eprintln!("{}", "Please commit or stash your changes first.".yellow());
        }
        _ => {
            eprintln!("{} {}", "Error:".red().bold(), error);
        }
    }
}
