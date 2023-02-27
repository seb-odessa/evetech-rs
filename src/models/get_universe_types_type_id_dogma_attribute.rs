/* 
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * OpenAPI spec version: 1.17
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// GetUniverseTypesTypeIdDogmaAttribute : dogma_attribute object

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetUniverseTypesTypeIdDogmaAttribute {
  /// attribute_id integer
  #[serde(rename = "attribute_id")]
  attribute_id: i32,
  /// value number
  #[serde(rename = "value")]
  value: f32
}

impl GetUniverseTypesTypeIdDogmaAttribute {
  /// dogma_attribute object
  pub fn new(attribute_id: i32, value: f32) -> GetUniverseTypesTypeIdDogmaAttribute {
    GetUniverseTypesTypeIdDogmaAttribute {
      attribute_id: attribute_id,
      value: value
    }
  }

  pub fn set_attribute_id(&mut self, attribute_id: i32) {
    self.attribute_id = attribute_id;
  }

  pub fn with_attribute_id(mut self, attribute_id: i32) -> GetUniverseTypesTypeIdDogmaAttribute {
    self.attribute_id = attribute_id;
    self
  }

  pub fn attribute_id(&self) -> &i32 {
    &self.attribute_id
  }


  pub fn set_value(&mut self, value: f32) {
    self.value = value;
  }

  pub fn with_value(mut self, value: f32) -> GetUniverseTypesTypeIdDogmaAttribute {
    self.value = value;
    self
  }

  pub fn value(&self) -> &f32 {
    &self.value
  }


}


