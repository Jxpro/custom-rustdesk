use super::crypto::{decrypt, encrypt};
use super::validation::{validate_custom_id, validate_encrypted_id, validate_uuid};
use rust_i18n::t;
use arboard::Clipboard;

/// 加密操作结果
#[derive(Debug, Clone)]
pub enum EncryptResult {
    Success {
        original_id: String,
        encrypted_id: String,
    },
    Error(String),
}

/// 解密操作结果
#[derive(Debug, Clone)]
pub enum DecryptResult {
    Success {
        encrypted_id: String,
        decrypted_id: String,
    },
    Error(String),
}

/// 执行加密操作
pub fn perform_encrypt(custom_id: &str, uuid: &str) -> EncryptResult {
    // 输入验证
    if let Err(e) = validate_custom_id(custom_id) {
        return EncryptResult::Error(format!("{}", e));
    }

    if let Err(e) = validate_uuid(uuid) {
        return EncryptResult::Error(format!("{}", e));
    }

    match encrypt(custom_id.as_bytes(), uuid) {
        Ok(encrypted_string) => EncryptResult::Success {
            original_id: custom_id.to_string(),
            encrypted_id: encrypted_string,
        },
        Err(_) => EncryptResult::Error(format!("{}", t!("encryption_error"))),
    }
}

/// 执行解密操作
pub fn perform_decrypt(enc_id: &str, uuid: &str) -> DecryptResult {
    // 输入验证
    if let Err(e) = validate_encrypted_id(enc_id) {
        return DecryptResult::Error(format!("{}", e));
    }

    if let Err(e) = validate_uuid(uuid) {
        return DecryptResult::Error(format!("{}", e));
    }

    if enc_id.len() < 2 {
        return DecryptResult::Error(t!("invalid_encrypted_id_format"));
    }

    match decrypt(&enc_id.as_bytes()[2..], uuid) {
        Ok(decrypted_bytes) => match String::from_utf8(decrypted_bytes) {
            Ok(decrypted_id) => DecryptResult::Success {
                encrypted_id: enc_id.to_string(),
                decrypted_id,
            },
            Err(e) => {
                DecryptResult::Error(format!("{}: {}", t!("invalid_decryption_result_error"), e))
            }
        },
        Err(_) => DecryptResult::Error(format!("{}", t!("decryption_error"))),
    }
}

/// 复制文本到剪切板
fn copy_to_clipboard(text: &str) -> Result<(), String> {
    match Clipboard::new() {
        Ok(mut clipboard) => {
            match clipboard.set_text(text) {
                Ok(_) => Ok(()),
                Err(e) => Err(format!("复制到剪切板失败: {}", e)),
            }
        }
        Err(e) => Err(format!("无法访问剪切板: {}", e)),
    }
}

/// 显示加密成功结果
pub fn display_encrypt_success(result: &EncryptResult) {
    if let EncryptResult::Success {
        original_id,
        encrypted_id,
    } = result
    {
        println!(
            "{}",
            t!(
                "encrypt_success_with_id",
                id = original_id,
                encrypted_id = encrypted_id
            )
        );
        
        // 尝试复制加密ID到剪切板（包含00前缀）
        let full_encrypted_id = format!("00{}", encrypted_id);
        match copy_to_clipboard(&full_encrypted_id) {
            Ok(_) => println!("{}", t!("clipboard_copy_success")),
            Err(_) => println!("{}", t!("clipboard_copy_failed")),
        }
        
        // 显示详细的使用说明
        println!();
        println!("{}", t!("usage_instructions"));
        println!("{}", t!("usage_1"));
        println!("{}", t!("usage_2"));
        println!("{}", t!("usage_3"));
        println!("{}", t!("usage_4"));
        println!();
        println!("{}", t!("config_file_location_title"));
        println!("{}", t!("config_file_location_macos"));
        println!("{}", t!("config_file_location_windows"));
        println!("{}", t!("config_file_location_windows_service"));
    }
}

/// 显示解密成功结果
pub fn display_decrypt_success(result: &DecryptResult) {
    if let DecryptResult::Success {
        encrypted_id,
        decrypted_id,
    } = result
    {
        println!(
            "{}",
            t!(
                "decrypt_success_with_id",
                id = encrypted_id,
                decrypted_id = decrypted_id
            )
        );
        
        // 尝试复制解密ID到剪切板
        match copy_to_clipboard(decrypted_id) {
            Ok(_) => println!("{}", t!("clipboard_copy_success")),
            Err(_) => println!("{}", t!("clipboard_copy_failed")),
        }
        
        println!("{}", t!("compare_id_prompt"));
    }
}

/// 显示操作错误
pub fn display_error(error_msg: &str) {
    println!("{}", error_msg);
}
