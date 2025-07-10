//! 应用程序错误处理模块
//!
//! 定义了应用程序中使用的各种错误类型，提供统一的错误处理机制。

use rust_i18n::t;
use std::fmt;

/// 应用程序主要错误类型
#[derive(Debug, Clone)]
pub enum AppError {
    /// 输入验证失败
    ValidationError(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::ValidationError(msg) => write!(f, "{}: {}", t!("validation_error"), msg),
        }
    }
}

impl std::error::Error for AppError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

/// 应用程序结果类型别名
pub type AppResult<T> = Result<T, AppError>;

/// 从字符串转换为验证错误
impl From<String> for AppError {
    fn from(msg: String) -> Self {
        AppError::ValidationError(msg)
    }
}

/// 从 &str 转换为验证错误
impl From<&str> for AppError {
    fn from(msg: &str) -> Self {
        AppError::ValidationError(msg.to_string())
    }
}
