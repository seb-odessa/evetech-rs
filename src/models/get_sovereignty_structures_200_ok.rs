/* 
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * OpenAPI spec version: 1.17
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// GetSovereigntyStructures200Ok : 200 ok object

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetSovereigntyStructures200Ok {
  /// The alliance that owns the structure. 
  #[serde(rename = "alliance_id")]
  alliance_id: i32,
  /// Solar system in which the structure is located. 
  #[serde(rename = "solar_system_id")]
  solar_system_id: i32,
  /// Unique item ID for this structure.
  #[serde(rename = "structure_id")]
  structure_id: i64,
  /// A reference to the type of structure this is. 
  #[serde(rename = "structure_type_id")]
  structure_type_id: i32,
  /// The occupancy level for the next or current vulnerability window. This takes into account all development indexes and capital system bonuses. Also known as Activity Defense Multiplier from in the client. It increases the time that attackers must spend using their entosis links on the structure. 
  #[serde(rename = "vulnerability_occupancy_level")]
  vulnerability_occupancy_level: Option<f32>,
  /// The time at which the next or current vulnerability window ends. At the end of a vulnerability window the next window is recalculated and locked in along with the vulnerabilityOccupancyLevel. If the structure is not in 100% entosis control of the defender, it will go in to 'overtime' and stay vulnerable for as long as that situation persists. Only once the defenders have 100% entosis control and has the vulnerableEndTime passed does the vulnerability interval expire and a new one is calculated. 
  #[serde(rename = "vulnerable_end_time")]
  vulnerable_end_time: Option<String>,
  /// The next time at which the structure will become vulnerable. Or the start time of the current window if current time is between this and vulnerableEndTime. 
  #[serde(rename = "vulnerable_start_time")]
  vulnerable_start_time: Option<String>
}

impl GetSovereigntyStructures200Ok {
  /// 200 ok object
  pub fn new(alliance_id: i32, solar_system_id: i32, structure_id: i64, structure_type_id: i32) -> GetSovereigntyStructures200Ok {
    GetSovereigntyStructures200Ok {
      alliance_id: alliance_id,
      solar_system_id: solar_system_id,
      structure_id: structure_id,
      structure_type_id: structure_type_id,
      vulnerability_occupancy_level: None,
      vulnerable_end_time: None,
      vulnerable_start_time: None
    }
  }

  pub fn set_alliance_id(&mut self, alliance_id: i32) {
    self.alliance_id = alliance_id;
  }

  pub fn with_alliance_id(mut self, alliance_id: i32) -> GetSovereigntyStructures200Ok {
    self.alliance_id = alliance_id;
    self
  }

  pub fn alliance_id(&self) -> &i32 {
    &self.alliance_id
  }


  pub fn set_solar_system_id(&mut self, solar_system_id: i32) {
    self.solar_system_id = solar_system_id;
  }

  pub fn with_solar_system_id(mut self, solar_system_id: i32) -> GetSovereigntyStructures200Ok {
    self.solar_system_id = solar_system_id;
    self
  }

  pub fn solar_system_id(&self) -> &i32 {
    &self.solar_system_id
  }


  pub fn set_structure_id(&mut self, structure_id: i64) {
    self.structure_id = structure_id;
  }

  pub fn with_structure_id(mut self, structure_id: i64) -> GetSovereigntyStructures200Ok {
    self.structure_id = structure_id;
    self
  }

  pub fn structure_id(&self) -> &i64 {
    &self.structure_id
  }


  pub fn set_structure_type_id(&mut self, structure_type_id: i32) {
    self.structure_type_id = structure_type_id;
  }

  pub fn with_structure_type_id(mut self, structure_type_id: i32) -> GetSovereigntyStructures200Ok {
    self.structure_type_id = structure_type_id;
    self
  }

  pub fn structure_type_id(&self) -> &i32 {
    &self.structure_type_id
  }


  pub fn set_vulnerability_occupancy_level(&mut self, vulnerability_occupancy_level: f32) {
    self.vulnerability_occupancy_level = Some(vulnerability_occupancy_level);
  }

  pub fn with_vulnerability_occupancy_level(mut self, vulnerability_occupancy_level: f32) -> GetSovereigntyStructures200Ok {
    self.vulnerability_occupancy_level = Some(vulnerability_occupancy_level);
    self
  }

  pub fn vulnerability_occupancy_level(&self) -> Option<&f32> {
    self.vulnerability_occupancy_level.as_ref()
  }

  pub fn reset_vulnerability_occupancy_level(&mut self) {
    self.vulnerability_occupancy_level = None;
  }

  pub fn set_vulnerable_end_time(&mut self, vulnerable_end_time: String) {
    self.vulnerable_end_time = Some(vulnerable_end_time);
  }

  pub fn with_vulnerable_end_time(mut self, vulnerable_end_time: String) -> GetSovereigntyStructures200Ok {
    self.vulnerable_end_time = Some(vulnerable_end_time);
    self
  }

  pub fn vulnerable_end_time(&self) -> Option<&String> {
    self.vulnerable_end_time.as_ref()
  }

  pub fn reset_vulnerable_end_time(&mut self) {
    self.vulnerable_end_time = None;
  }

  pub fn set_vulnerable_start_time(&mut self, vulnerable_start_time: String) {
    self.vulnerable_start_time = Some(vulnerable_start_time);
  }

  pub fn with_vulnerable_start_time(mut self, vulnerable_start_time: String) -> GetSovereigntyStructures200Ok {
    self.vulnerable_start_time = Some(vulnerable_start_time);
    self
  }

  pub fn vulnerable_start_time(&self) -> Option<&String> {
    self.vulnerable_start_time.as_ref()
  }

  pub fn reset_vulnerable_start_time(&mut self) {
    self.vulnerable_start_time = None;
  }

}



