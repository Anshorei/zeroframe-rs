use wasm_bindgen::prelude::*;

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
pub mod ui_server;
pub mod wrapper;
pub mod responses;

pub use wrapper::{NotificationType, PromptType};
pub use error::ZeroFrameError;

#[wasm_bindgen(module = "/js/zeroframe.js")]
extern "C" {
  fn cmd(cmd: &str, params: Vec<JsValue>);
  async fn cmdp(cmd: &str, params: Vec<JsValue>) -> JsValue;
}
