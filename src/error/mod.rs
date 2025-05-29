//! 错误处理系统
//! 
//! 提供统一的错误类型和处理机制，支持丰富的错误信息和恢复策略。

pub mod types;
pub mod handler;
pub mod recovery;

pub use types::{GtError, GtResult};
pub use handler::ErrorHandler;
pub use recovery::RecoveryStrategy; 