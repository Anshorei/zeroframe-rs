use wasm_bindgen::prelude::*;
use wasm_bindgen::{closure::Closure, JsValue};

pub mod admin;
pub mod bigfile;
pub mod chart;
pub mod cors;
pub mod crypt_message;
pub mod error;
pub mod merger_site;
pub mod multiuser;
pub mod mute;
pub mod newsfeed;
pub mod optional_manager;
pub mod responses;
pub mod ui_server;
pub mod wrapper;

pub use error::ZeroFrameError;
pub use wrapper::{NotificationType, PromptType};

#[wasm_bindgen(module = "/js/zeroframe.js")]
extern "C" {
  fn cmd(cmd: &str, params: Vec<JsValue>);
  async fn cmdp(cmd: &str, params: Vec<JsValue>) -> JsValue;
  fn on_request(cmd: &str, handler: &Closure<dyn Fn(String, JsValue)>);
}
