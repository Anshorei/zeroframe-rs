use super::{cmd, cmdp};
use wasm_bindgen::prelude::*;
use crate::ZeroFrameError as Error;
use crate::responses::ZeroResponse;

/// Set followed SQL queries.
pub async fn feed_follow(query: &str) -> Result<(), Error> {
  let response = cmdp("feedFollow", vec![JsValue::from_str(query)]).await;
  response.result()
}

// TODO: no JsValue
/// Return currently followed feeds
pub async fn feed_list_follow() -> JsValue {
  cmdp("feedListFollow", vec![]).await
}

// TODO: find solution without JsValue
/// Execute all queries for followed sites in the user's notifications feed
pub async fn feed_query(limit: usize, day_limit: usize) -> JsValue {
  cmdp(
    "feedQuery",
    vec![
      JsValue::from_f64(limit as f64),
      JsValue::from_f64(day_limit as f64),
    ],
  ).await
}
