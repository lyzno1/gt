//! 用户交互模块
//! 
//! 提供用户输入、确认、选择等交互功能

use super::colors::{Color, ColoredString};
use std::io::{self, Write};

/// 确认操作（Y/n）
pub fn confirm_action(message: &str, default_yes: bool) -> bool {
    let prompt = if default_yes {
        ColoredString::new("[Y/n]", Color::Yellow)
    } else {
        ColoredString::new("[y/N]", Color::Yellow)
    };
    
    print!("{} {} ", 
        ColoredString::new(message, Color::Yellow), 
        prompt
    );
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            let input = input.trim().to_lowercase();
            
            if input.is_empty() {
                default_yes
            } else {
                matches!(input.as_str(), "y" | "yes" | "是")
            }
        }
        Err(_) => default_yes,
    }
}

/// 提示用户输入文本
pub fn prompt_input(message: &str, default: Option<&str>) -> String {
    let prompt_msg = if let Some(default_val) = default {
        format!("{} [{}]", message, default_val)
    } else {
        message.to_string()
    };
    
    print!("{}: ", ColoredString::new(&prompt_msg, Color::Cyan));
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            let input = input.trim();
            if input.is_empty() && default.is_some() {
                default.unwrap().to_string()
            } else {
                input.to_string()
            }
        }
        Err(_) => default.unwrap_or("").to_string(),
    }
}

/// 多选一对话框，返回选择的索引
pub fn select_option<T>(
    title: &str, 
    options: &[(T, &str)],
    default_index: Option<usize>
) -> Option<usize>
where 
    T: Clone 
{
    println!("{}", ColoredString::new(title, Color::Cyan));
    
    for (i, (_, description)) in options.iter().enumerate() {
        let prefix = if Some(i) == default_index {
            ColoredString::new(&format!("{})", i + 1), Color::Green)
        } else {
            ColoredString::new(&format!("{})", i + 1), Color::Blue)
        };
        
        println!("   {} {}", prefix, description);
    }
    
    let prompt = if let Some(default) = default_index {
        format!("请选择 [1-{}，默认 {}]", options.len(), default + 1)
    } else {
        format!("请选择 [1-{}]", options.len())
    };
    
    print!("{}: ", ColoredString::new(&prompt, Color::Yellow));
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            let input = input.trim();
            
            if input.is_empty() && default_index.is_some() {
                return default_index;
            }
            
            if let Ok(choice) = input.parse::<usize>() {
                if choice > 0 && choice <= options.len() {
                    return Some(choice - 1);
                }
            }
            
            println!("{}", ColoredString::new("无效的选择", Color::Red));
            None
        }
        Err(_) => None,
    }
}

/// 交互式文件选择器
pub fn select_files(title: &str, files: &[String]) -> Vec<String> {
    if files.is_empty() {
        println!("{}", ColoredString::new("没有可选择的文件", Color::Yellow));
        return Vec::new();
    }
    
    println!("{}", ColoredString::new(title, Color::Cyan));
    println!("输入文件编号（用空格分隔多个编号），或输入 'a' 选择全部，输入 'q' 取消：");
    
    for (i, file) in files.iter().enumerate() {
        println!("[{}] {}", 
            ColoredString::new(&format!("{}", i), Color::Blue), 
            file
        );
    }
    
    print!("{}: ", 
        ColoredString::new(&format!("请选择 (0-{}, a=全部, q=取消)", files.len() - 1), Color::Yellow)
    );
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            let input = input.trim();
            
            if input == "q" {
                println!("{}", ColoredString::new("已取消选择", Color::Yellow));
                return Vec::new();
            }
            
            if input == "a" {
                return files.to_vec();
            }
            
            let mut selected = Vec::new();
            for part in input.split_whitespace() {
                if let Ok(index) = part.parse::<usize>() {
                    if index < files.len() {
                        selected.push(files[index].clone());
                    } else {
                        println!("{}", 
                            ColoredString::new(&format!("忽略无效选择: {}", part), Color::Yellow)
                        );
                    }
                }
            }
            
            if !selected.is_empty() {
                println!("{} {}:", 
                    ColoredString::new("已选择", Color::Green),
                    ColoredString::new(&format!("{} 个文件", selected.len()), Color::Bold)
                );
                for file in &selected {
                    println!(" - {}", file);
                }
            }
            
            selected
        }
        Err(_) => Vec::new(),
    }
}

/// 等待用户按 Enter 继续
pub fn wait_for_enter(message: Option<&str>) {
    let msg = message.unwrap_or("按 Enter 键继续...");
    print!("{}", ColoredString::new(msg, Color::Cyan));
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    let _ = io::stdin().read_line(&mut input);
} 