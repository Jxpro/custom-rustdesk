use crate::core::crypto::{decrypt, encrypt};
use rust_i18n::t;

/// 加密操作结果
pub enum EncryptResult {
    Success {
        original_id: String,
        encrypted_id: String,
    },
    Error(String),
}

/// 解密操作结果
pub enum DecryptResult {
    Success {
        encrypted_id: String,
        decrypted_id: String,
    },
    Error(String),
}

/// 执行加密操作
pub fn perform_encrypt(custom_id: &str, uuid: &str) -> EncryptResult {
    match encrypt(custom_id.as_bytes(), uuid) {
        Ok(encrypted_string) => EncryptResult::Success {
            original_id: custom_id.to_string(),
            encrypted_id: encrypted_string,
        },
        Err(_) => EncryptResult::Error(t!("encryption_error")),
    }
}

/// 执行解密操作
pub fn perform_decrypt(enc_id: &str, uuid: &str) -> DecryptResult {
    if enc_id.len() < 2 {
        return DecryptResult::Error(t!("invalid_encrypted_id_format"));
    }

    match decrypt(&enc_id.as_bytes()[2..], uuid) {
        Ok(decrypted_bytes) => match String::from_utf8(decrypted_bytes) {
            Ok(decrypted_id) => DecryptResult::Success {
                encrypted_id: enc_id.to_string(),
                decrypted_id,
            },
            Err(_) => DecryptResult::Error(t!("invalid_decryption_result_error")),
        },
        Err(_) => DecryptResult::Error(t!("decryption_error")),
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
        println!("{}", t!("replace_id_prompt"));
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
        println!("{}", t!("compare_id_prompt"));
    }
}

/// 显示操作错误
pub fn display_error(error_msg: &str) {
    println!("{}", error_msg);
}
