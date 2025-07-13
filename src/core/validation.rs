//! 输入验证模块
//!
//! 提供各种输入数据的验证功能。

use super::error::{AppError, AppResult};
use regex::Regex;
use rust_i18n::t;
use std::sync::OnceLock;

/// 标准 UUID 格式验证的正则表达式（带连字符）
static UUID_STANDARD_REGEX: OnceLock<Regex> = OnceLock::new();

/// Linux 机器 ID 格式验证的正则表达式（无连字符）
static UUID_LINUX_REGEX: OnceLock<Regex> = OnceLock::new();

/// 获取标准 UUID 验证正则表达式
fn get_uuid_standard_regex() -> &'static Regex {
    UUID_STANDARD_REGEX.get_or_init(|| {
        Regex::new(r"^[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}$")
            .expect(&t!("regex_compile_error_standard"))
    })
}

/// 获取 Linux 机器 ID 验证正则表达式
fn get_uuid_linux_regex() -> &'static Regex {
    UUID_LINUX_REGEX.get_or_init(|| {
        Regex::new(r"^[0-9a-fA-F]{32}$")
            .expect(&t!("regex_compile_error_linux"))
    })
}

/// 验证器 trait
trait Validator<T: ?Sized> {
    /// 验证输入数据
    fn validate(&self, input: &T) -> AppResult<()>;
}

/// UUID 验证器
struct UuidValidator;

impl Validator<str> for UuidValidator {
    fn validate(&self, uuid: &str) -> AppResult<()> {
        if uuid.trim().is_empty() {
            return Err(AppError::ValidationError(t!("validation_error_uuid_empty")));
        }

        let uuid = uuid.trim();

        // 检查 UUID 格式（支持标准格式和 Linux 机器 ID 格式）
        let is_standard_format = get_uuid_standard_regex().is_match(uuid);
        let is_linux_format = get_uuid_linux_regex().is_match(uuid);
        
        if !is_standard_format && !is_linux_format {
            return Err(AppError::ValidationError(
                t!("validation_error_uuid_format"),
            ));
        }

        Ok(())
    }
}

/// 加密 ID 验证器
struct EncryptedIdValidator;

impl Validator<str> for EncryptedIdValidator {
    fn validate(&self, encrypted_id: &str) -> AppResult<()> {
        if encrypted_id.trim().is_empty() {
            return Err(AppError::ValidationError(t!("validation_error_encrypted_id_empty")));
        }

        let encrypted_id = encrypted_id.trim();

        // 检查最小长度
        if encrypted_id.len() < 2 {
            return Err(AppError::ValidationError(t!("validation_error_encrypted_id_length")));
        }

        // 检查是否包含有效的 base64 字符
        if !encrypted_id
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '+' || c == '/' || c == '=')
        {
            return Err(AppError::ValidationError(
                t!("validation_error_encrypted_id_chars"),
            ));
        }

        Ok(())
    }
}

/// 自定义 ID 验证器
struct CustomIdValidator;

impl Validator<str> for CustomIdValidator {
    fn validate(&self, custom_id: &str) -> AppResult<()> {
        if custom_id.trim().is_empty() {
            return Err(AppError::ValidationError(t!("validation_error_custom_id_empty")));
        }

        let custom_id = custom_id.trim();

        // 检查长度限制
        if custom_id.len() > 100 {
            return Err(AppError::ValidationError(
                t!("validation_error_custom_id_length"),
            ));
        }

        // 检查是否包含不安全字符
        if custom_id.contains(['\0', '\n', '\r', '\t']) {
            return Err(AppError::ValidationError(
                t!("validation_error_custom_id_control_chars"),
            ));
        }

        Ok(())
    }
}

/// 验证 UUID 格式
pub fn validate_uuid(uuid: &str) -> AppResult<()> {
    UuidValidator.validate(uuid)
}

/// 验证自定义 ID
pub fn validate_custom_id(custom_id: &str) -> AppResult<()> {
    CustomIdValidator.validate(custom_id)
}

/// 验证加密 ID
pub fn validate_encrypted_id(encrypted_id: &str) -> AppResult<()> {
    EncryptedIdValidator.validate(encrypted_id)
}
