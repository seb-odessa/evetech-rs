/* 
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * OpenAPI spec version: 1.17
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// GetUniverseStargatesStargateIdDestination : destination object

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetUniverseStargatesStargateIdDestination {
  /// The stargate this stargate connects to
  #[serde(rename = "stargate_id")]
  stargate_id: i32,
  /// The solar system this stargate connects to
  #[serde(rename = "system_id")]
  system_id: i32
}

impl GetUniverseStargatesStargateIdDestination {
  /// destination object
  pub fn new(stargate_id: i32, system_id: i32) -> GetUniverseStargatesStargateIdDestination {
    GetUniverseStargatesStargateIdDestination {
      stargate_id: stargate_id,
      system_id: system_id
    }
  }

  pub fn set_stargate_id(&mut self, stargate_id: i32) {
    self.stargate_id = stargate_id;
  }

  pub fn with_stargate_id(mut self, stargate_id: i32) -> GetUniverseStargatesStargateIdDestination {
    self.stargate_id = stargate_id;
    self
  }

  pub fn stargate_id(&self) -> &i32 {
    &self.stargate_id
  }


  pub fn set_system_id(&mut self, system_id: i32) {
    self.system_id = system_id;
  }

  pub fn with_system_id(mut self, system_id: i32) -> GetUniverseStargatesStargateIdDestination {
    self.system_id = system_id;
    self
  }

  pub fn system_id(&self) -> &i32 {
    &self.system_id
  }


}



