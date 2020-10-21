use super::{cmd, cmdp};
use futures::FutureExt;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use crate::error::ZeroFrameError as Error;
use crate::responses::{ZeroResponse, AnnouncerInfo, FileRules, SiteInfo, ServerInfo};
use serde::{Serialize, de::DeserializeOwned};

pub async fn announcer_info() -> Result<AnnouncerInfo, Error> {
  let response = cmdp("announcerInfo", vec![]).await;
  response.response::<AnnouncerInfo>()
}

/// Request to add a new certificate to the current user.
/// Returns Ok(true) if the certificate was added and Ok(false) if it already existed prior.
pub async fn cert_add(
  domain: &str,
  auth_type: &str,
  auth_user_name: &str,
  cert: &str,
) -> Result<bool, Error> {
  let response = cmdp(
    "certAdd",
    vec![
      JsValue::from_str(domain),
      JsValue::from_str(auth_type),
      JsValue::from_str(auth_user_name),
      JsValue::from_str(cert),
    ],
  ).await;
  response.result_changed()
}

pub fn cert_select(accepted_domains: Vec<String>, accept_any: bool, accepted_pattern: &str) {
  let params = vec![
    JsValue::from_serde(&accepted_domains).unwrap(),
    JsValue::from_bool(accept_any),
    JsValue::from_str(accepted_pattern),
  ];
  cmd("certSelect", params)
}

pub fn channel_join(channel: &str) {
  cmd("channelJoin", vec![JsValue::from_str(channel)])
}

pub async fn db_query<T: DeserializeOwned>(query: &str, params: HashMap<String, String>) -> Result<Vec<T>, Error> {
  let response = cmdp(
    "dbQuery",
    vec![
      JsValue::from_str(query),
      JsValue::from_serde(&params).unwrap(),
    ],
  ).await;
  response.response::<Vec<T>>()
}

pub async fn dir_list(inner_path: &str) -> Result<Vec<String>, Error> {
  let response = cmdp("dirList", vec![JsValue::from_str(inner_path)]).await;
  response.response::<Vec<String>>()
}

pub async fn file_delete(inner_path: &str) -> Result<(), Error> {
  let response = cmdp("fileDelete", vec![JsValue::from_str(inner_path)]).await;
  response.result()
}

// TODO: replace with file_get_string and file_get_bytes
async fn file_get(
  inner_path: String,
  required: bool,
  format: &str,
  timeout: Option<usize>,
) -> JsValue {
  cmdp(
    "fileGet",
    vec![
      JsValue::from_str(&inner_path),
      JsValue::from_bool(required),
      JsValue::from_str(format),
      JsValue::from_f64(timeout.unwrap_or(0) as f64),
    ],
  ).await
}

// TODO: return result
pub async fn file_get_string(
  inner_path: String,
  required: bool,
  timeout: Option<usize>,
) -> Option<String> {
  file_get(inner_path, required, "text", timeout)
    .map(|js_value| js_value.as_string())
    .await
}

// TODO: return result
pub async fn file_get_bytes(
  inner_path: String,
  required: bool,
  timeout: Option<usize>,
) -> Option<Vec<u8>> {
  file_get(inner_path, required, "base64", timeout)
    .map(|js_value| {
      js_value
        .as_string()
        .map(|js_string| base64::decode(&js_string).unwrap())
    }).await
}

/// Recursively list of files in a directory
pub async fn file_list(inner_path: &str) -> Result<Vec<String>, Error> {
  let response = cmdp("fileList", vec![JsValue::from_str(inner_path)]).await;
  response.response::<Vec<String>>()
}

/// Initialize download of an (optional) file
pub async fn file_need(inner_path: &str, timeout: usize) -> Result<(), Error> {
  let response = cmdp(
    "fileNeed",
    vec![
      JsValue::from_str(inner_path),
      JsValue::from_f64(timeout as f64),
    ],
  ).await;
  response.result()
}

/// Simple json file query command
pub async fn file_query<T: DeserializeOwned>(dir_inner_path: &str, query: Option<String>) -> Result<Vec<T>, Error> {
  let mut params = vec![JsValue::from_str(dir_inner_path)];
  if let Some(query) = query {
    params.push(JsValue::from_str(&query));
  }
  let response = cmdp("fileQuery", params).await;
  response.response::<Vec<T>>()
}

pub async fn file_rules(inner_path: &str) -> Result<FileRules, Error> {
  let response = cmdp("fileRules", vec![JsValue::from_str(inner_path)]).await;
  response.response::<FileRules>()
}

pub async fn file_write(inner_path: &str, content_base64: &str) -> Result<(), Error>  {
  let response = cmdp(
    "fileWrite",
    vec![
      JsValue::from_str(inner_path),
      JsValue::from_str(content_base64),
    ],
  ).await;
  response.result()
}

/// Returns Ok(()) if ZeroNet returns pong
pub async fn ping() -> Result<(), Error> {
  let result = cmdp("ping", vec![]).await;
  if let Some(result) = result.as_string() {
    if result == "pong".to_string() {
      return Ok(())
    }
  }

  Err(Error::InvalidResponse)
}

/// Get information about the server
pub async fn server_info() -> Result<ServerInfo, Error> {
  let response = cmdp("serverInfo", vec![]).await;
  response.response::<ServerInfo>()
}

/// Get information about the site
pub async fn site_info() -> Result<SiteInfo, Error> {
  let response = cmdp("siteInfo", vec![]).await;
  response.response::<SiteInfo>()
}

///
pub async fn site_publish(
  privatekey: Option<String>,
  inner_path: Option<String>,
  sign: bool,
) -> Result<(), Error> {
  let response = cmdp(
    "sitePublish",
    vec![
      JsValue::from_str(&privatekey.unwrap_or("stored".to_string())),
      JsValue::from_str(&inner_path.unwrap_or("content.json".to_string())),
      JsValue::from_bool(sign),
    ],
  ).await;
  response.result()
}

pub async fn site_reload() -> Result<(), Error> {
  let response = cmdp("siteReload", vec![]).await;
  response.result()
}

pub async fn site_sign(
  privatekey: Option<String>,
  inner_path: Option<String>,
  remove_missing_optional: bool,
) -> Result<(), Error> {
  let response = cmdp(
    "siteSign",
    vec![
      JsValue::from(privatekey.unwrap_or("stored".to_string())),
      JsValue::from(inner_path.unwrap_or("content.json".to_string())),
      JsValue::from_bool(remove_missing_optional),
    ],
  ).await;
  response.result()
}

pub fn site_update(address: Option<String>) {
  let params = match address {
    Some(address) => vec![JsValue::from_str(&address)],
    None => vec![],
  };
  cmd("siteUpdate", params)
}

/// Get the user specific settings for this site
pub async fn user_get_settings<T: DeserializeOwned>() -> Result<T, Error> {
  let response = cmdp("userGetSettings", vec![]).await;
  response.response::<T>()
}

/// Set the user specific settings for this site
pub async fn user_set_settings<T: Serialize>(settings: T) -> Result<(), Error> {
  let settings = JsValue::from_serde(&settings)?;
  let response = cmdp("userSetSettings", vec![settings]).await;
  response.result()
}
