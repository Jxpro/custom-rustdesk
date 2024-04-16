extern crate sodiumoxide;
use sodiumoxide::base64;
use sodiumoxide::crypto::secretbox;

fn main() {
    let enc_id: &str = "LTvZ+G4sqHwhzZcIwWUYlPUGs1bCf6EZ";
    let custom_id: &str = "jokerxin";

    match encrypt(custom_id.as_bytes()) {
        Ok(encrypted_string) => {
            println!("{:?} is encrypted to {:?}", custom_id, encrypted_string);
        }
        Err(_) => println!("Error occurred during encryption"),
    }

    match decrypt(enc_id.as_bytes()) {
        Ok(decrypted_bytes) => {
            let decrypted_string = String::from_utf8(decrypted_bytes).unwrap();
            println!("{:?} is decrypted to {:?}", enc_id, decrypted_string);
        }
        Err(_) => println!("Error occurred during decryption"),
    }
}

fn decrypt(v: &[u8]) -> Result<Vec<u8>, ()> {
    base64::decode(v, base64::Variant::Original).and_then(|v: Vec<u8>| symmetric_crypt(&v, false))
}

fn encrypt(v: &[u8]) -> Result<String, ()> {
    symmetric_crypt(v, true).map(|v: Vec<u8>| base64::encode(v, base64::Variant::Original))
}

pub fn symmetric_crypt(data: &[u8], encrypt: bool) -> Result<Vec<u8>, ()> {
    // (Windows) 来自注册表：HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Cryptography中的MachineGuid值
    // (MacOS) 来自注册表：HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Cryptography中的MachineGuid值
    let uuid: &str = "15b1cfe5-c36c-46f7-93fa-dde4fcccd80e";
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
