/* 
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * OpenAPI spec version: 1.17
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// GetCorporationsCorporationIdContractsContractIdItemsError520 : Error 520

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetCorporationsCorporationIdContractsContractIdItemsError520 {
  /// Error 520 message
  #[serde(rename = "error")]
  error: Option<String>
}

impl GetCorporationsCorporationIdContractsContractIdItemsError520 {
  /// Error 520
  pub fn new() -> GetCorporationsCorporationIdContractsContractIdItemsError520 {
    GetCorporationsCorporationIdContractsContractIdItemsError520 {
      error: None
    }
  }

  pub fn set_error(&mut self, error: String) {
    self.error = Some(error);
  }

  pub fn with_error(mut self, error: String) -> GetCorporationsCorporationIdContractsContractIdItemsError520 {
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


