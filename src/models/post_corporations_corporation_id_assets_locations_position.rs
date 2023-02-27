/* 
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * OpenAPI spec version: 1.17
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// PostCorporationsCorporationIdAssetsLocationsPosition : position object

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct PostCorporationsCorporationIdAssetsLocationsPosition {
  /// x number
  #[serde(rename = "x")]
  x: f64,
  /// y number
  #[serde(rename = "y")]
  y: f64,
  /// z number
  #[serde(rename = "z")]
  z: f64
}

impl PostCorporationsCorporationIdAssetsLocationsPosition {
  /// position object
  pub fn new(x: f64, y: f64, z: f64) -> PostCorporationsCorporationIdAssetsLocationsPosition {
    PostCorporationsCorporationIdAssetsLocationsPosition {
      x: x,
      y: y,
      z: z
    }
  }

  pub fn set_x(&mut self, x: f64) {
    self.x = x;
  }

  pub fn with_x(mut self, x: f64) -> PostCorporationsCorporationIdAssetsLocationsPosition {
    self.x = x;
    self
  }

  pub fn x(&self) -> &f64 {
    &self.x
  }


  pub fn set_y(&mut self, y: f64) {
    self.y = y;
  }

  pub fn with_y(mut self, y: f64) -> PostCorporationsCorporationIdAssetsLocationsPosition {
    self.y = y;
    self
  }

  pub fn y(&self) -> &f64 {
    &self.y
  }


  pub fn set_z(&mut self, z: f64) {
    self.z = z;
  }

  pub fn with_z(mut self, z: f64) -> PostCorporationsCorporationIdAssetsLocationsPosition {
    self.z = z;
    self
  }

  pub fn z(&self) -> &f64 {
    &self.z
  }


}



