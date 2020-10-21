use super::{cmd, cmdp};
use wasm_bindgen::prelude::*;

pub async fn optional_file_list(
  address: Option<String>,
  orderby: Option<String>,
  limit: usize,
) -> JsValue {
  cmdp(
    "optionalFileList",
    vec![
      JsValue::from_str(&address.unwrap_or("current site".to_string())),
      JsValue::from_str(&orderby.unwrap_or("time_downloaded DESC".to_string())),
      JsValue::from_f64(limit as f64),
    ],
  )
  .await
}

pub async fn optional_file_info(inner_path: &str) -> JsValue {
  cmdp("optionalFileInfo", vec![JsValue::from_str(inner_path)]).await
}

pub fn optional_file_pin(inner_path: &str, address: Option<String>) {
  let mut params = vec![JsValue::from_str(inner_path)];
  if let Some(address) = address {
    params.push(JsValue::from_str(&address));
  }
  cmd("optionalFilePin", params)
}

pub fn optional_file_unpin(inner_path: &str, address: Option<String>) {
  let mut params = vec![JsValue::from_str(inner_path)];
  if let Some(address) = address {
    params.push(JsValue::from_str(&address));
  }
  cmd("optionalFileUnpin", params)
}

pub async fn optional_file_delete() -> JsValue {
  cmdp("optionalLimitStats", vec![]).await
}

pub fn optional_limit_set(limit: f64) {
  cmd("optionalLimitSet", vec![JsValue::from_f64(limit)])
}

pub async fn optional_help_list(address: &str) -> JsValue {
  cmdp("optionalHelpList", vec![JsValue::from_str(address)]).await
}

pub fn optional_help(directory: &str, title: &str, address: Option<String>) {
  let mut params = vec![JsValue::from_str(directory), JsValue::from_str(title)];
  if let Some(address) = address {
    params.push(JsValue::from(&address));
  }
  cmd("optionalHelp", params)
}

pub fn optional_help_remove(directory: &str, address: Option<String>) {
  let mut params = vec![JsValue::from_str(directory)];
  if let Some(address) = address {
    params.push(JsValue::from_str(&address));
  }
  cmd("optionalHelpRemove", params)
}

pub fn optional_help_all(value: bool, address: Option<String>) {
  let mut params = vec![JsValue::from_bool(value)];
  if let Some(address) = address {
    params.push(JsValue::from_str(&address));
  }
  cmd("optionalHelpAll", params)
}
