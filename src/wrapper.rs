use super::{cmd, cmdp, on_request};
use crate::ZeroFrameError as Error;
use wasm_bindgen::prelude::*;
use wasm_bindgen::{closure::Closure};

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

pub fn notify_info<S: ToString>(message: S, timeout: Option<usize>) {
  notification(NotificationType::Info, &message.to_string(), timeout)
}
pub fn notify_error<S: ToString>(message: S, timeout: Option<usize>) {
  notification(NotificationType::Error, &message.to_string(), timeout)
}
pub fn notify_done<S: ToString>(message: S, timeout: Option<usize>) {
  notification(NotificationType::Done, &message.to_string(), timeout)
}

pub async fn confirm<S1: ToString, S2: ToString>(message: S1, button: S2) -> Result<bool, Error> {
  let mut params = vec![];
  params.push(JsValue::from_str(&message.to_string()));
  params.push(JsValue::from_str(&button.to_string()));
  let result = cmdp("wrapperConfirm", params).await;
  match result.as_f64() {
    Some(i) => Ok(i == 1.0),
    None => Err(Error::InvalidResponse),
  }
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

pub fn set_title<S: ToString>(title: S) {
  cmd("wrapperSetTitle", vec![JsValue::from_str(&title.to_string())])
}

pub fn set_viewport(viewport: &str) {
  cmd("wrapperSetViewport", vec![JsValue::from_str(viewport)])
}

pub fn add_request_handler<F: Fn(String, JsValue) + 'static>(cmd: &str, handler: F) {
  let handler = Box::new(handler) as Box<dyn Fn(_, _)>;
  // let handler = Box::new(move |a, b| handler(a, b)) as Box<dyn Fn(_, _)>;
  let closure = Closure::wrap(handler);
  on_request(cmd, &closure);
  closure.forget();
}
