use super::{cmdp};
use wasm_bindgen::prelude::*;

// TODO: rustify big file upload
pub async fn bigfile_upload_init(inner_path: &str, size: usize) -> JsValue {
  cmdp(
    "bigfileUploadInit",
    vec![
      JsValue::from_str(inner_path),
      JsValue::from_f64(size as f64),
    ],
  ).await
}
