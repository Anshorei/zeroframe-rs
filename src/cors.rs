use super::{cmdp};
use wasm_bindgen::prelude::*;
use crate::responses::ZeroResponse;
use crate::ZeroFrameError as Error;

pub async fn cors_permission(address: &str) -> Result<(), Error> {
  let response = cmdp("corsPermission", vec![JsValue::from_str(address)]).await;
  response.result()
}
