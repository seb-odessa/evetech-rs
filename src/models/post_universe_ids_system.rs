/* 
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * OpenAPI spec version: 1.17
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// PostUniverseIdsSystem : system object

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct PostUniverseIdsSystem {
  /// id integer
  #[serde(rename = "id")]
  id: Option<i32>,
  /// name string
  #[serde(rename = "name")]
  name: Option<String>
}

impl PostUniverseIdsSystem {
  /// system object
  pub fn new() -> PostUniverseIdsSystem {
    PostUniverseIdsSystem {
      id: None,
      name: None
    }
  }

  pub fn set_id(&mut self, id: i32) {
    self.id = Some(id);
  }

  pub fn with_id(mut self, id: i32) -> PostUniverseIdsSystem {
    self.id = Some(id);
    self
  }

  pub fn id(&self) -> Option<&i32> {
    self.id.as_ref()
  }

  pub fn reset_id(&mut self) {
    self.id = None;
  }

  pub fn set_name(&mut self, name: String) {
    self.name = Some(name);
  }

  pub fn with_name(mut self, name: String) -> PostUniverseIdsSystem {
    self.name = Some(name);
    self
  }

  pub fn name(&self) -> Option<&String> {
    self.name.as_ref()
  }

  pub fn reset_name(&mut self) {
    self.name = None;
  }

}



