//! 终端颜色输出模块
//! 
//! 提供彩色终端输出功能，对应 gw 的 colors.sh

use std::fmt;

/// ANSI 颜色代码
#[derive(Debug, Clone, Copy)]
pub enum Color {
    Red,
    Green, 
    Yellow,
    Blue,
    Cyan,
    Purple,
    Bold,
    Reset,
}

impl Color {
    /// 获取 ANSI 颜色代码
    pub fn code(self) -> &'static str {
        match self {
            Color::Red => "\x1b[0;31m",
            Color::Green => "\x1b[0;32m", 
            Color::Yellow => "\x1b[1;33m",
            Color::Blue => "\x1b[0;34m",
            Color::Cyan => "\x1b[0;36m",
            Color::Purple => "\x1b[0;35m",
            Color::Bold => "\x1b[1m",
            Color::Reset => "\x1b[0m",
        }
    }
    
    /// 检查是否支持颜色输出
    pub fn is_supported() -> bool {
        // 检查是否在 TTY 中运行且支持颜色
        use std::io::IsTerminal;
        std::io::stderr().is_terminal() && 
        std::env::var("NO_COLOR").is_err() &&
        !matches!(std::env::var("TERM").as_deref(), Ok("dumb") | Err(_))
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if Color::is_supported() {
            write!(f, "{}", self.code())
        } else {
            Ok(())
        }
    }
}

/// 彩色字符串包装器
pub struct ColoredString {
    text: String,
    color: Color,
}

impl ColoredString {
    pub fn new(text: impl Into<String>, color: Color) -> Self {
        Self {
            text: text.into(),
            color,
        }
    }
}

impl fmt::Display for ColoredString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if Color::is_supported() {
            write!(f, "{}{}{}", self.color, self.text, Color::Reset)
        } else {
            write!(f, "{}", self.text)
        }
    }
}

/// 便捷宏：创建彩色文本
#[macro_export]
macro_rules! colored {
    ($text:expr, $color:expr) => {
        $crate::ui::ColoredString::new($text, $color)
    };
}

/// 便捷函数：红色文本
pub fn red(text: impl Into<String>) -> ColoredString {
    ColoredString::new(text, Color::Red)
}

/// 便捷函数：绿色文本
pub fn green(text: impl Into<String>) -> ColoredString {
    ColoredString::new(text, Color::Green)
}

/// 便捷函数：黄色文本
pub fn yellow(text: impl Into<String>) -> ColoredString {
    ColoredString::new(text, Color::Yellow)
}

/// 便捷函数：蓝色文本
pub fn blue(text: impl Into<String>) -> ColoredString {
    ColoredString::new(text, Color::Blue)
}

/// 便捷函数：青色文本
pub fn cyan(text: impl Into<String>) -> ColoredString {
    ColoredString::new(text, Color::Cyan)
}

/// 便捷函数：紫色文本
pub fn purple(text: impl Into<String>) -> ColoredString {
    ColoredString::new(text, Color::Purple)
}

/// 便捷函数：粗体文本
pub fn bold(text: impl Into<String>) -> ColoredString {
    ColoredString::new(text, Color::Bold)
} 