extern crate clap;
extern crate sodiumoxide;
use clap::load_yaml;
use clap::App;
use sodiumoxide::base64;
use sodiumoxide::crypto::secretbox;
use std::io::{self, Write};

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let has_id = matches.is_present("id");
    let has_eid = matches.is_present("eid");
    let uuid_option = matches.value_of("uuid");

    // 如果没有提供任何参数，显示交互式界面
    if !has_id && !has_eid && uuid_option.is_none() {
        show_interactive_menu();
        return;
    }

    let uuid = uuid_option.unwrap();

    if !(has_id ^ has_eid) {
        println!("Please provide only one of id or eid, where id is the custom id to encrypt and eid is the encrypted id to decrypt");
        println!("For help use --help");
        return;
    }

    if has_id {
        let custom_id = matches.value_of("id").unwrap();
        match encrypt(custom_id.as_bytes(), uuid) {
            Ok(encrypted_string) => {
                println!("{:?} is encrypted to {:?}", custom_id, format!("00{}", encrypted_string));
                println!("Please replace the id with the enc_id field in the config file");
            }
            Err(_) => println!("Error occurred during encryption"),
        }
    } else if has_eid {
        let enc_id = matches.value_of("eid").unwrap();
        match decrypt(enc_id[2..].as_bytes(), uuid) {
            Ok(decrypted_bytes) => {
                println!("{:?} is decrypted to {:?}", enc_id, String::from_utf8(decrypted_bytes).unwrap());
                println!("Please compare the id with the enc_id field in the config file");
            }
            Err(_) => println!("Error occurred during decryption"),
        }
    }
}

fn show_interactive_menu() {
    // 显示 ASCII Logo
    println!(r#"
 ____            _   ____            _    
|  _ \ _   _ ___| |_|  _ \  ___  ___| | __
| |_) | | | / __| __| | | |/ _ \/ __| |/ /
|  _ <| |_| \__ \ |_| |_| |  __/\__ \   < 
|_| \_\\__,_|___/\__|____/ \___||___/_|\_\
                                        
   自定义 ID 工具 v0.2.0
"#);
    
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
    match decrypt(enc_id[2..].as_bytes(), &uuid) {
        Ok(decrypted_bytes) => {
            match String::from_utf8(decrypted_bytes) {
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
            }
        }
        Err(_) => {
            println!("❌ 解密失败，请检查加密 ID 和 UUID 是否正确");
        }
    }
}

fn get_uuid_input() -> String {
    println!();
    println!("📋 如何获取 UUID：");
    println!("   Windows: 注册表 HKEY_LOCAL_MACHINE\\SOFTWARE\\Microsoft\\Cryptography 中的 MachineGuid");
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

fn decrypt(v: &[u8], uuid: &str) -> Result<Vec<u8>, ()> {
    base64::decode(v, base64::Variant::Original)
        .and_then(|v: Vec<u8>| symmetric_crypt(&v, uuid, false))
}

fn encrypt(v: &[u8], uuid: &str) -> Result<String, ()> {
    symmetric_crypt(v, uuid, true).map(|v: Vec<u8>| base64::encode(v, base64::Variant::Original))
}

pub fn symmetric_crypt(data: &[u8], uuid: &str, encrypt: bool) -> Result<Vec<u8>, ()> {
    // 将字符串转换为 Vec<u8>
    let mut keybuf: Vec<u8> = uuid.into();
    // 调整 Vec 大小以适应密钥长度要求
    keybuf.resize(secretbox::KEYBYTES, 0);
    // 尝试将 Vec<u8> 转换为密钥结构，失败则返回错误
    let key = secretbox::Key(keybuf.try_into().map_err(|_| ())?);
    // 创建一个全0的 nonce
    let nonce = secretbox::Nonce([0; secretbox::NONCEBYTES]);
    // 根据 encrypt 参数选择加密或解密
    if encrypt {
        Ok(secretbox::seal(data, &nonce, &key))
    } else {
        secretbox::open(data, &nonce, &key)
    }
}
