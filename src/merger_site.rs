use super::{cmd, cmdp};
use wasm_bindgen::prelude::*;
use crate::ZeroFrameError as Error;
use crate::responses::{SiteInfo, ZeroResponse};

/// Start downloading new merger site(s)
pub fn merger_site_add(addresses: Vec<String>) {
  cmd(
    "mergerSiteAdd",
    vec![JsValue::from_serde(&addresses).unwrap()],
  );
}

/// Stop seeding and delete a merged site.
pub fn merger_site_delete(address: &str) {
  cmd("mergerSiteDelete", vec![JsValue::from_str(address)]);
}

/// Return merged sites
pub async fn merger_site_list() -> Result<Vec<String>, Error> {
  let response = cmdp("mergerSiteList", vec![JsValue::from_bool(false)]).await;
  response.response::<Vec<String>>()
}

pub async fn merger_site_info_list() -> Result<Vec<SiteInfo>, Error> {
  let response = cmdp("mergerSiteList", vec![JsValue::from_bool(true)]).await;
  response.response::<Vec<SiteInfo>>()
}
