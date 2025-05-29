//! 命令行参数定义
//! 
//! 使用 clap 定义所有命令和参数，支持工作流命令和Git兼容封装。

use clap::{Parser, Subcommand};
use crate::error::GtResult;

/// GT (Git Toolkit) - 下一代 Git 工作流工具
/// 
/// GT 是一个现代化的 Git 工作流工具，专为开发者日常工作流程设计。
/// 它提供了简洁直观的命令来处理常见的 Git 操作，让版本控制变得更加高效。
/// 
/// 🚀 快速开始:
///   gt start feature/login    # 开始新功能开发
///   gt save "实现登录功能"     # 保存进度  
///   gt update                 # 同步最新代码
///   gt ship --pr              # 提交工作成果并创建PR
/// 
/// 💡 核心理念:
///   - 流程驱动: 命令对应开发意图，而非Git技术细节
///   - 智能默认: 减少决策负担，提供最佳实践
///   - 安全第一: 防止误操作，提供清晰反馈
///   - 高性能: 基于Rust，启动快速，操作高效
#[derive(Parser)]
#[command(name = "gt")]
#[command(about = "🚀 Git Toolkit - 现代化 Git 工作流工具")]
#[command(long_about = r#"
GT (Git Toolkit) - 下一代 Git 工作流工具

GT 是一个现代化的 Git 工作流工具，专为开发者日常工作流程设计。
它提供了简洁直观的命令来处理常见的 Git 操作，让版本控制变得更加高效。

🚀 快速开始:
  gt start feature/login    # 开始新功能开发
  gt save "实现登录功能"     # 保存进度  
  gt update                 # 同步最新代码
  gt ship --pr              # 提交工作成果并创建PR

💡 核心理念:
  - 流程驱动: 命令对应开发意图，而非Git技术细节
  - 智能默认: 减少决策负担，提供最佳实践
  - 安全第一: 防止误操作，提供清晰反馈
  - 高性能: 基于Rust，启动快速，操作高效

📖 更多信息: https://github.com/lyzno1/gt
"#)]
#[command(version)]
#[command(author = "GT Team <gt@lyzno1.dev>")]
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
/// 
/// GT 的命令按功能分为四大类：
/// 1. 🌟 核心工作流 - 日常开发的主要命令
/// 2. 🛠️ Git增强 - Git命令的现代化封装
/// 3. 🚀 仓库管理 - 项目初始化和配置
/// 4. 🔧 高级功能 - 专业开发者工具
#[derive(Subcommand)]
pub enum Commands {
    // 🌟 核心工作流命令 (Core Workflow Commands)
    // 这些命令覆盖了日常开发的80%场景
    
    /// 🌱 开始新的功能分支
    /// 
    /// 这是所有功能开发的起点。GT会：
    /// • 从最新的主分支创建新分支
    /// • 自动同步远程更新  
    /// • 设置合适的上游追踪
    /// 
    /// 示例：
    ///   gt start feature/user-auth      # 标准功能分支
    ///   gt start hotfix/login-bug -b develop  # 从develop分支创建hotfix
    ///   gt start experiment/new-ui -l  # 仅本地分支
    #[command(visible_alias = "new")]
    Start {
        /// 分支名称 (建议格式: feature/name, hotfix/name, experiment/name)
        #[arg(help = "分支名称，建议使用 feature/name 格式")]
        branch: String,
        
        /// 基础分支 (默认: main)
        #[arg(short = 'b', long, default_value = "main")]
        #[arg(help = "基础分支，新分支将从此分支创建")]
        base: String,
        
        /// 本地模式：不拉取远程更新，不推送到远程
        #[arg(short = 'l', long)]
        #[arg(help = "仅在本地创建分支，不同步远程")]
        local: bool,
    },
    
    /// 💾 保存当前工作进度
    /// 
    /// 相当于 git add + git commit 的智能组合。GT会：
    /// • 智能选择要提交的文件
    /// • 提供交互式提交信息编辑
    /// • 验证提交内容的合理性
    /// 
    /// 示例：
    ///   gt save                         # 交互式提交所有变更
    ///   gt save -m "修复登录bug"        # 快速提交
    ///   gt save src/auth.rs -e          # 提交特定文件并编辑消息
    #[command(visible_alias = "s")]
    Save {
        /// 提交信息 (如果不提供将进入交互模式)
        #[arg(short = 'm', long)]
        #[arg(help = "提交信息，留空将进入交互模式")]
        message: Option<String>,
        
        /// 强制使用编辑器编辑提交信息
        #[arg(short = 'e', long)]
        #[arg(help = "使用编辑器编辑提交信息")]
        edit: bool,
        
        /// 要添加的文件 (默认为所有变更)
        #[arg(help = "指定要提交的文件，留空则提交所有变更")]
        files: Vec<String>,
    },
    
