use crate::error::ZeroFrameError as Error;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use wasm_bindgen::JsValue;

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
      } else if self.is_falsy() {
        return Err(Error::FalsyResponse);
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
        return Ok(false);
      } else if let Some(err) = Error::from_response(result) {
        return Err(err);
      }
    }
    Err(Error::InvalidResponse)
  }
  fn response<T: DeserializeOwned>(self) -> Result<T, Error> {
    match self.into_serde() {
      Ok(response) => return Ok(response),
      Err(err) => {
        if let Some(result) = self.as_string() {
          serde_json::from_str(&result).map_err(|_| match Error::from_response(result) {
            Some(err) => err,
            _ => Error::InvalidResponse,
          })
        } else {
          return Err(Error::from(err));
        }
      }
    }
  }
}

#[derive(Serialize, Deserialize)]
pub struct PeerLocation {
  pub lat:     f64,
  pub city:    String,
  pub ping:    Option<f64>,
  pub lon:     f64,
  pub country: String,
}

#[derive(Serialize, Deserialize)]
pub struct FileRules {
  pub current_size:  usize,
  pub cert_signers:  HashMap<String, Vec<String>>,
  pub files_allowed: String,
  pub signers:       Vec<String>,
  pub user_address:  String,
  pub max_size:      usize,
}

#[derive(Serialize, Deserialize)]
pub struct AnnouncerStats {
  pub status:          String,
  pub num_success:     usize,
  pub time_last_error: f64,
  pub time_status:     f64,
  pub num_request:     usize,
  pub time_request:    f64,
  pub num_error:       usize,
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
  pub debug:           bool,
  pub fileserver_ip:   String,
  pub fileserver_port: u16,
  pub ip_external:     bool,
  pub platform:        String,
  pub ui_ip:           String,
  pub ui_port:         u16,
  pub version:         String,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SiteSettings {
  pub added:               u64,
  pub ajax_key:            String,
  pub bytes_recv:          u64,
  pub bytes_sent:          u64,
  pub cache:               Value,
  pub downloaded:          Option<u64>,
  pub modified:            u64,
  pub optional_downloaded: u64,
  pub own:                 bool,
  pub peers:               u64,
  pub permissions:         Vec<String>,
  pub serving:             bool,
  pub size:                u64,
  pub size_files_optional: u64,
  pub size_optional:       u64,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SiteContentSummary {
  pub address: String,
  pub address_index: u64,
  #[serde(rename = "background-color")]
  pub background_color: String,
  pub clone_root: String,
  pub cloneable: bool,
  pub cloned_from: String,
  pub description: String,
  pub files: u64,
  pub files_optional: u64,
  pub ignore: String,
  pub includes: u64,
  pub inner_path: String,
  pub merged_type: String,
  pub modified: u64,
  pub optional: String,
  pub postmessage_nonce_security: bool,
  pub signs_required: u64,
  pub title: String,
  pub translate: Vec<String>,
  pub zeronet_version: String,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SiteInfo {
  pub address:          String,
  pub address_hash:     String,
  pub address_short:    String,
  pub tasks:            u64,
  pub size_limit:       u64,
  pub next_size_limit:  u64,
  pub auth_address:     Option<String>,
  pub auth_key_sha512:  String,
  pub peers:            u64,
  pub auth_key:         String,
  pub settings:         SiteSettings,
  pub bad_files:        u64,
  pub workers:          u64,
  pub content:          SiteContentSummary,
  pub cert_user_id:     Option<String>,
  pub started_task_num: u64,
  // pub content_updated: Option<bool>,
}
