use crate::responses::ErrorResponse;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ZeroFrameError {
  #[error("API call returned falsy response")]
  FalsyResponse,
  #[error("zeronet internal error")]
  RemoteError(String),
  #[error("could not parse response")]
  InvalidResponse,
  #[error("could not de/serialize object")]
  SerializationError(#[from] serde_json::Error),
}

impl ZeroFrameError {
  pub fn from_response(response: String) -> Option<Self> {
    let response: Result<ErrorResponse, _> = serde_json::from_str(&response);
    match response {
      Ok(resp) => Some(ZeroFrameError::RemoteError(resp.error)),
      Err(_) => None,
    }
  }
}
