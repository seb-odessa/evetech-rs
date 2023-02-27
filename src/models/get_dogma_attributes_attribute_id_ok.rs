/* 
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * OpenAPI spec version: 1.17
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// GetDogmaAttributesAttributeIdOk : 200 ok object

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetDogmaAttributesAttributeIdOk {
  /// attribute_id integer
  #[serde(rename = "attribute_id")]
  attribute_id: i32,
  /// default_value number
  #[serde(rename = "default_value")]
  default_value: Option<f32>,
  /// description string
  #[serde(rename = "description")]
  description: Option<String>,
  /// display_name string
  #[serde(rename = "display_name")]
  display_name: Option<String>,
  /// high_is_good boolean
  #[serde(rename = "high_is_good")]
  high_is_good: Option<bool>,
  /// icon_id integer
  #[serde(rename = "icon_id")]
  icon_id: Option<i32>,
  /// name string
  #[serde(rename = "name")]
  name: Option<String>,
  /// published boolean
  #[serde(rename = "published")]
  published: Option<bool>,
  /// stackable boolean
  #[serde(rename = "stackable")]
  stackable: Option<bool>,
  /// unit_id integer
  #[serde(rename = "unit_id")]
  unit_id: Option<i32>
}

impl GetDogmaAttributesAttributeIdOk {
  /// 200 ok object
  pub fn new(attribute_id: i32) -> GetDogmaAttributesAttributeIdOk {
    GetDogmaAttributesAttributeIdOk {
      attribute_id: attribute_id,
      default_value: None,
      description: None,
      display_name: None,
      high_is_good: None,
      icon_id: None,
      name: None,
      published: None,
      stackable: None,
      unit_id: None
    }
  }

  pub fn set_attribute_id(&mut self, attribute_id: i32) {
    self.attribute_id = attribute_id;
  }

  pub fn with_attribute_id(mut self, attribute_id: i32) -> GetDogmaAttributesAttributeIdOk {
    self.attribute_id = attribute_id;
    self
  }

  pub fn attribute_id(&self) -> &i32 {
    &self.attribute_id
  }


  pub fn set_default_value(&mut self, default_value: f32) {
    self.default_value = Some(default_value);
  }

  pub fn with_default_value(mut self, default_value: f32) -> GetDogmaAttributesAttributeIdOk {
    self.default_value = Some(default_value);
    self
  }

  pub fn default_value(&self) -> Option<&f32> {
    self.default_value.as_ref()
  }

  pub fn reset_default_value(&mut self) {
    self.default_value = None;
  }

  pub fn set_description(&mut self, description: String) {
    self.description = Some(description);
  }

  pub fn with_description(mut self, description: String) -> GetDogmaAttributesAttributeIdOk {
    self.description = Some(description);
    self
  }

  pub fn description(&self) -> Option<&String> {
    self.description.as_ref()
  }

  pub fn reset_description(&mut self) {
    self.description = None;
  }

  pub fn set_display_name(&mut self, display_name: String) {
    self.display_name = Some(display_name);
  }

  pub fn with_display_name(mut self, display_name: String) -> GetDogmaAttributesAttributeIdOk {
    self.display_name = Some(display_name);
    self
  }

  pub fn display_name(&self) -> Option<&String> {
    self.display_name.as_ref()
  }

  pub fn reset_display_name(&mut self) {
    self.display_name = None;
  }

  pub fn set_high_is_good(&mut self, high_is_good: bool) {
    self.high_is_good = Some(high_is_good);
  }

  pub fn with_high_is_good(mut self, high_is_good: bool) -> GetDogmaAttributesAttributeIdOk {
    self.high_is_good = Some(high_is_good);
    self
  }

  pub fn high_is_good(&self) -> Option<&bool> {
    self.high_is_good.as_ref()
  }

  pub fn reset_high_is_good(&mut self) {
    self.high_is_good = None;
  }

  pub fn set_icon_id(&mut self, icon_id: i32) {
    self.icon_id = Some(icon_id);
  }

  pub fn with_icon_id(mut self, icon_id: i32) -> GetDogmaAttributesAttributeIdOk {
    self.icon_id = Some(icon_id);
    self
  }

  pub fn icon_id(&self) -> Option<&i32> {
    self.icon_id.as_ref()
  }

  pub fn reset_icon_id(&mut self) {
    self.icon_id = None;
  }

  pub fn set_name(&mut self, name: String) {
    self.name = Some(name);
  }

  pub fn with_name(mut self, name: String) -> GetDogmaAttributesAttributeIdOk {
    self.name = Some(name);
    self
  }

  pub fn name(&self) -> Option<&String> {
    self.name.as_ref()
  }

  pub fn reset_name(&mut self) {
    self.name = None;
  }

  pub fn set_published(&mut self, published: bool) {
    self.published = Some(published);
  }

  pub fn with_published(mut self, published: bool) -> GetDogmaAttributesAttributeIdOk {
    self.published = Some(published);
    self
  }

  pub fn published(&self) -> Option<&bool> {
    self.published.as_ref()
  }

  pub fn reset_published(&mut self) {
    self.published = None;
  }

  pub fn set_stackable(&mut self, stackable: bool) {
    self.stackable = Some(stackable);
  }

  pub fn with_stackable(mut self, stackable: bool) -> GetDogmaAttributesAttributeIdOk {
    self.stackable = Some(stackable);
    self
  }

  pub fn stackable(&self) -> Option<&bool> {
    self.stackable.as_ref()
  }

  pub fn reset_stackable(&mut self) {
    self.stackable = None;
  }

  pub fn set_unit_id(&mut self, unit_id: i32) {
    self.unit_id = Some(unit_id);
  }

  pub fn with_unit_id(mut self, unit_id: i32) -> GetDogmaAttributesAttributeIdOk {
    self.unit_id = Some(unit_id);
    self
  }

  pub fn unit_id(&self) -> Option<&i32> {
    self.unit_id.as_ref()
  }

  pub fn reset_unit_id(&mut self) {
    self.unit_id = None;
  }

}



