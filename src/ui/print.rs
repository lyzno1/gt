//! 打印输出模块
//! 
//! 提供不同级别的消息打印功能，对应 gw 的 utils_print.sh

use super::colors::{Color, ColoredString};
use std::io::{self, Write};

/// 消息级别
#[derive(Debug, Clone, Copy)]
pub enum MessageLevel {
    Info,
    Step,
    Success,
    Warning,
    Error,
}

impl MessageLevel {
    fn color(self) -> Color {
        match self {
            MessageLevel::Info => Color::Blue,
            MessageLevel::Step => Color::Cyan,
            MessageLevel::Success => Color::Green,
            MessageLevel::Warning => Color::Yellow,
            MessageLevel::Error => Color::Red,
        }
    }
    
    fn prefix(self) -> &'static str {
        match self {
            MessageLevel::Info => "[INFO]",
            MessageLevel::Step => "[STEP]",
            MessageLevel::Success => "[SUCCESS]",
            MessageLevel::Warning => "[WARN]",
            MessageLevel::Error => "[ERROR]",
        }
    }
}

/// 打印带前缀的消息
fn print_message(level: MessageLevel, message: &str) {
    let colored_prefix = ColoredString::new(level.prefix(), level.color());
    
    match level {
        MessageLevel::Warning | MessageLevel::Error => {
            eprintln!("{} {}", colored_prefix, message);
        }
        _ => {
            println!("{} {}", colored_prefix, message);
        }
    }
}

/// 打印信息消息
pub fn print_info(message: &str) {
    print_message(MessageLevel::Info, message);
}

/// 打印步骤消息
pub fn print_step(message: &str) {
    print_message(MessageLevel::Step, message);
}

/// 打印成功消息
pub fn print_success(message: &str) {
    print_message(MessageLevel::Success, message);
}

/// 打印警告消息
pub fn print_warning(message: &str) {
    print_message(MessageLevel::Warning, message);
}

/// 打印错误消息
pub fn print_error(message: &str) {
    print_message(MessageLevel::Error, message);
}

/// 打印分隔线
pub fn print_separator(title: Option<&str>) {
    let cyan = ColoredString::new("===", Color::Cyan);
    
    match title {
        Some(title) => println!("{} {} {}", cyan, title, cyan),
        None => println!("{}", ColoredString::new("========================", Color::Cyan)),
    }
}

/// 打印进度条（简单版本）
pub fn print_progress(current: usize, total: usize, message: &str) {
    let percentage = if total > 0 { (current * 100) / total } else { 0 };
    let progress_bar = "=".repeat(percentage / 5);
    let empty_bar = " ".repeat(20 - progress_bar.len());
    
    print!("\r{} [{}>{}] {}% ", 
        ColoredString::new("[PROGRESS]", Color::Cyan),
        ColoredString::new(&progress_bar, Color::Green),
        empty_bar,
        percentage
    );
    
    if !message.is_empty() {
        print!("{}", message);
    }
    
    if current >= total {
        println!(); // 完成时换行
    } else {
        io::stdout().flush().unwrap();
    }
}

/// 便捷宏：格式化打印
#[macro_export]
macro_rules! print_info {
    ($($arg:tt)*) => {
        $crate::ui::print_info(&format!($($arg)*))
    };
}

#[macro_export]
macro_rules! print_step {
    ($($arg:tt)*) => {
        $crate::ui::print_step(&format!($($arg)*))
    };
}

#[macro_export]
macro_rules! print_success {
    ($($arg:tt)*) => {
        $crate::ui::print_success(&format!($($arg)*))
    };
}

#[macro_export]
macro_rules! print_warning {
    ($($arg:tt)*) => {
        $crate::ui::print_warning(&format!($($arg)*))
    };
}

#[macro_export]
macro_rules! print_error {
    ($($arg:tt)*) => {
        $crate::ui::print_error(&format!($($arg)*))
    };
} 