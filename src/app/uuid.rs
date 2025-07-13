use machine_uid;
use rust_i18n::t;
use std::io::{self, Write};

/// UUID 处理结果
#[derive(Debug, Clone)]
pub enum UuidResult {
    /// 成功获取 UUID
    Success(String),
    /// 用户取消操作
    Cancelled,
    /// 发生错误
    Error,
}

/// 自动检测并获取 UUID，支持用户确认
pub fn get_uuid_with_confirmation() -> UuidResult {
    match machine_uid::get() {
        Ok(machine_uuid) => {
            println!("{}", t!("auto_detected_uuid"));
            println!("{}: {}", t!("detected_uuid_label"), machine_uuid);
            println!();

            loop {
                print!("{}", t!("confirm_uuid_prompt"));
                if io::stdout().flush().is_err() {
                    return UuidResult::Error;
                }
                let mut choice = String::new();
                if io::stdin().read_line(&mut choice).is_err() {
                    return UuidResult::Error;
                }
                let choice = choice.trim().to_lowercase();

                match choice.as_str() {
                    "" | "y" | "yes" => {
                        return UuidResult::Success(machine_uuid);
                    }
                    "n" | "no" => {
                        println!("{}", t!("manual_uuid_required"));
                        break;
                    }
                    _ => {
                        println!("{}", t!("invalid_choice_yn"));
                        continue;
                    }
                }
            }

            // 用户选择手动输入
            get_manual_uuid_input()
        }
        Err(e) => {
            println!("{}: {}", t!("auto_uuid_failed"), e);
            println!("{}", t!("manual_uuid_required"));
            get_manual_uuid_input()
        }
    }
}

/// 获取手动输入的 UUID
fn get_manual_uuid_input() -> UuidResult {
    println!();
    println!("{}", t!("how_to_get_uuid"));
    println!("{}", t!("get_uuid_windows"));
    println!("{}", t!("get_uuid_macos"));
    println!();

    print!("{}", t!("enter_uuid"));
    if io::stdout().flush().is_err() {
        return UuidResult::Error;
    }
    let mut uuid = String::new();
    if io::stdin().read_line(&mut uuid).is_err() {
        return UuidResult::Error;
    }
    let uuid = uuid.trim().to_string();

    if uuid.is_empty() {
        println!("{}", t!("operation_cancelled"));
        return UuidResult::Cancelled;
    }

    UuidResult::Success(uuid)
}

/// 简化版本的 UUID 获取，用于交互模式
pub fn get_uuid_input() -> String {
    match get_uuid_with_confirmation() {
        UuidResult::Success(uuid) => uuid,
        UuidResult::Cancelled | UuidResult::Error => String::new(),
    }
}
