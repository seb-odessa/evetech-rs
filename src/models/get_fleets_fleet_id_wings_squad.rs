/* 
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * OpenAPI spec version: 1.17
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// GetFleetsFleetIdWingsSquad : squad object

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetFleetsFleetIdWingsSquad {
  /// id integer
  #[serde(rename = "id")]
  id: i64,
  /// name string
  #[serde(rename = "name")]
  name: String
}

impl GetFleetsFleetIdWingsSquad {
  /// squad object
  pub fn new(id: i64, name: String) -> GetFleetsFleetIdWingsSquad {
    GetFleetsFleetIdWingsSquad {
      id: id,
      name: name
    }
  }

  pub fn set_id(&mut self, id: i64) {
    self.id = id;
  }

  pub fn with_id(mut self, id: i64) -> GetFleetsFleetIdWingsSquad {
    self.id = id;
    self
  }

  pub fn id(&self) -> &i64 {
    &self.id
  }


  pub fn set_name(&mut self, name: String) {
    self.name = name;
  }

  pub fn with_name(mut self, name: String) -> GetFleetsFleetIdWingsSquad {
    self.name = name;
    self
  }

  pub fn name(&self) -> &String {
    &self.name
  }


}



