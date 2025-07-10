use crate::core::crypto::{decrypt, encrypt};
use clap::Parser;
use rust_i18n::t;
use std::io::{self, Write};

#[derive(Parser)]
#[clap(name = "RustDesk ID Tool")]
#[clap(about = "A tool for encrypting and decrypting RustDesk IDs", long_about = None)]
#[clap(disable_help_flag = true)]
pub struct Cli {
    /// Custom ID to encrypt
    #[clap(short, long)]
    id: Option<String>,

    /// Encrypted ID to decrypt
    #[clap(short, long)]
    eid: Option<String>,

    /// UUID for encryption/decryption
    #[clap(short, long)]
    uuid: Option<String>,

    /// Set the language
    #[clap(short, long, default_value = "en")]
    lang: String,

    /// Show detailed help information
    #[clap(short, long, action = clap::ArgAction::SetTrue)]
    help: bool,
}

pub fn run() {
    let cli = Cli::parse();
    rust_i18n::set_locale(&cli.lang);

    // Check if help flag is set
    if cli.help {
        show_help(&cli.lang);
        return;
    }

    let has_id = cli.id.is_some();
    let has_eid = cli.eid.is_some();
    let uuid_option = cli.uuid.as_deref();

    if !has_id && !has_eid && uuid_option.is_none() {
        show_interactive_menu(&cli.lang);
        return;
    }

    let uuid = match uuid_option {
        Some(u) => u,
        None => {
            println!("{}", t!("error_uuid_required"));
            println!("{}", t!("help_prompt"));
            return;
        }
    };

    if has_id {
        if let Some(ref custom_id) = cli.id {
            match encrypt(custom_id.as_bytes(), uuid) {
                Ok(encrypted_string) => {
                    println!(
                        "{}",
                        t!("encrypt_success_with_id", id = custom_id, encrypted_id = encrypted_string)
                    );
                    println!("{}", t!("replace_id_prompt"));
                }
                Err(_) => println!("{}", t!("encryption_error")),
            }
        }
    } else if has_eid {
        if let Some(ref enc_id) = cli.eid {
            if enc_id.len() >= 2 {
                match decrypt(&enc_id.as_bytes()[2..], uuid) {
                    Ok(decrypted_bytes) => {
                        println!(
                            "{}",
                            t!("decrypt_success_with_id", id = enc_id, decrypted_id = String::from_utf8_lossy(&decrypted_bytes))
                        );
                        println!("{}", t!("compare_id_prompt"));
                    }
                    Err(_) => println!("{}", t!("decryption_error")),
                }
            } else {
                println!("{}", t!("invalid_encrypted_id_format"));
            }
        }
    }
}

pub fn show_interactive_menu(lang: &str) {
    // 显示 ASCII Logo
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

    loop {
        print!("{}", t!("enter_choice"));
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let choice = input.trim();

        match choice {
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

fn handle_encrypt_mode(lang: &str) {
    println!();
    println!("{}", t!("encrypt_mode_title"));
    println!();

    // 获取自定义 ID
    print!("{}", t!("enter_custom_id"));
    io::stdout().flush().unwrap();
    let mut custom_id = String::new();
    io::stdin().read_line(&mut custom_id).unwrap();
    let custom_id = custom_id.trim();

    if custom_id.is_empty() {
        println!("{}", t!("empty_id_error"));
        return;
    }

    // 获取 UUID
    let uuid = get_uuid_input(lang);
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

fn handle_decrypt_mode(lang: &str) {
    println!();
    println!("{}", t!("decrypt_mode_title"));
    println!();

    // 获取加密 ID
    print!("{}", t!("enter_encrypted_id"));
    io::stdout().flush().unwrap();
    let mut enc_id = String::new();
    io::stdin().read_line(&mut enc_id).unwrap();
    let enc_id = enc_id.trim();

    if enc_id.is_empty() {
        println!("{}", t!("empty_encrypted_id_error"));
        return;
    }

    if enc_id.len() < 2 {
        println!("{}", t!("invalid_encrypted_id_format_error"));
        return;
    }

    // 获取 UUID
    let uuid = get_uuid_input(lang);
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

fn get_uuid_input(_lang: &str) -> String {
    println!();
    println!("{}", t!("how_to_get_uuid"));
    println!("{}", t!("get_uuid_windows"));
    println!("{}", t!("get_uuid_macos"));
    println!();

    print!("{}", t!("enter_uuid"));
    io::stdout().flush().unwrap();
    let mut uuid = String::new();
    io::stdin().read_line(&mut uuid).unwrap();
    let uuid = uuid.trim().to_string();

    if uuid.is_empty() {
        println!("{}", t!("empty_uuid_error"));
        return String::new();
    }

    uuid
}

fn show_help(_lang: &str) {
    println!();
    println!("{}", t!("help_title"));
    println!();
    println!("{}", t!("program_function_title"));
    println!("{}", t!("program_function_desc1"));
    println!("{}", t!("program_function_desc2"));
    println!();
    println!("{}", t!("cli_usage_title"));
    println!("{}", t!("cli_usage_encrypt"));
    println!("{}", t!("cli_usage_decrypt"));
    println!();
    println!("{}", t!("cli_params_title"));
    println!("{}", t!("cli_param_id"));
    println!("{}", t!("cli_param_eid"));
    println!("{}", t!("cli_param_uuid"));
    println!("{}", t!("cli_param_lang"));
    println!("{}", t!("cli_param_help"));
    println!();
    println!("{}", t!("cli_examples_title"));
    println!("{}", t!("cli_example_encrypt"));
    println!("{}", t!("cli_example_decrypt"));
    println!("{}", t!("cli_example_help"));
    println!();
    println!("{}", t!("config_file_location_title"));
    println!("{}", t!("config_file_location_macos"));
    println!("{}", t!("config_file_location_windows"));
    println!();
    println!("{}", t!("notes_title"));
    println!("{}", t!("note_1"));
    println!("{}", t!("note_2"));
    println!("{}", t!("note_3"));
    println!();
    println!("{}", t!("get_help_title"));
    println!("{}", t!("get_help_github"));
    println!("{}", t!("get_help_email"));
    println!();
}
