use super::cmdp;
use wasm_bindgen::prelude::*;

pub async fn user_publickey(index: Option<usize>) -> JsValue {
  let params = match index {
    Some(index) => vec![JsValue::from_f64(index as f64)],
    None => vec![],
  };
  cmdp("userPublickey", params).await
}

pub async fn ecies_encrypt(text: &str, publickey_index: usize, return_aes_key: bool) -> JsValue {
  cmdp(
    "eciesEncrypt",
    vec![
      JsValue::from_str(text),
      JsValue::from_f64(publickey_index as f64),
      JsValue::from_bool(return_aes_key),
    ],
  )
  .await
}

pub async fn ecies_decrypt(params: &str, privatekey_index: usize) -> JsValue {
  cmdp(
    "eciesDecrypt",
    vec![
      JsValue::from_str(params),
      JsValue::from_f64(privatekey_index as f64),
    ],
  )
  .await
}

pub async fn ecies_decrypt_multiple() {
  unimplemented!()
}

pub async fn aes_encrypt(text: &str, key: Option<String>, iv: Option<String>) -> JsValue {
  cmdp(
    "aesEncrypt",
    vec![
      JsValue::from_str(text),
      JsValue::from_str(&key.unwrap_or("generate new".to_string())),
      JsValue::from_str(&iv.unwrap_or("generate new".to_string())),
    ],
  )
  .await
}

pub async fn aes_decrypt(iv: &str, encrypted_text: &str, key: &str) -> JsValue {
  cmdp(
    "aesDecrypt",
    vec![
      JsValue::from_str(iv),
      JsValue::from_str(encrypted_text),
      JsValue::null(),
      JsValue::from_str(key),
    ],
  )
  .await
}

pub async fn aes_decrypt_multiple() {
  unimplemented!()
  // TODO:
  // cmdp("aesDecrypt", vec![
  //   JsValue::from_str(iv),
  //   JsValue::null(),
  // ]).await
}
