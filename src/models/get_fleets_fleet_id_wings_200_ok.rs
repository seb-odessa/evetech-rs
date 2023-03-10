/* 
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * OpenAPI spec version: 1.17
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// GetFleetsFleetIdWings200Ok : 200 ok object

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetFleetsFleetIdWings200Ok {
  /// id integer
  #[serde(rename = "id")]
  id: i64,
  /// name string
  #[serde(rename = "name")]
  name: String,
  /// squads array
  #[serde(rename = "squads")]
  squads: Vec<::models::GetFleetsFleetIdWingsSquad>
}

impl GetFleetsFleetIdWings200Ok {
  /// 200 ok object
  pub fn new(id: i64, name: String, squads: Vec<::models::GetFleetsFleetIdWingsSquad>) -> GetFleetsFleetIdWings200Ok {
    GetFleetsFleetIdWings200Ok {
      id: id,
      name: name,
      squads: squads
    }
  }

  pub fn set_id(&mut self, id: i64) {
    self.id = id;
  }

  pub fn with_id(mut self, id: i64) -> GetFleetsFleetIdWings200Ok {
    self.id = id;
    self
  }

  pub fn id(&self) -> &i64 {
    &self.id
  }


  pub fn set_name(&mut self, name: String) {
    self.name = name;
  }

  pub fn with_name(mut self, name: String) -> GetFleetsFleetIdWings200Ok {
    self.name = name;
    self
  }

  pub fn name(&self) -> &String {
    &self.name
  }


  pub fn set_squads(&mut self, squads: Vec<::models::GetFleetsFleetIdWingsSquad>) {
    self.squads = squads;
  }

  pub fn with_squads(mut self, squads: Vec<::models::GetFleetsFleetIdWingsSquad>) -> GetFleetsFleetIdWings200Ok {
    self.squads = squads;
    self
  }

  pub fn squads(&self) -> &Vec<::models::GetFleetsFleetIdWingsSquad> {
    &self.squads
  }


}



