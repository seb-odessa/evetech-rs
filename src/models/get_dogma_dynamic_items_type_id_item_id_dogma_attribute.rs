/* 
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * OpenAPI spec version: 1.17
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// GetDogmaDynamicItemsTypeIdItemIdDogmaAttribute : dogma_attribute object

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetDogmaDynamicItemsTypeIdItemIdDogmaAttribute {
  /// attribute_id integer
  #[serde(rename = "attribute_id")]
  attribute_id: i32,
  /// value number
  #[serde(rename = "value")]
  value: f32
}

impl GetDogmaDynamicItemsTypeIdItemIdDogmaAttribute {
  /// dogma_attribute object
  pub fn new(attribute_id: i32, value: f32) -> GetDogmaDynamicItemsTypeIdItemIdDogmaAttribute {
    GetDogmaDynamicItemsTypeIdItemIdDogmaAttribute {
      attribute_id: attribute_id,
      value: value
    }
  }

  pub fn set_attribute_id(&mut self, attribute_id: i32) {
    self.attribute_id = attribute_id;
  }

  pub fn with_attribute_id(mut self, attribute_id: i32) -> GetDogmaDynamicItemsTypeIdItemIdDogmaAttribute {
    self.attribute_id = attribute_id;
    self
  }

  pub fn attribute_id(&self) -> &i32 {
    &self.attribute_id
  }


  pub fn set_value(&mut self, value: f32) {
    self.value = value;
  }

  pub fn with_value(mut self, value: f32) -> GetDogmaDynamicItemsTypeIdItemIdDogmaAttribute {
    self.value = value;
    self
  }

  pub fn value(&self) -> &f32 {
    &self.value
  }


}



