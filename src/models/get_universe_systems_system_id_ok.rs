/* 
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * OpenAPI spec version: 1.17
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// GetUniverseSystemsSystemIdOk : 200 ok object

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetUniverseSystemsSystemIdOk {
  /// The constellation this solar system is in
  #[serde(rename = "constellation_id")]
  constellation_id: i32,
  /// name string
  #[serde(rename = "name")]
  name: String,
  /// planets array
  #[serde(rename = "planets")]
  planets: Option<Vec<::models::GetUniverseSystemsSystemIdPlanet>>,
  #[serde(rename = "position")]
  position: ::models::GetUniverseSystemsSystemIdPosition,
  /// security_class string
  #[serde(rename = "security_class")]
  security_class: Option<String>,
  /// security_status number
  #[serde(rename = "security_status")]
  security_status: f32,
  /// star_id integer
  #[serde(rename = "star_id")]
  star_id: Option<i32>,
  /// stargates array
  #[serde(rename = "stargates")]
  stargates: Option<Vec<i32>>,
  /// stations array
  #[serde(rename = "stations")]
  stations: Option<Vec<i32>>,
  /// system_id integer
  #[serde(rename = "system_id")]
  system_id: i32
}

impl GetUniverseSystemsSystemIdOk {
  /// 200 ok object
  pub fn new(constellation_id: i32, name: String, position: ::models::GetUniverseSystemsSystemIdPosition, security_status: f32, system_id: i32) -> GetUniverseSystemsSystemIdOk {
    GetUniverseSystemsSystemIdOk {
      constellation_id: constellation_id,
      name: name,
      planets: None,
      position: position,
      security_class: None,
      security_status: security_status,
      star_id: None,
      stargates: None,
      stations: None,
      system_id: system_id
    }
  }

  pub fn set_constellation_id(&mut self, constellation_id: i32) {
    self.constellation_id = constellation_id;
  }

  pub fn with_constellation_id(mut self, constellation_id: i32) -> GetUniverseSystemsSystemIdOk {
    self.constellation_id = constellation_id;
    self
  }

  pub fn constellation_id(&self) -> &i32 {
    &self.constellation_id
  }


  pub fn set_name(&mut self, name: String) {
    self.name = name;
  }

  pub fn with_name(mut self, name: String) -> GetUniverseSystemsSystemIdOk {
    self.name = name;
    self
  }

  pub fn name(&self) -> &String {
    &self.name
  }


  pub fn set_planets(&mut self, planets: Vec<::models::GetUniverseSystemsSystemIdPlanet>) {
    self.planets = Some(planets);
  }

  pub fn with_planets(mut self, planets: Vec<::models::GetUniverseSystemsSystemIdPlanet>) -> GetUniverseSystemsSystemIdOk {
    self.planets = Some(planets);
    self
  }

  pub fn planets(&self) -> Option<&Vec<::models::GetUniverseSystemsSystemIdPlanet>> {
    self.planets.as_ref()
  }

  pub fn reset_planets(&mut self) {
    self.planets = None;
  }

  pub fn set_position(&mut self, position: ::models::GetUniverseSystemsSystemIdPosition) {
    self.position = position;
  }

  pub fn with_position(mut self, position: ::models::GetUniverseSystemsSystemIdPosition) -> GetUniverseSystemsSystemIdOk {
    self.position = position;
    self
  }

  pub fn position(&self) -> &::models::GetUniverseSystemsSystemIdPosition {
    &self.position
  }


  pub fn set_security_class(&mut self, security_class: String) {
    self.security_class = Some(security_class);
  }

  pub fn with_security_class(mut self, security_class: String) -> GetUniverseSystemsSystemIdOk {
    self.security_class = Some(security_class);
    self
  }

  pub fn security_class(&self) -> Option<&String> {
    self.security_class.as_ref()
  }

  pub fn reset_security_class(&mut self) {
    self.security_class = None;
  }

  pub fn set_security_status(&mut self, security_status: f32) {
    self.security_status = security_status;
  }

  pub fn with_security_status(mut self, security_status: f32) -> GetUniverseSystemsSystemIdOk {
    self.security_status = security_status;
    self
  }

  pub fn security_status(&self) -> &f32 {
    &self.security_status
  }


  pub fn set_star_id(&mut self, star_id: i32) {
    self.star_id = Some(star_id);
  }

  pub fn with_star_id(mut self, star_id: i32) -> GetUniverseSystemsSystemIdOk {
    self.star_id = Some(star_id);
    self
  }

  pub fn star_id(&self) -> Option<&i32> {
    self.star_id.as_ref()
  }

  pub fn reset_star_id(&mut self) {
    self.star_id = None;
  }

  pub fn set_stargates(&mut self, stargates: Vec<i32>) {
    self.stargates = Some(stargates);
  }

  pub fn with_stargates(mut self, stargates: Vec<i32>) -> GetUniverseSystemsSystemIdOk {
    self.stargates = Some(stargates);
    self
  }

  pub fn stargates(&self) -> Option<&Vec<i32>> {
    self.stargates.as_ref()
  }

  pub fn reset_stargates(&mut self) {
    self.stargates = None;
  }

  pub fn set_stations(&mut self, stations: Vec<i32>) {
    self.stations = Some(stations);
  }

  pub fn with_stations(mut self, stations: Vec<i32>) -> GetUniverseSystemsSystemIdOk {
    self.stations = Some(stations);
    self
  }

  pub fn stations(&self) -> Option<&Vec<i32>> {
    self.stations.as_ref()
  }

  pub fn reset_stations(&mut self) {
    self.stations = None;
  }

  pub fn set_system_id(&mut self, system_id: i32) {
    self.system_id = system_id;
  }

  pub fn with_system_id(mut self, system_id: i32) -> GetUniverseSystemsSystemIdOk {
    self.system_id = system_id;
    self
  }

  pub fn system_id(&self) -> &i32 {
    &self.system_id
  }


}



