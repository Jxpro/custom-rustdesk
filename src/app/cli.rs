use crate::core::crypto::{decrypt, encrypt};
use clap::Parser;
use std::io::{self, Write};

#[derive(Parser)]
#[clap(name = "RustDesk ID Tool")]
#[clap(about = "A tool for encrypting and decrypting RustDesk IDs", long_about = None)]
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
}

pub fn run() {
    let cli = Cli::parse();

    let has_id = cli.id.is_some();
    let has_eid = cli.eid.is_some();
    let uuid_option = cli.uuid.as_deref();

    if !has_id && !has_eid && uuid_option.is_none() {
        show_interactive_menu();
        return;
    }

    let uuid = match uuid_option {
        Some(u) => u,
        None => {
            println!("Error: UUID is required for encryption or decryption.");
            println!("For help use --help");
            return;
        }
    };

    if has_id {
        if let Some(ref custom_id) = cli.id {
            match encrypt(custom_id.as_bytes(), uuid) {
                Ok(encrypted_string) => {
                    println!(
                        "\"{}\" is encrypted to \"00{}\"",
                        custom_id, encrypted_string
                    );
                    println!("Please replace the id with the enc_id field in the config file");
                }
                Err(_) => println!("Error occurred during encryption"),
            }
        }
    } else if has_eid {
        if let Some(ref enc_id) = cli.eid {
            if enc_id.len() >= 2 {
                match decrypt(&enc_id.as_bytes()[2..], uuid) {
                    Ok(decrypted_bytes) => {
                        println!(
                            "\"{}\" is decrypted to \"{}\"",
                            enc_id,
                            String::from_utf8_lossy(&decrypted_bytes)
                        );
                        println!("Please compare the id with the enc_id field in the config file");
                    }
                    Err(_) => println!("Error occurred during decryption"),
                }
            } else {
                println!("Invalid encrypted ID format.");
            }
        }
    }
}

pub fn show_interactive_menu() {
    // 显示 ASCII Logo
    println!(
        r#"
 ____            _   ____            _    
|  _ \ _   _ ___| |_|  _ \  ___  ___| | __
| |_) | | | / __| __| | | |/ _ \/ __| |/ /
|  _ <| |_| \__ \ |_| |_| |  __/\__ \   < 
|_| \_\\__,_|___/\__|____/ \___||___/_|\_\
                                        
   自定义 ID 工具 v0.2.0
"#
    );

    println!("═══════════════════════════════════════════════════════════");
    println!("🎯 欢迎使用 RustDesk 自定义 ID 工具！");
    println!("📝 本工具可以帮助您生成和验证 RustDesk 的自定义 ID");
    println!("🔐 使用您的机器 UUID 作为加密密钥，确保安全性");
    println!("═══════════════════════════════════════════════════════════");
    println!();

    println!("请选择您要执行的操作：");
    println!();
    println!("  [1] 🔑 生成自定义 ID (加密模式)");
    println!("      将您的自定义 ID 加密为 RustDesk 可用格式");
    println!();
    println!("  [2] 🔍 验证加密 ID (解密模式)");
    println!("      验证现有的加密 ID 是否正确");
    println!();
    println!("  [3] 📖 查看使用帮助");
    println!("      显示详细的使用说明和示例");
    println!();
    println!("  [0] 🚪 退出程序");
    println!();

    loop {
        print!("请输入您的选择 (0-3): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let choice = input.trim();

        match choice {
            "1" => {
                handle_encrypt_mode();
                break;
            }
            "2" => {
                handle_decrypt_mode();
                break;
            }
            "3" => {
                show_help();
                break;
            }
            "0" => {
                println!("👋 感谢使用，再见！");
                break;
            }
            _ => {
                println!("❌ 无效选择，请输入 0-3 之间的数字");
                println!();
            }
        }
    }
}

