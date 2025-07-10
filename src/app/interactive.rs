use crate::app::help::show_help;
use crate::app::input::{is_empty_input, prompt_input};
use crate::app::uuid::get_uuid_input;
use crate::core::crypto::{decrypt, encrypt};
use rust_i18n::t;

/// 显示交互式主菜单
pub fn show_interactive_menu(lang: &str) {
    show_logo();
    show_menu_options();
    handle_menu_loop(lang);
}

/// 显示 ASCII Logo
fn show_logo() {
    println!(
        r#"
 ____            _   ____            _    
|  _ \ _   _ ___| |_|  _ \  ___  ___| | __
| |_) | | | / __| __| | | |/ _ \/ __| |/ /
|  _ <| |_| \__ \ |_| |_| |  __/\__ \   < 
|_| \_\\__,_|___/\__|____/ \___||___/_|\_\
                                        
   {} v{}
"#,
        t!("app_title"),
        env!("CARGO_PKG_VERSION")
    );
}

/// 显示菜单选项
fn show_menu_options() {
    println!("═══════════════════════════════════════════════════════════");
    println!("{}", t!("welcome"));
    println!("{}", t!("description"));
    println!("{}", t!("security"));
    println!("═══════════════════════════════════════════════════════════");
    println!();

    println!("{}", t!("choose_action"));
    println!();
    println!("{}", t!("generate_id"));
    println!("{}", t!("generate_id_desc"));
    println!();
    println!("{}", t!("validate_id"));
    println!("{}", t!("validate_id_desc"));
    println!();
    println!("{}", t!("help"));
    println!("{}", t!("help_desc"));
    println!();
    println!("{}", t!("exit"));
    println!();
}

/// 处理菜单循环
fn handle_menu_loop(lang: &str) {
    loop {
        let choice = prompt_input(&t!("enter_choice"));

        match choice.as_str() {
            "1" => {
                handle_encrypt_mode(lang);
                break;
            }
            "2" => {
                handle_decrypt_mode(lang);
                break;
            }
            "3" => {
                show_help(lang);
                break;
            }
            "0" => {
                println!("{}", t!("thanks"));
                break;
            }
            _ => {
                println!("{}", t!("invalid_choice"));
                println!();
            }
        }
    }
}

/// 处理加密模式
fn handle_encrypt_mode(_lang: &str) {
    println!();
    println!("{}", t!("encrypt_mode_title"));
    println!();

    // 获取自定义 ID
    let custom_id = prompt_input(&t!("enter_custom_id"));

    if is_empty_input(&custom_id) {
        println!("{}", t!("empty_id_error"));
        return;
    }

    // 获取 UUID
    let uuid = get_uuid_input();
    if uuid.is_empty() {
        return;
    }

    // 执行加密
    match encrypt(custom_id.as_bytes(), &uuid) {
        Ok(encrypted_string) => {
            let final_id = format!("00{}", encrypted_string);
            println!();
            println!("{}", t!("encrypt_success"));
            println!("{}", t!("original_id", id = custom_id));
            println!("{}", t!("encrypted_id", id = final_id));
            println!();
            println!("{}", t!("usage_instructions"));
            println!("{}", t!("usage_1"));
            println!("{}", t!("usage_2"));
            println!("{}", t!("usage_3"));
            println!("{}", t!("usage_4"));
        }
        Err(_) => {
            println!("{}", t!("encrypt_error"));
        }
    }
}

/// 处理解密模式
fn handle_decrypt_mode(_lang: &str) {
    println!();
    println!("{}", t!("decrypt_mode_title"));
    println!();

    // 获取加密 ID
    let enc_id = prompt_input(&t!("enter_encrypted_id"));

    if is_empty_input(&enc_id) {
        println!("{}", t!("empty_encrypted_id_error"));
        return;
    }

    if enc_id.len() < 2 {
        println!("{}", t!("invalid_encrypted_id_format_error"));
        return;
    }

    // 获取 UUID
    let uuid = get_uuid_input();
    if uuid.is_empty() {
        return;
    }

    // 执行解密
    match decrypt(&enc_id.as_bytes()[2..], &uuid) {
        Ok(decrypted_bytes) => match String::from_utf8(decrypted_bytes) {
            Ok(decrypted_id) => {
                println!();
                println!("{}", t!("decrypt_success_title"));
                println!("{}", t!("encrypted_id_label", id = enc_id));
                println!("{}", t!("original_id_label", id = decrypted_id));
                println!();
                println!("{}", t!("compare_id_suggestion"));
            }
            Err(_) => {
                println!("{}", t!("invalid_decryption_result_error"));
            }
        },
        Err(_) => {
            println!("{}", t!("decryption_failed_error"));
        }
    }
}
