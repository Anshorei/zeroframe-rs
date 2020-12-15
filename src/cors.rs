use super::cmdp;
use crate::responses::ZeroResponse;
use crate::ZeroFrameError as Error;
use wasm_bindgen::prelude::*;

pub async fn cors_permission(address: &str) -> Result<(), Error> {
  let response = cmdp("corsPermission", vec![JsValue::from_str(address)]).await;
  response.result()
}
