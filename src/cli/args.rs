//! 命令行参数定义
//! 
//! 使用 clap 定义所有命令和参数。

use clap::{Parser, Subcommand};
use crate::error::GtResult;

/// GT (Git Toolkit) - 下一代 Git 工作流工具
#[derive(Parser)]
#[command(name = "gt")]
#[command(about = "Git Toolkit - Next generation Git workflow tool")]
#[command(version)]
#[command(author = "GT Team")]
pub struct Cli {
    /// 启用详细输出
    #[arg(short, long, global = true)]
    pub verbose: bool,
    
    /// 预演模式，不执行实际操作
    #[arg(short = 'n', long, global = true)]
    pub dry_run: bool,
    
    /// 非交互模式
    #[arg(short = 'y', long, global = true)]
    pub yes: bool,
    
    #[command(subcommand)]
    pub command: Commands,
}

impl Cli {
    /// 执行命令
    pub async fn execute(self) -> GtResult<()> {
        use crate::cli::CommandRouter;
        
        let router = CommandRouter::new(self.verbose, self.dry_run, !self.yes);
        router.route(self.command).await
    }
}

/// 所有可用的命令
#[derive(Subcommand)]
pub enum Commands {
    /// 开始新的功能分支
    Start {
        /// 分支名称
        branch: String,
        
        /// 基础分支
        #[arg(short = 'b', long, default_value = "main")]
        base: String,
        
        /// 仅使用本地分支，不拉取远程
        #[arg(short = 'l', long)]
        local: bool,
    },
    
    /// 保存当前工作 (add + commit)
    Save {
        /// 提交信息
        #[arg(short = 'm', long)]
        message: Option<String>,
        
        /// 强制使用编辑器
        #[arg(short = 'e', long)]
        edit: bool,
        
        /// 要添加的文件 (默认为所有文件)
        files: Vec<String>,
    },
    
    /// 更新当前分支 (对应 gw update)
    Update {
        /// 强制推送
        #[arg(short = 'f', long)]
        force: bool,
    },
    
    /// 提交工作成果 (对应 gw submit)
    Ship {
        /// 不切换回主分支
        #[arg(long)]
        no_switch: bool,
        
        /// 创建 Pull Request
        #[arg(short = 'p', long)]
        pr: bool,
        
        /// 自动合并 (rebase 策略)
        #[arg(short = 'a', long)]
        auto_merge: bool,
        
        /// 自动合并 (squash 策略)
        #[arg(short = 's', long)]
        squash: bool,
        
        /// 自动合并 (merge 策略)
        #[arg(short = 'm', long)]
        merge: bool,
        
        /// 合并后删除分支
        #[arg(long)]
        delete_branch: bool,
    },
    
    /// 清理分支 (对应 gw rm)
    Clean {
        /// 分支名称或 "all"
        branch: String,
        
        /// 强制删除
        #[arg(short = 'f', long)]
        force: bool,
    },
    
    /// 显示仓库状态
    Status {
        /// 显示远程信息
        #[arg(short = 'r', long)]
        remote: bool,
        
        /// 显示最近日志
        #[arg(short = 'l', long)]
        log: bool,
    },
    
    /// 初始化 Git 仓库
    Init {
        /// 目录路径
        path: Option<String>,
    },
    
    /// 配置管理
    Config {
        #[command(subcommand)]
        action: Option<ConfigAction>,
    },
}

/// 配置操作
#[derive(Subcommand)]
pub enum ConfigAction {
    /// 显示当前配置
    Show,
    
    /// 设置配置项
    Set {
        /// 配置键
        key: String,
        /// 配置值
        value: String,
    },
    
    /// 获取配置项
    Get {
        /// 配置键
        key: String,
    },
    
    /// 从 gw 迁移配置
    Migrate,
} 