    /// 🚀 保存并推送 (save + push)
    /// 
    /// 完整的进度保存流程，包括：
    /// • 保存当前工作 (如 save 命令)
    /// • 推送到远程仓库
    /// • 验证推送结果
    /// 
    /// 示例：
    ///   gt sp -m "完成用户认证模块"     # 保存并推送
    #[command(name = "sp")]
    #[command(about = "保存并推送到远程 (save + push)")]
    Sp {
        /// 提交信息
        #[arg(short = 'm', long)]
        message: Option<String>,
        
        /// 强制使用编辑器
        #[arg(short = 'e', long)]
        edit: bool,
        
        /// 要添加的文件
        files: Vec<String>,
    },
    
    /// 🔄 同步分支到最新状态
    /// 
    /// 智能同步当前分支，自动处理：
    /// • 暂存未提交的变更
    /// • 拉取主分支最新更新
    /// • 将当前分支rebase到最新主分支
    /// • 恢复之前暂存的变更
    /// 
    /// 示例：
    ///   gt update                       # 标准同步
    ///   gt update -f                    # 强制同步(忽略未提交变更)
    #[command(visible_alias = "sync")]
    Update {
        /// 强制模式：忽略未提交的变更进行同步
        #[arg(short = 'f', long)]
        #[arg(help = "强制同步，忽略未提交的变更")]
        force: bool,
    },
    
    /// 🚢 提交工作成果 (ship to production)
    /// 
    /// 完整的功能交付流程，包括：
    /// • 推送分支到远程
    /// • 创建Pull Request (可选)
    /// • 自动合并 (可选)
    /// • 切换回主分支并清理
    /// 
    /// 示例：
    ///   gt ship                         # 简单推送
    ///   gt ship --pr                    # 创建PR
    ///   gt ship -a                      # 创建PR并自动合并(rebase)
    ///   gt ship -s --delete-branch      # 使用squash合并并删除分支
    #[command(visible_alias = "submit")]
    Ship {
        /// 完成后不切换回主分支
        #[arg(long)]
        #[arg(help = "完成后保持在当前分支")]
        no_switch: bool,
        
        /// 创建 Pull Request
        #[arg(short = 'p', long)]
        #[arg(help = "在GitHub上创建Pull Request")]
        pr: bool,
        
        /// 自动合并 (使用rebase策略，推荐)
        #[arg(short = 'a', long)]
        #[arg(help = "创建PR并自动合并，使用rebase策略")]
        auto_merge: bool,
        
        /// 自动合并 (使用squash策略)
        #[arg(short = 's', long)]
        #[arg(help = "创建PR并自动合并，使用squash策略")]
        squash: bool,
        
        /// 自动合并 (使用merge策略)
        #[arg(short = 'm', long)]
        #[arg(help = "创建PR并自动合并，使用merge策略")]
        merge: bool,
        
        /// 合并后删除源分支
        #[arg(long)]
        #[arg(help = "合并完成后删除功能分支")]
        delete_branch: bool,
    },
    
    /// 🗑️ 删除分支
    /// 
    /// 安全地删除本地或远程分支：
    /// • 检查分支是否已合并
    /// • 提供强制删除选项
    /// • 支持批量删除
    /// 
    /// 示例：
    ///   gt rm feature/old-feature       # 删除已合并的分支
    ///   gt rm feature/broken -f         # 强制删除分支
    ///   gt rm all                       # 删除所有已合并的分支
    Rm {
        /// 分支名称，或 "all" 删除所有已合并分支
        #[arg(help = "分支名称，或使用 'all' 删除所有已合并分支")]
        branch: String,
        
        /// 强制删除 (即使未合并)
        #[arg(short = 'f', long)]
        #[arg(help = "强制删除，即使分支未合并")]
        force: bool,
    },
    
    /// 🧹 清理和重置分支
    /// 
    /// 重置分支到干净状态：
    /// • 撤销未提交的变更
    /// • 清理未追踪的文件  
    /// • 重置到指定状态
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
    
    /// 🔄 更新 GT 自身
    /// 
    /// 自动更新 GT 到最新版本：
    /// • 使用 GT 自己拉取最新源码
    /// • 重新编译和安装
    /// • 验证更新结果
    /// 
    /// 示例：
    ///   gt update-self                  # 更新到最新版本
    ///   gt update-self --check          # 仅检查是否有更新
    #[command(name = "update-self")]
    UpdateSelf {
        /// 仅检查更新，不执行安装
        #[arg(long)]
        #[arg(help = "仅检查是否有可用更新")]
        check: bool,
        
        /// 跳过确认提示
        #[arg(short = 'y', long)]
        #[arg(help = "跳过确认提示，直接更新")]
        yes: bool,
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