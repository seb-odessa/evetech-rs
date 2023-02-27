/* 
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * OpenAPI spec version: 1.17
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// GetCharactersCharacterIdPlanetsPlanetIdNotFound : Colony not found

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetCharactersCharacterIdPlanetsPlanetIdNotFound {
  /// error message
  #[serde(rename = "error")]
  error: Option<String>
}

impl GetCharactersCharacterIdPlanetsPlanetIdNotFound {
  /// Colony not found
  pub fn new() -> GetCharactersCharacterIdPlanetsPlanetIdNotFound {
    GetCharactersCharacterIdPlanetsPlanetIdNotFound {
      error: None
    }
  }

  pub fn set_error(&mut self, error: String) {
    self.error = Some(error);
  }

  pub fn with_error(mut self, error: String) -> GetCharactersCharacterIdPlanetsPlanetIdNotFound {
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



