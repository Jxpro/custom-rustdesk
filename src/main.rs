extern crate clap;
extern crate sodiumoxide;
use clap::load_yaml;
use clap::App;
use sodiumoxide::base64;
use sodiumoxide::crypto::secretbox;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let has_id = matches.is_present("id");
    let has_eid = matches.is_present("eid");
    let uuid = matches.value_of("uuid").unwrap();

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
