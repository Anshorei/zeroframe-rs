use super::{cmd, cmdp};
use wasm_bindgen::prelude::*;

pub enum NotificationType {
  Error,
  Info,
  Done,
}

impl ToString for NotificationType {
  fn to_string(&self) -> String {
    match self {
      NotificationType::Error => String::from("error"),
      NotificationType::Info => String::from("info"),
      NotificationType::Done => String::from("done"),
    }
  }
}

impl Into<JsValue> for NotificationType {
  fn into(self) -> JsValue {
    JsValue::from(&self.to_string())
  }
}

pub fn notify_info(message: &str, timeout: Option<usize>) {
  notification(NotificationType::Info, message, timeout)
}
pub fn notify_error(message: &str, timeout: Option<usize>) {
  notification(NotificationType::Error, message, timeout)
}
pub fn notify_done(message: &str, timeout: Option<usize>) {
  notification(NotificationType::Done, message, timeout)
}

pub fn confirm() {
  unimplemented!()
}

pub fn inner_loaded() {
  cmd("wrapperInnerLoaded", vec![]);
}

pub async fn get_local_storage() -> JsValue {
  cmdp("wrapperGetLocalStorage", vec![]).await
}

pub async fn get_state() -> JsValue {
  cmdp("wrapperGetState", vec![]).await
}

pub async fn get_ajax_key() -> JsValue {
  cmdp("wrapperGetAjaxKey", vec![]).await
}

pub fn notification(notification_type: NotificationType, message: &str, timeout: Option<usize>) {
  let mut params = vec![notification_type.into(), JsValue::from_str(message)];
  if let Some(duration) = timeout {
    params.push(JsValue::from_f64(duration as f64))
  }
  cmd("wrapperNotification", params);
}

pub fn open_window(url: &str, target: Option<String>, specs: Option<String>) {
  let mut params = vec![JsValue::from_str(url)];
  if let Some(target) = target {
    params.push(JsValue::from_str(&target))
  }
  if let Some(specs) = specs {
    params.push(JsValue::from_str(&specs))
  }
  cmd("wrapperOpenWindow", params);
}

pub async fn permission_add(permission: &str) -> bool {
  let res = cmdp("wrapperPermissionAdd", vec![JsValue::from_str(permission)]).await;
  return res.as_string() == Some(String::from("ok"));
}

pub enum PromptType {
  Text,
  Password,
}

impl ToString for PromptType {
  fn to_string(&self) -> String {
    match self {
      PromptType::Text => String::from("text"),
      PromptType::Password => String::from("password"),
    }
  }
}

impl Into<JsValue> for PromptType {
  fn into(self) -> JsValue {
    JsValue::from_str(&self.to_string())
  }
}

pub async fn prompt(message: &str, prompt_type: PromptType) -> JsValue {
  cmdp(
    "wrapperPrompt",
    vec![JsValue::from_str(message), prompt_type.into()],
  )
  .await
}

pub fn push_state(state: JsValue, title: &str, url: &str) {
  cmd(
    "wrapperPushState",
    vec![state, JsValue::from_str(title), JsValue::from_str(url)],
  )
}

pub fn replace_state(state: JsValue, title: &str, url: &str) {
  cmd(
    "wrapperReplaceState",
    vec![state, JsValue::from_str(title), JsValue::from_str(url)],
  )
}

#[deprecated = "Starting from ZeroNet Rev3136 you can use the fullscreen javascript API directly, without needing to ask the wrapper first."]
pub fn request_fullscreen() {
  cmd("wrapperRequestFullscreen", vec![])
}

pub fn set_local_storage(data: JsValue) {
  cmd("wrapperSetLocalStorage", vec![data])
}

pub fn set_title(title: &str) {
  cmd("wrapperSetTitle", vec![JsValue::from_str(title)])
}

pub fn set_viewport(viewport: &str) {
  cmd("wrapperSetViewport", vec![JsValue::from_str(viewport)])
}
