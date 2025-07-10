//! 输入验证模块
//!
//! 提供各种输入数据的验证功能。

use super::error::{AppError, AppResult};
use regex::Regex;
use std::sync::OnceLock;

/// UUID 格式验证的正则表达式
static UUID_REGEX: OnceLock<Regex> = OnceLock::new();

/// 获取 UUID 验证正则表达式
fn get_uuid_regex() -> &'static Regex {
    UUID_REGEX.get_or_init(|| {
        Regex::new(r"^[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}$")
            .expect("UUID 正则表达式编译失败")
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
            return Err(AppError::ValidationError("UUID 不能为空".to_string()));
        }

        let uuid = uuid.trim();

        // 检查 UUID 格式
        if !get_uuid_regex().is_match(uuid) {
            return Err(AppError::ValidationError(
                "UUID 格式无效，应为 xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx 格式".to_string(),
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
            return Err(AppError::ValidationError("加密 ID 不能为空".to_string()));
        }

        let encrypted_id = encrypted_id.trim();

        // 检查最小长度
        if encrypted_id.len() < 2 {
            return Err(AppError::ValidationError("加密 ID 长度不足".to_string()));
        }

        // 检查是否包含有效的 base64 字符
        if !encrypted_id
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '+' || c == '/' || c == '=')
        {
            return Err(AppError::ValidationError(
                "加密 ID 包含无效字符".to_string(),
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
            return Err(AppError::ValidationError("自定义 ID 不能为空".to_string()));
        }

        let custom_id = custom_id.trim();

        // 检查长度限制
        if custom_id.len() > 100 {
            return Err(AppError::ValidationError(
                "自定义 ID 长度不能超过 100 个字符".to_string(),
            ));
        }

        // 检查是否包含不安全字符
        if custom_id.contains(['\0', '\n', '\r', '\t']) {
            return Err(AppError::ValidationError(
                "自定义 ID 不能包含控制字符".to_string(),
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
