use super::{cmd, cmdp};
use crate::error::ZeroFrameError as Error;
use crate::responses::{AnnouncerInfo, FileRules, ServerInfo, SiteInfo, ZeroResponse};
use futures::FutureExt;
use serde::{de::DeserializeOwned, Serialize};
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

pub async fn announcer_info() -> Result<AnnouncerInfo, Error> {
  let response = cmdp("announcerInfo", vec![]).await;
  response.response::<AnnouncerInfo>()
}

/// Request to add a new certificate to the current user.
/// Returns Ok(true) if the certificate was added and Ok(false) if it already existed prior.
pub async fn cert_add<S: Into<String>>(
  domain: S,
  auth_type: S,
  auth_user_name: S,
  cert: S,
) -> Result<bool, Error> {
  let response = cmdp(
    "certAdd",
    vec![
      JsValue::from_str(&domain.into()),
      JsValue::from_str(&auth_type.into()),
      JsValue::from_str(&auth_user_name.into()),
      JsValue::from_str(&cert.into()),
    ],
  )
  .await;
  response.result_changed()
}

pub fn cert_select(accepted_domains: Vec<String>, accept_any: bool, accepted_pattern: String) {
  let params = vec![
    JsValue::from_serde(&accepted_domains).unwrap(),
    JsValue::from_bool(accept_any),
    JsValue::from_str(&accepted_pattern),
  ];
  cmd("certSelect", params)
}

pub fn channel_join(channel: String) {
  cmd("channelJoin", vec![JsValue::from_str(&channel)])
}

pub async fn db_query<T: DeserializeOwned>(
  query: String,
  params: HashMap<String, String>,
) -> Result<Vec<T>, Error> {
  let response = cmdp(
    "dbQuery",
    vec![
      JsValue::from_str(&query),
      JsValue::from_serde(&params).unwrap(),
    ],
  )
  .await;
  response.response::<Vec<T>>()
}

pub async fn dir_list<S: Into<String>>(inner_path: S) -> Result<Vec<String>, Error> {
  let response = cmdp("dirList", vec![JsValue::from_str(&inner_path.into())]).await;
  response.response::<Vec<String>>()
}

pub async fn file_delete<S: Into<String>>(inner_path: S) -> Result<(), Error> {
  let response = cmdp("fileDelete", vec![JsValue::from_str(&inner_path.into())]).await;
  response.result()
}

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
  )
  .await
}

// TODO: return result
pub async fn file_get_string<S: Into<String>>(
  inner_path: S,
  required: bool,
  timeout: Option<usize>,
) -> Option<String> {
  file_get(inner_path.into(), required, "text", timeout)
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
    })
    .await
}

/// Recursively list of files in a directory
pub async fn file_list(inner_path: String) -> Result<Vec<String>, Error> {
  let response = cmdp("fileList", vec![JsValue::from_str(&inner_path)]).await;
  response.response::<Vec<String>>()
}

/// Initialize download of an (optional) file
/// This function does nothing unless the future is resolved
/// This function may result in errors if timeout is 0
pub async fn file_need(inner_path: String, timeout: usize) -> Result<(), Error> {
  if timeout == 0 {
    return Err(Error::RemoteError("Timeout should not be 0".to_string()));
  }
  let response = cmdp(
    "fileNeed",
    vec![
      JsValue::from_str(&inner_path),
      JsValue::from_f64(timeout as f64),
    ],
  )
  .await;
  response.result()
}

/// Simple json file query command
pub async fn file_query<T: DeserializeOwned>(
  dir_inner_path: String,
  query: Option<String>,
) -> Result<Vec<T>, Error> {
  let mut params = vec![JsValue::from_str(&dir_inner_path)];
  if let Some(query) = query {
    params.push(JsValue::from_str(&query));
  }
  let response = cmdp("fileQuery", params).await;
  response.response::<Vec<T>>()
}

pub async fn file_rules(inner_path: String) -> Result<FileRules, Error> {
  let response = cmdp("fileRules", vec![JsValue::from_str(&inner_path)]).await;
  response.response::<FileRules>()
}

pub async fn file_write_bytes(inner_path: String, content: Vec<u8>) -> Result<(), Error> {
  let content_base64 = base64::encode(content);
  file_write(inner_path, content_base64).await
}

pub async fn file_write_string(inner_path: String, content: String) -> Result<(), Error> {
  file_write_bytes(inner_path, content.as_bytes().to_vec()).await
}

async fn file_write(inner_path: String, content_base64: String) -> Result<(), Error> {
  let response = cmdp(
    "fileWrite",
    vec![
      JsValue::from_str(&inner_path),
      JsValue::from_str(&content_base64),
    ],
  )
  .await;
  response.result()
}

/// Returns Ok(()) if ZeroNet returns pong
pub async fn ping() -> Result<(), Error> {
  let result = cmdp("ping", vec![]).await;
  if let Some(result) = result.as_string() {
    if result == "pong".to_string() {
      return Ok(());
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
  )
  .await;
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
  )
  .await;
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
