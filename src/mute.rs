use super::{cmd, cmdp};
use wasm_bindgen::prelude::*;
use crate::ZeroFrameError as Error;
use crate::responses::ZeroResponse;

pub async fn mute_add(auth_address: &str, cert_user_id: &str, reason: &str) -> Result<(), Error> {
  let response = cmdp(
    "muteAdd",
    vec![
      JsValue::from_str(auth_address),
      JsValue::from_str(cert_user_id),
      JsValue::from_str(reason),
    ],
  )
  .await;
  response.result()
}

pub async fn mute_remove(auth_address: &str) -> Result<(), Error> {
  let response = cmdp("muteRemove", vec![JsValue::from_str(auth_address)]).await;
  response.result()
}

pub async fn mute_list() -> Result<Vec<String>, Error> {
  let response = cmdp("muteList", vec![]).await;
  response.response::<Vec<String>>()
}
