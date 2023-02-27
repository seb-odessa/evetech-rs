/* 
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * OpenAPI spec version: 1.17
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// GetUniverseFactions200Ok : 200 ok object

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetUniverseFactions200Ok {
  /// corporation_id integer
  #[serde(rename = "corporation_id")]
  corporation_id: Option<i32>,
  /// description string
  #[serde(rename = "description")]
  description: String,
  /// faction_id integer
  #[serde(rename = "faction_id")]
  faction_id: i32,
  /// is_unique boolean
  #[serde(rename = "is_unique")]
  is_unique: bool,
  /// militia_corporation_id integer
  #[serde(rename = "militia_corporation_id")]
  militia_corporation_id: Option<i32>,
  /// name string
  #[serde(rename = "name")]
  name: String,
  /// size_factor number
  #[serde(rename = "size_factor")]
  size_factor: f32,
  /// solar_system_id integer
  #[serde(rename = "solar_system_id")]
  solar_system_id: Option<i32>,
  /// station_count integer
  #[serde(rename = "station_count")]
  station_count: i32,
  /// station_system_count integer
  #[serde(rename = "station_system_count")]
  station_system_count: i32
}

impl GetUniverseFactions200Ok {
  /// 200 ok object
  pub fn new(description: String, faction_id: i32, is_unique: bool, name: String, size_factor: f32, station_count: i32, station_system_count: i32) -> GetUniverseFactions200Ok {
    GetUniverseFactions200Ok {
      corporation_id: None,
      description: description,
      faction_id: faction_id,
      is_unique: is_unique,
      militia_corporation_id: None,
      name: name,
      size_factor: size_factor,
      solar_system_id: None,
      station_count: station_count,
      station_system_count: station_system_count
    }
  }

  pub fn set_corporation_id(&mut self, corporation_id: i32) {
    self.corporation_id = Some(corporation_id);
  }

  pub fn with_corporation_id(mut self, corporation_id: i32) -> GetUniverseFactions200Ok {
    self.corporation_id = Some(corporation_id);
    self
  }

  pub fn corporation_id(&self) -> Option<&i32> {
    self.corporation_id.as_ref()
  }

  pub fn reset_corporation_id(&mut self) {
    self.corporation_id = None;
  }

  pub fn set_description(&mut self, description: String) {
    self.description = description;
  }

  pub fn with_description(mut self, description: String) -> GetUniverseFactions200Ok {
    self.description = description;
    self
  }

  pub fn description(&self) -> &String {
    &self.description
  }


  pub fn set_faction_id(&mut self, faction_id: i32) {
    self.faction_id = faction_id;
  }

  pub fn with_faction_id(mut self, faction_id: i32) -> GetUniverseFactions200Ok {
    self.faction_id = faction_id;
    self
  }

  pub fn faction_id(&self) -> &i32 {
    &self.faction_id
  }


  pub fn set_is_unique(&mut self, is_unique: bool) {
    self.is_unique = is_unique;
  }

  pub fn with_is_unique(mut self, is_unique: bool) -> GetUniverseFactions200Ok {
    self.is_unique = is_unique;
    self
  }

  pub fn is_unique(&self) -> &bool {
    &self.is_unique
  }


  pub fn set_militia_corporation_id(&mut self, militia_corporation_id: i32) {
    self.militia_corporation_id = Some(militia_corporation_id);
  }

  pub fn with_militia_corporation_id(mut self, militia_corporation_id: i32) -> GetUniverseFactions200Ok {
    self.militia_corporation_id = Some(militia_corporation_id);
    self
  }

  pub fn militia_corporation_id(&self) -> Option<&i32> {
    self.militia_corporation_id.as_ref()
  }

  pub fn reset_militia_corporation_id(&mut self) {
    self.militia_corporation_id = None;
  }

  pub fn set_name(&mut self, name: String) {
    self.name = name;
  }

  pub fn with_name(mut self, name: String) -> GetUniverseFactions200Ok {
    self.name = name;
    self
  }

  pub fn name(&self) -> &String {
    &self.name
  }


  pub fn set_size_factor(&mut self, size_factor: f32) {
    self.size_factor = size_factor;
  }

  pub fn with_size_factor(mut self, size_factor: f32) -> GetUniverseFactions200Ok {
    self.size_factor = size_factor;
    self
  }

  pub fn size_factor(&self) -> &f32 {
    &self.size_factor
  }


  pub fn set_solar_system_id(&mut self, solar_system_id: i32) {
    self.solar_system_id = Some(solar_system_id);
  }

  pub fn with_solar_system_id(mut self, solar_system_id: i32) -> GetUniverseFactions200Ok {
    self.solar_system_id = Some(solar_system_id);
    self
  }

  pub fn solar_system_id(&self) -> Option<&i32> {
    self.solar_system_id.as_ref()
  }

  pub fn reset_solar_system_id(&mut self) {
    self.solar_system_id = None;
  }

  pub fn set_station_count(&mut self, station_count: i32) {
    self.station_count = station_count;
  }

  pub fn with_station_count(mut self, station_count: i32) -> GetUniverseFactions200Ok {
    self.station_count = station_count;
    self
  }

  pub fn station_count(&self) -> &i32 {
    &self.station_count
  }


  pub fn set_station_system_count(&mut self, station_system_count: i32) {
    self.station_system_count = station_system_count;
  }

  pub fn with_station_system_count(mut self, station_system_count: i32) -> GetUniverseFactions200Ok {
    self.station_system_count = station_system_count;
    self
  }

  pub fn station_system_count(&self) -> &i32 {
    &self.station_system_count
  }


}



