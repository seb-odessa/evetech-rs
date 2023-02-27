/* 
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * OpenAPI spec version: 1.17
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// GatewayTimeout : Gateway timeout model

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct GatewayTimeout {
  /// Gateway timeout message
  #[serde(rename = "error")]
  error: String,
  /// number of seconds the request was given
  #[serde(rename = "timeout")]
  timeout: Option<i32>
}

impl GatewayTimeout {
  /// Gateway timeout model
  pub fn new(error: String) -> GatewayTimeout {
    GatewayTimeout {
      error: error,
      timeout: None
    }
  }

  pub fn set_error(&mut self, error: String) {
    self.error = error;
  }

  pub fn with_error(mut self, error: String) -> GatewayTimeout {
    self.error = error;
    self
  }

  pub fn error(&self) -> &String {
    &self.error
  }


  pub fn set_timeout(&mut self, timeout: i32) {
    self.timeout = Some(timeout);
  }

  pub fn with_timeout(mut self, timeout: i32) -> GatewayTimeout {
    self.timeout = Some(timeout);
    self
  }

  pub fn timeout(&self) -> Option<&i32> {
    self.timeout.as_ref()
  }

  pub fn reset_timeout(&mut self) {
    self.timeout = None;
  }

}



