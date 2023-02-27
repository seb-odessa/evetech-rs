/* 
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * OpenAPI spec version: 1.17
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// GetMarketsRegionIdOrdersUnprocessableEntity : Unprocessable entity

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetMarketsRegionIdOrdersUnprocessableEntity {
  /// Unprocessable entity message
  #[serde(rename = "error")]
  error: Option<String>
}

impl GetMarketsRegionIdOrdersUnprocessableEntity {
  /// Unprocessable entity
  pub fn new() -> GetMarketsRegionIdOrdersUnprocessableEntity {
    GetMarketsRegionIdOrdersUnprocessableEntity {
      error: None
    }
  }

  pub fn set_error(&mut self, error: String) {
    self.error = Some(error);
  }

  pub fn with_error(mut self, error: String) -> GetMarketsRegionIdOrdersUnprocessableEntity {
    self.error = Some(error);
    self
  }

  pub fn error(&self) -> Option<&String> {
    self.error.as_ref()
  }

  pub fn reset_error(&mut self) {
    self.error = None;
  }

}



