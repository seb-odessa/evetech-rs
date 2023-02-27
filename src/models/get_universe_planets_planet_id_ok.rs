/* 
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * OpenAPI spec version: 1.17
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// GetUniversePlanetsPlanetIdOk : 200 ok object

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetUniversePlanetsPlanetIdOk {
  /// name string
  #[serde(rename = "name")]
  name: String,
  /// planet_id integer
  #[serde(rename = "planet_id")]
  planet_id: i32,
  #[serde(rename = "position")]
  position: ::models::GetUniversePlanetsPlanetIdPosition,
  /// The solar system this planet is in
  #[serde(rename = "system_id")]
  system_id: i32,
  /// type_id integer
  #[serde(rename = "type_id")]
  type_id: i32
}

impl GetUniversePlanetsPlanetIdOk {
  /// 200 ok object
  pub fn new(name: String, planet_id: i32, position: ::models::GetUniversePlanetsPlanetIdPosition, system_id: i32, type_id: i32) -> GetUniversePlanetsPlanetIdOk {
    GetUniversePlanetsPlanetIdOk {
      name: name,
      planet_id: planet_id,
      position: position,
      system_id: system_id,
      type_id: type_id
    }
  }

  pub fn set_name(&mut self, name: String) {
    self.name = name;
  }

  pub fn with_name(mut self, name: String) -> GetUniversePlanetsPlanetIdOk {
    self.name = name;
    self
  }

  pub fn name(&self) -> &String {
    &self.name
  }


  pub fn set_planet_id(&mut self, planet_id: i32) {
    self.planet_id = planet_id;
  }

  pub fn with_planet_id(mut self, planet_id: i32) -> GetUniversePlanetsPlanetIdOk {
    self.planet_id = planet_id;
    self
  }

  pub fn planet_id(&self) -> &i32 {
    &self.planet_id
  }


  pub fn set_position(&mut self, position: ::models::GetUniversePlanetsPlanetIdPosition) {
    self.position = position;
  }

  pub fn with_position(mut self, position: ::models::GetUniversePlanetsPlanetIdPosition) -> GetUniversePlanetsPlanetIdOk {
    self.position = position;
    self
  }

  pub fn position(&self) -> &::models::GetUniversePlanetsPlanetIdPosition {
    &self.position
  }


  pub fn set_system_id(&mut self, system_id: i32) {
    self.system_id = system_id;
  }

  pub fn with_system_id(mut self, system_id: i32) -> GetUniversePlanetsPlanetIdOk {
    self.system_id = system_id;
    self
  }

  pub fn system_id(&self) -> &i32 {
    &self.system_id
  }


  pub fn set_type_id(&mut self, type_id: i32) {
    self.type_id = type_id;
  }

  pub fn with_type_id(mut self, type_id: i32) -> GetUniversePlanetsPlanetIdOk {
    self.type_id = type_id;
    self
  }

  pub fn type_id(&self) -> &i32 {
    &self.type_id
  }


}


