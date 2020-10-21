use serde::{Serialize, Deserialize, de::DeserializeOwned};
use crate::error::ZeroFrameError as Error;
use wasm_bindgen::JsValue;
use std::collections::HashMap;

pub trait ZeroResponse {
  fn result(self) -> Result<(), Error>;
  fn result_changed(self) -> Result<bool, Error>;
  fn response<T: DeserializeOwned>(self) -> Result<T, Error>;
}

impl ZeroResponse for JsValue {
  fn result(self) -> Result<(), Error> {
    if let Some(result) = self.as_string() {
      if result == "ok".to_string() {
        return Ok(());
      } else if let Some(err) = Error::from_response(result) {
        return Err(err);
      }
    }
    Err(Error::InvalidResponse)
  }
  fn result_changed(self) -> Result<bool, Error> {
    if let Some(result) = self.as_string() {
      if result == "ok".to_string() {
        return Ok(true);
      } else if result == "Not changed".to_string() {
        return Ok(false)
      } else if let Some(err) = Error::from_response(result) {
        return Err(err);
      }
    }
    Err(Error::InvalidResponse)
  }
  fn response<T: DeserializeOwned>(self) -> Result<T, Error> {
    if let Some(result) = self.as_string() {
      let response: Result<T, _> = serde_json::from_str(&result);
      return response
        .map_err(|_| match Error::from_response(result) {
          Some(err) => err,
          _ => Error::InvalidResponse,
        })
    }
    Err(Error::InvalidResponse)
  }
}

#[derive(Serialize, Deserialize)]
pub struct PeerLocation {
  pub lat: f64,
  pub city: String,
  pub ping: Option<f64>,
  pub lon: f64,
  pub country: String,
}

#[derive(Serialize, Deserialize)]
pub struct FileRules {
  pub current_size: usize,
  pub cert_signers: HashMap<String, Vec<String>>,
  pub files_allowed: String,
  pub signers: Vec<String>,
  pub user_address: String,
  pub max_size: usize,
}

#[derive(Serialize, Deserialize)]
pub struct AnnouncerStats {
  pub status: String,
  pub num_success: usize,
  pub time_last_error: f64,
  pub time_status: f64,
  pub num_request: usize,
  pub time_request: f64,
  pub num_error: usize,
}

#[derive(Serialize, Deserialize)]
pub struct AnnouncerInfo {
  pub stats: HashMap<String, AnnouncerStats>,
}

#[derive(Serialize, Deserialize)]
pub struct ErrorResponse {
  pub error: String,
}

#[derive(Serialize, Deserialize)]
pub struct ServerInfo {
  pub debug: bool,
  pub fileserver_ip: String,
  pub fileserver_port: u16,
  pub ip_external: bool,
  pub platform: String,
  pub ui_ip: String,
  pub ui_port: u16,
  pub version: String,
}

#[derive(Serialize, Deserialize)]
pub struct SiteSettings {
  pub peers: usize,
  pub serving: bool,
  pub modified: f64,
  pub own: bool,
  pub permissions: Vec<String>,
  pub size: usize,
}

#[derive(Serialize, Deserialize)]
pub struct SiteContentSummary{}

#[derive(Serialize, Deserialize)]
pub struct SiteInfo {
  pub tasks: usize,
  pub size_limit: usize,
  pub address: String,
  pub next_size_limit: usize,
  pub auth_address: String,
  pub auth_key_sha512: String,
  pub peers: usize,
  pub auth_key: String,
  pub settings: SiteSettings,
  pub bad_files: usize,
  pub workers: usize,
  pub content: SiteContentSummary,
  pub cert_user_id: String,
  pub started_task_num: usize,
  pub content_updated: f64,
}
