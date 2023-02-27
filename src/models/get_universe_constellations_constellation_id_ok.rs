/* 
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * OpenAPI spec version: 1.17
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// GetUniverseConstellationsConstellationIdOk : 200 ok object

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetUniverseConstellationsConstellationIdOk {
  /// constellation_id integer
  #[serde(rename = "constellation_id")]
  constellation_id: i32,
  /// name string
  #[serde(rename = "name")]
  name: String,
  #[serde(rename = "position")]
  position: ::models::GetUniverseConstellationsConstellationIdPosition,
  /// The region this constellation is in
  #[serde(rename = "region_id")]
  region_id: i32,
  /// systems array
  #[serde(rename = "systems")]
  systems: Vec<i32>
}

impl GetUniverseConstellationsConstellationIdOk {
  /// 200 ok object
  pub fn new(constellation_id: i32, name: String, position: ::models::GetUniverseConstellationsConstellationIdPosition, region_id: i32, systems: Vec<i32>) -> GetUniverseConstellationsConstellationIdOk {
    GetUniverseConstellationsConstellationIdOk {
      constellation_id: constellation_id,
      name: name,
      position: position,
      region_id: region_id,
      systems: systems
    }
  }

  pub fn set_constellation_id(&mut self, constellation_id: i32) {
    self.constellation_id = constellation_id;
  }

  pub fn with_constellation_id(mut self, constellation_id: i32) -> GetUniverseConstellationsConstellationIdOk {
    self.constellation_id = constellation_id;
    self
  }

  pub fn constellation_id(&self) -> &i32 {
    &self.constellation_id
  }


  pub fn set_name(&mut self, name: String) {
    self.name = name;
  }

  pub fn with_name(mut self, name: String) -> GetUniverseConstellationsConstellationIdOk {
    self.name = name;
    self
  }

  pub fn name(&self) -> &String {
    &self.name
  }


  pub fn set_position(&mut self, position: ::models::GetUniverseConstellationsConstellationIdPosition) {
    self.position = position;
  }

  pub fn with_position(mut self, position: ::models::GetUniverseConstellationsConstellationIdPosition) -> GetUniverseConstellationsConstellationIdOk {
    self.position = position;
    self
  }

  pub fn position(&self) -> &::models::GetUniverseConstellationsConstellationIdPosition {
    &self.position
  }


  pub fn set_region_id(&mut self, region_id: i32) {
    self.region_id = region_id;
  }

  pub fn with_region_id(mut self, region_id: i32) -> GetUniverseConstellationsConstellationIdOk {
    self.region_id = region_id;
    self
  }

  pub fn region_id(&self) -> &i32 {
    &self.region_id
  }


  pub fn set_systems(&mut self, systems: Vec<i32>) {
    self.systems = systems;
  }

  pub fn with_systems(mut self, systems: Vec<i32>) -> GetUniverseConstellationsConstellationIdOk {
    self.systems = systems;
    self
  }

  pub fn systems(&self) -> &Vec<i32> {
    &self.systems
  }


}



