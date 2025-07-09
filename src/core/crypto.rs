use sodiumoxide::base64;
use sodiumoxide::crypto::secretbox;

pub fn decrypt(v: &[u8], uuid: &str) -> Result<Vec<u8>, ()> {
    base64::decode(v, base64::Variant::Original)
        .and_then(|v: Vec<u8>| symmetric_crypt(&v, uuid, false))
}

pub fn encrypt(v: &[u8], uuid: &str) -> Result<String, ()> {
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
