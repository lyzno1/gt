//! GitHub CLI 核心抽象
//! 
//! 提供 GitHub CLI (gh) 命令的 Rust 抽象接口

use crate::error::{GtResult, GtError};
use crate::ui::{print_step, print_warning, print_error, print_success};
use std::process::Command;
use std::str;

/// GitHub 认证状态
#[derive(Debug, Clone, PartialEq)]
pub enum GithubAuth {
    /// 已认证，包含用户名
    Authenticated(String),
    /// 未认证
    NotAuthenticated,
    /// 认证状态未知
    Unknown,
}

/// GitHub CLI 抽象
#[derive(Clone)]
pub struct GithubCli {
    verbose: bool,
}

impl GithubCli {
    /// 创建新的 GitHub CLI 实例
    pub fn new(verbose: bool) -> Self {
        Self { verbose }
    }
    
    /// 获取 verbose 状态
    pub fn is_verbose(&self) -> bool {
        self.verbose
    }
    
    /// 检查 gh 命令是否可用
    pub fn is_available(&self) -> bool {
        Command::new("gh")
            .arg("--version")
            .output()
            .is_ok()
    }
    
    /// 检查认证状态
    pub fn check_auth(&self) -> GtResult<GithubAuth> {
        if !self.is_available() {
            return Err(GtError::ConfigError {
                message: "GitHub CLI (gh) 未安装或不可用".to_string()
            });
        }
        
        if self.verbose {
            print_step("检查 GitHub CLI 认证状态...");
        }
        
        let output = Command::new("gh")
            .args(["auth", "status"])
            .output()
            .map_err(|e| GtError::ConfigError {
                message: format!("执行 gh auth status 失败: {}", e)
            })?;
        
        if output.status.success() {
            // 解析输出获取用户名
            let stdout = str::from_utf8(&output.stdout).unwrap_or("");
            let stderr = str::from_utf8(&output.stderr).unwrap_or("");
            let combined = format!("{}{}", stdout, stderr);
            
            // 查找 "Logged in to github.com as username" 模式
            if let Some(line) = combined.lines().find(|line| line.contains("Logged in to github.com as")) {
                if let Some(username) = line.split_whitespace().last() {
                    if self.verbose {
                        print_success(&format!("已认证为用户: {}", username));
                    }
                    return Ok(GithubAuth::Authenticated(username.to_string()));
                }
            }
            
            // 如果找不到用户名但命令成功，返回已认证但用户名未知
            if self.verbose {
                print_success("GitHub CLI 已认证");
            }
            Ok(GithubAuth::Authenticated("unknown".to_string()))
        } else {
            if self.verbose {
                print_warning("GitHub CLI 未认证");
            }
            Ok(GithubAuth::NotAuthenticated)
        }
    }
    
    /// 执行认证流程
    pub fn authenticate(&self) -> GtResult<()> {
        if self.verbose {
            print_step("启动 GitHub CLI 认证流程...");
        }
        
        let status = Command::new("gh")
            .args(["auth", "login"])
            .status()
            .map_err(|e| GtError::ConfigError {
                message: format!("执行 gh auth login 失败: {}", e)
            })?;
        
        if status.success() {
            if self.verbose {
                print_success("GitHub CLI 认证完成");
            }
            Ok(())
        } else {
            Err(GtError::GitHubAuthError)
        }
    }
    
    /// 执行 gh 命令并返回输出
    pub fn execute_command(&self, args: &[&str]) -> GtResult<String> {
        if !self.is_available() {
            return Err(GtError::ConfigError {
                message: "GitHub CLI (gh) 未安装或不可用".to_string()
            });
        }
        
        if self.verbose {
            print_step(&format!("执行: gh {}", args.join(" ")));
        }
        
        let output = Command::new("gh")
            .args(args)
            .output()
            .map_err(|e| GtError::ConfigError {
                message: format!("执行 gh 命令失败: {}", e)
            })?;
        
        if output.status.success() {
            let result = str::from_utf8(&output.stdout)
                .map_err(|e| GtError::ConfigError {
                    message: format!("解析 gh 命令输出失败: {}", e)
                })?
                .trim()
                .to_string();
            
            if self.verbose && !result.is_empty() {
                print_success("命令执行成功");
            }
            
            Ok(result)
        } else {
            let error_msg = str::from_utf8(&output.stderr)
                .unwrap_or("未知错误")
                .trim();
            
            if self.verbose {
                print_error(&format!("gh 命令执行失败: {}", error_msg));
            }
            
            Err(GtError::ConfigError {
                message: format!("gh 命令失败: {}", error_msg)
            })
        }
    }
    
    /// 在浏览器中打开 URL
    pub fn open_in_browser(&self, url: &str) -> GtResult<()> {
        if self.verbose {
            print_step(&format!("在浏览器中打开: {}", url));
        }
        
        let status = Command::new("gh")
            .args(["web", url])
            .status()
            .map_err(|e| GtError::ConfigError {
                message: format!("打开浏览器失败: {}", e)
            })?;
        
        if status.success() {
            if self.verbose {
                print_success("已在浏览器中打开");
            }
            Ok(())
        } else {
            if self.verbose {
                print_warning("无法在浏览器中打开，请手动访问该 URL");
            }
            Ok(()) // 浏览器打开失败不应该中断流程
        }
    }
    
    /// 确保已认证，如果未认证则提示用户
    pub fn ensure_authenticated(&self) -> GtResult<()> {
        match self.check_auth()? {
            GithubAuth::Authenticated(_) => Ok(()),
            GithubAuth::NotAuthenticated => {
                print_warning("GitHub CLI 未认证，需要先登录");
                print_step("请运行以下命令进行认证: gh auth login");
                Err(GtError::GitHubAuthError)
            }
            GithubAuth::Unknown => {
                print_warning("无法确定 GitHub CLI 认证状态");
                Ok(()) // 允许继续，让后续命令自己处理
            }
        }
    }
}

impl Default for GithubCli {
    fn default() -> Self {
        Self::new(true)
    }
}

/// 便捷函数：检查 GitHub CLI 是否可用且已认证
pub fn check_github_cli() -> GtResult<GithubCli> {
    let gh = GithubCli::default();
    
    if !gh.is_available() {
        return Err(GtError::ConfigError {
            message: "GitHub CLI (gh) 未安装。请安装 GitHub CLI 后再使用此功能".to_string()
        });
    }
    
    gh.ensure_authenticated()?;
    Ok(gh)
}

/// 便捷函数：创建静默模式的 GitHub CLI
pub fn github_cli_silent() -> GithubCli {
    GithubCli::new(false)
} 