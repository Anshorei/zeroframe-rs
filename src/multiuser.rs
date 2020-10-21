use super::cmd;

/// Request to login with a private key
pub fn user_login_form() {
  cmd("userLoginForm", vec![])
}

/// Request to show the user's private key
pub fn user_show_master_seed() {
  cmd("userShowMasterSeed", vec![])
}
