/* 
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * OpenAPI spec version: 1.17
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// GetIndustryFacilities200Ok : 200 ok object

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetIndustryFacilities200Ok {
  /// ID of the facility
  #[serde(rename = "facility_id")]
  facility_id: i64,
  /// Owner of the facility
  #[serde(rename = "owner_id")]
  owner_id: i32,
  /// Region ID where the facility is
  #[serde(rename = "region_id")]
  region_id: i32,
  /// Solar system ID where the facility is
  #[serde(rename = "solar_system_id")]
  solar_system_id: i32,
  /// Tax imposed by the facility
  #[serde(rename = "tax")]
  tax: Option<f32>,
  /// Type ID of the facility
  #[serde(rename = "type_id")]
  type_id: i32
}

impl GetIndustryFacilities200Ok {
  /// 200 ok object
  pub fn new(facility_id: i64, owner_id: i32, region_id: i32, solar_system_id: i32, type_id: i32) -> GetIndustryFacilities200Ok {
    GetIndustryFacilities200Ok {
      facility_id: facility_id,
      owner_id: owner_id,
      region_id: region_id,
      solar_system_id: solar_system_id,
      tax: None,
      type_id: type_id
    }
  }

  pub fn set_facility_id(&mut self, facility_id: i64) {
    self.facility_id = facility_id;
  }

  pub fn with_facility_id(mut self, facility_id: i64) -> GetIndustryFacilities200Ok {
    self.facility_id = facility_id;
    self
  }

  pub fn facility_id(&self) -> &i64 {
    &self.facility_id
  }


  pub fn set_owner_id(&mut self, owner_id: i32) {
    self.owner_id = owner_id;
  }

  pub fn with_owner_id(mut self, owner_id: i32) -> GetIndustryFacilities200Ok {
    self.owner_id = owner_id;
    self
  }

  pub fn owner_id(&self) -> &i32 {
    &self.owner_id
  }


  pub fn set_region_id(&mut self, region_id: i32) {
    self.region_id = region_id;
  }

  pub fn with_region_id(mut self, region_id: i32) -> GetIndustryFacilities200Ok {
    self.region_id = region_id;
    self
  }

  pub fn region_id(&self) -> &i32 {
    &self.region_id
  }


  pub fn set_solar_system_id(&mut self, solar_system_id: i32) {
    self.solar_system_id = solar_system_id;
  }

  pub fn with_solar_system_id(mut self, solar_system_id: i32) -> GetIndustryFacilities200Ok {
    self.solar_system_id = solar_system_id;
    self
  }

  pub fn solar_system_id(&self) -> &i32 {
    &self.solar_system_id
  }


  pub fn set_tax(&mut self, tax: f32) {
    self.tax = Some(tax);
  }

  pub fn with_tax(mut self, tax: f32) -> GetIndustryFacilities200Ok {
    self.tax = Some(tax);
    self
  }

  pub fn tax(&self) -> Option<&f32> {
    self.tax.as_ref()
  }

  pub fn reset_tax(&mut self) {
    self.tax = None;
  }

  pub fn set_type_id(&mut self, type_id: i32) {
    self.type_id = type_id;
  }

  pub fn with_type_id(mut self, type_id: i32) -> GetIndustryFacilities200Ok {
    self.type_id = type_id;
    self
  }

  pub fn type_id(&self) -> &i32 {
    &self.type_id
  }


}



