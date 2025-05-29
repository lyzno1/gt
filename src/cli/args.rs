//! 命令行参数定义
//! 
//! 使用 clap 定义所有命令和参数，支持工作流命令和Git兼容封装。

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
    // ⭐ 核心工作流命令 (Core Workflow) ⭐
    
    /// 开始新的功能分支 (对应 gw start)
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
    
    /// 保存当前工作 (对应 gw save - add + commit)
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
    
    /// 保存并推送 (对应 gw sp - save + push)
    Sp {
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
    
    /// 删除分支 (对应 gw rm)
    Rm {
        /// 分支名称或 "all"
        branch: String,
        
        /// 强制删除
        #[arg(short = 'f', long)]
        force: bool,
    },
    
    /// 清理分支 (对应 gw clean)
    Clean {
        /// 分支名称
        branch: String,
    },
    
    // 🛠️ Git操作增强封装 (Enhanced Git Wrappers) 🛠️
    
    /// 显示仓库状态 (增强版 git status)
    Status {
        /// 显示远程信息
        #[arg(short = 'r', long)]
        remote: bool,
        
        /// 显示最近日志
        #[arg(short = 'l', long)]
        log: bool,
    },
    
    /// 添加文件到暂存区 (增强版 git add)
    Add {
        /// 要添加的文件 (无参数则交互式选择)
        files: Vec<String>,
    },
    
    /// 添加所有变更 (git add -A)
    #[command(name = "add-all")]
    AddAll,
    
    /// 提交暂存区 (封装 git commit)
    Commit {
        /// 提交信息
        #[arg(short = 'm', long)]
        message: Option<String>,
        
        /// 其他 commit 参数
        #[arg(last = true)]
        args: Vec<String>,
    },
    
    /// 推送本地提交 (增强版 git push)
    Push {
        /// 其他 push 参数
        #[arg(last = true)]
        args: Vec<String>,
    },
    
    /// 拉取更新 (增强版 git pull)
    Pull {
        /// 其他 pull 参数
        #[arg(last = true)]
        args: Vec<String>,
    },
    
    /// 获取远程更新 (封装 git fetch)
    Fetch {
        /// 其他 fetch 参数
        #[arg(last = true)]
        args: Vec<String>,
    },
    
    /// 分支操作 (增强版 git branch)
    Branch {
        /// 其他 branch 参数
        #[arg(last = true)]
        args: Vec<String>,
    },
    
    /// 切换分支 (增强版 git checkout)
    Checkout {
        /// 分支名称
        branch: Option<String>,
        
        /// 其他 checkout 参数
        #[arg(last = true)]
        args: Vec<String>,
    },
    
    /// 合并分支 (增强版 git merge)
    Merge {
        /// 来源分支
        source: String,
        
        /// 其他 merge 参数
        #[arg(last = true)]
        args: Vec<String>,
    },
    
    /// 显示提交历史 (增强版 git log)
    Log {
        /// 其他 log 参数
        #[arg(last = true)]
        args: Vec<String>,
    },
    
    /// 显示变更差异 (封装 git diff)
    Diff {
        /// 其他 diff 参数
        #[arg(last = true)]
        args: Vec<String>,
    },
    
    /// 重置HEAD (增强版 git reset)
    Reset {
        /// 重置目标
        target: String,
        
        /// 其他 reset 参数
        #[arg(last = true)]
        args: Vec<String>,
    },
    
    /// 暂存工作区变更 (增强版 git stash)
    Stash {
        /// stash 子命令
        #[command(subcommand)]
        action: Option<StashAction>,
    },
    
    /// Rebase操作 (增强版 git rebase)
    Rebase {
        /// 目标分支
        target: String,
        
        /// 交互式rebase
        #[arg(short = 'i', long)]
        interactive: bool,
        
        /// 继续rebase
        #[arg(long)]
        continue_rebase: bool,
        
        /// 中止rebase
        #[arg(long)]
        abort: bool,
        
        /// 跳过当前patch
        #[arg(long)]
        skip: bool,
        
        /// 其他 rebase 参数
        #[arg(last = true)]
        args: Vec<String>,
    },
    
    /// 撤销上一次提交 (增强操作)
    Undo {
        /// 软撤销 (保留暂存)
        #[arg(long)]
        soft: bool,
        
        /// 硬撤销 (丢弃变更)
        #[arg(long)]
        hard: bool,
    },
    
    /// 将暂存区更改移回工作区
    Unstage {
        /// 交互式选择
        #[arg(short = 'i', long)]
        interactive: bool,
        
        /// 要unstage的文件
        files: Vec<String>,
    },
    
    // 🚀 仓库管理与配置 (Repository & Config) 🚀
    
    /// 初始化 Git 仓库 (封装 git init)
    Init {
        /// 目录路径
        path: Option<String>,
        
        /// 其他 init 参数
        #[arg(last = true)]
        args: Vec<String>,
    },
    
    /// 配置管理
    Config {
        #[command(subcommand)]
        action: Option<ConfigAction>,
    },
    
    /// 远程仓库管理 (封装 git remote)
    Remote {
        /// 其他 remote 参数
        #[arg(last = true)]
        args: Vec<String>,
    },
    
    /// 在 GitHub 创建仓库 (需要 gh CLI)
    #[command(name = "gh-create")]
    GhCreate {
        /// 仓库名称
        repo: Option<String>,
        
        /// 其他 gh repo create 参数
        #[arg(last = true)]
        args: Vec<String>,
    },
    
    /// 设置或显示默认编辑器
    Ide {
        /// 编辑器名称或命令
        editor: Option<String>,
    },
}

/// Stash 子命令
#[derive(Subcommand)]
pub enum StashAction {
    /// 保存当前变更
    Push {
        /// stash 消息
        #[arg(short = 'm', long)]
        message: Option<String>,
    },
    
    /// 恢复最近的stash
    Pop,
    
    /// 应用stash但不删除
    Apply {
        /// stash 索引
        index: Option<String>,
    },
    
    /// 列出所有stash
    List,
    
    /// 显示stash内容
    Show {
        /// stash 索引
        index: Option<String>,
    },
    
    /// 删除stash
    Drop {
        /// stash 索引
        index: Option<String>,
    },
    
    /// 清空所有stash
    Clear,
}

/// 配置操作
#[derive(Subcommand)]
pub enum ConfigAction {
    /// 显示当前配置
    Show,
    
    /// 列出配置
    List,
    
    /// 设置远程URL
    #[command(name = "set-url")]
    SetUrl {
        /// 远程名称或URL
        remote_or_url: String,
        /// URL (当第一个参数是远程名称时)
        url: Option<String>,
    },
    
    /// 添加远程仓库
    #[command(name = "add-remote")]
    AddRemote {
        /// 远程名称
        name: String,
        /// 远程URL
        url: String,
    },
    
    /// 设置用户信息
    User {
        /// 用户名
        name: String,
        /// 邮箱
        email: String,
        /// 全局设置
        #[arg(short = 'g', long)]
        global: bool,
    },
    
    /// 从 gw 迁移配置
    Migrate,
    
    /// 其他git config操作
    Git {
        /// git config 参数
        #[arg(last = true)]
        args: Vec<String>,
    },
} 