fn handle_encrypt_mode() {
    println!();
    println!("🔑 === 生成自定义 ID (加密模式) ===");
    println!();

    // 获取自定义 ID
    print!("请输入您的自定义 ID: ");
    io::stdout().flush().unwrap();
    let mut custom_id = String::new();
    io::stdin().read_line(&mut custom_id).unwrap();
    let custom_id = custom_id.trim();

    if custom_id.is_empty() {
        println!("❌ 自定义 ID 不能为空！");
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
            println!("✅ 加密成功！");
            println!("📋 原始 ID: {}", custom_id);
            println!("🔐 加密后的 ID: {}", final_id);
            println!();
            println!("📝 使用说明：");
            println!("   1. 复制上面的加密 ID");
            println!("   2. 打开 RustDesk 配置文件");
            println!("   3. 将 enc_id 字段替换为加密后的 ID");
            println!("   4. 重启 RustDesk 服务");
        }
        Err(_) => {
            println!("❌ 加密过程中发生错误，请检查输入是否正确");
        }
    }
}

fn handle_decrypt_mode() {
    println!();
    println!("🔍 === 验证加密 ID (解密模式) ===");
    println!();

    // 获取加密 ID
    print!("请输入要验证的加密 ID: ");
    io::stdout().flush().unwrap();
    let mut enc_id = String::new();
    io::stdin().read_line(&mut enc_id).unwrap();
    let enc_id = enc_id.trim();

    if enc_id.is_empty() {
        println!("❌ 加密 ID 不能为空！");
        return;
    }

    if enc_id.len() < 2 {
        println!("❌ 加密 ID 格式不正确！");
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
                println!("✅ 解密成功！");
                println!("🔐 加密 ID: {}", enc_id);
                println!("📋 原始 ID: {}", decrypted_id);
                println!();
                println!("💡 请将解密后的 ID 与您期望的自定义 ID 进行比较");
            }
            Err(_) => {
                println!("❌ 解密结果包含无效字符，请检查加密 ID 是否正确");
            }
        },
        Err(_) => {
            println!("❌ 解密失败，请检查加密 ID 和 UUID 是否正确");
        }
    }
}

fn get_uuid_input() -> String {
    println!();
    println!("📋 如何获取 UUID：");
    println!(
        "   Windows: 注册表 HKEY_LOCAL_MACHINE\\SOFTWARE\\Microsoft\\Cryptography 中的 MachineGuid"
    );
    println!("   macOS: 终端执行 ioreg -rd1 -c IOPlatformExpertDevice | grep IOPlatformUUID");
    println!();

    print!("请输入您的机器 UUID: ");
    io::stdout().flush().unwrap();
    let mut uuid = String::new();
    io::stdin().read_line(&mut uuid).unwrap();
    let uuid = uuid.trim().to_string();

    if uuid.is_empty() {
        println!("❌ UUID 不能为空！");
        return String::new();
    }

    uuid
}

fn show_help() {
    println!();
    println!("📖 === 使用帮助 ===");
    println!();
    println!("🎯 程序功能：");
    println!("   本工具用于生成和验证 RustDesk 的自定义 ID，让您可以使用");
    println!("   容易记忆的 ID 来代替随机生成的数字 ID。");
    println!();
    println!("🔧 命令行用法：");
    println!("   生成加密 ID: cargo run -- --id <自定义ID> --uuid <机器UUID>");
    println!("   验证加密 ID: cargo run -- --eid <加密ID> --uuid <机器UUID>");
    println!();
    println!("📁 配置文件位置：");
    println!("   macOS: ~/Library/Preferences/com.carriez.RustDesk/RustDesk.toml");
    println!("   Windows: C:\\Users\\用户名\\AppData\\Roaming\\RustDesk\\config\\RustDesk.toml");
    println!();
    println!("⚠️  注意事项：");
    println!("   1. UUID 必须与运行 RustDesk 的机器匹配");
    println!("   2. 自定义 ID 不宜过短，避免与其他用户冲突");
    println!("   3. 修改配置文件后需要重启 RustDesk 服务");
    println!();
    println!("📞 获取帮助：");
    println!("   GitHub: https://github.com/Jxpro/custom-rustdesk");
    println!("   Email: jxpro@qq.com");
    println!();
}
