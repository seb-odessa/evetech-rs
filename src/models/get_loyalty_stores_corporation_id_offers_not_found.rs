/* 
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * OpenAPI spec version: 1.17
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// GetLoyaltyStoresCorporationIdOffersNotFound : Not found

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetLoyaltyStoresCorporationIdOffersNotFound {
  /// Not found message
  #[serde(rename = "error")]
  error: Option<String>
}

impl GetLoyaltyStoresCorporationIdOffersNotFound {
  /// Not found
  pub fn new() -> GetLoyaltyStoresCorporationIdOffersNotFound {
    GetLoyaltyStoresCorporationIdOffersNotFound {
      error: None
    }
  }

  pub fn set_error(&mut self, error: String) {
    self.error = Some(error);
  }

  pub fn with_error(mut self, error: String) -> GetLoyaltyStoresCorporationIdOffersNotFound {
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



