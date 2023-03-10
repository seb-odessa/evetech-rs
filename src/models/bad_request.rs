/* 
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * OpenAPI spec version: 1.17
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// BadRequest : Bad request model

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct BadRequest {
  /// Bad request message
  #[serde(rename = "error")]
  error: String
}

impl BadRequest {
  /// Bad request model
  pub fn new(error: String) -> BadRequest {
    BadRequest {
      error: error
    }
  }

  pub fn set_error(&mut self, error: String) {
    self.error = error;
  }

  pub fn with_error(mut self, error: String) -> BadRequest {
    self.error = error;
    self
  }

  pub fn error(&self) -> &String {
    &self.error
  }


}



