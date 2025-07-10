//! 应用程序单元测试模块
//!
//! 包含各个模块的单元测试。

#[cfg(test)]
mod tests {

    use crate::core::error::AppError;
    use crate::core::validation::{validate_custom_id, validate_encrypted_id, validate_uuid};

    /// 测试 UUID 验证
    #[test]
    fn test_uuid_validation() {
        // 有效的 UUID
        assert!(validate_uuid("550e8400-e29b-41d4-a716-446655440000").is_ok());
        assert!(validate_uuid("6ba7b810-9dad-11d1-80b4-00c04fd430c8").is_ok());

        // 无效的 UUID
        assert!(validate_uuid("").is_err());
        assert!(validate_uuid("invalid-uuid").is_err());
        assert!(validate_uuid("550e8400-e29b-41d4-a716").is_err());
        assert!(validate_uuid("550e8400-e29b-41d4-a716-446655440000-extra").is_err());
    }

    /// 测试自定义 ID 验证
    #[test]
    fn test_custom_id_validation() {
        // 有效的自定义 ID
        assert!(validate_custom_id("test123").is_ok());
        assert!(validate_custom_id("用户ID").is_ok());
        assert!(validate_custom_id("user@example.com").is_ok());

        // 无效的自定义 ID
        assert!(validate_custom_id("").is_err());
        assert!(validate_custom_id("   ").is_err());
        assert!(validate_custom_id("test\0id").is_err());
        assert!(validate_custom_id("test\nid").is_err());
        assert!(validate_custom_id(&"a".repeat(101)).is_err());
    }

    /// 测试加密 ID 验证
    #[test]
    fn test_encrypted_id_validation() {
        // 有效的加密 ID（模拟 base64 格式）
        assert!(validate_encrypted_id("SGVsbG8gV29ybGQ=").is_ok());
        assert!(validate_encrypted_id("YWJjZGVmZ2hpams=").is_ok());

        // 无效的加密 ID
        assert!(validate_encrypted_id("").is_err());
        assert!(validate_encrypted_id("a").is_err());
        assert!(validate_encrypted_id("invalid@#$%").is_err());
    }

    /// 测试错误类型转换
    #[test]
    fn test_error_conversions() {
        // 测试从字符串转换
        let error: AppError = "测试错误".into();
        assert!(matches!(error, AppError::ValidationError(_)));

        // 测试从 String 转换
        let error: AppError = "测试错误".to_string().into();
        assert!(matches!(error, AppError::ValidationError(_)));
    }

    /// 测试错误显示
    #[test]
    fn test_error_display() {
        let error = AppError::ValidationError("测试错误".to_string());
        let display = format!("{}", error);
        assert!(display.contains("测试错误"));
    }
}
