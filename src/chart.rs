use super::{cmdp};
use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use serde::de::DeserializeOwned;
use crate::ZeroFrameError as Error;
use crate::responses::{PeerLocation, ZeroResponse};

pub async fn db_query<T: DeserializeOwned>(query: &str, params: HashMap<String, String>) -> Result<Vec<T>, Error> {
  let response = cmdp(
    "chartDbQuery",
    vec![
      JsValue::from_str(query),
      JsValue::from_serde(&params).unwrap(),
    ],
  ).await;
  response.response::<Vec<T>>()
}

pub async fn get_peer_locations() -> Result<Vec<PeerLocation>, Error> {
  let response = cmdp("chartGetPeerLocations", vec![]).await;
  response.response::<Vec<PeerLocation>>()
}
