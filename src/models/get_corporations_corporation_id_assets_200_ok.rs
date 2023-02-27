/* 
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * OpenAPI spec version: 1.17
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// GetCorporationsCorporationIdAssets200Ok : 200 ok object

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetCorporationsCorporationIdAssets200Ok {
  /// is_blueprint_copy boolean
  #[serde(rename = "is_blueprint_copy")]
  is_blueprint_copy: Option<bool>,
  /// is_singleton boolean
  #[serde(rename = "is_singleton")]
  is_singleton: bool,
  /// item_id integer
  #[serde(rename = "item_id")]
  item_id: i64,
  /// location_flag string
  #[serde(rename = "location_flag")]
  location_flag: String,
  /// location_id integer
  #[serde(rename = "location_id")]
  location_id: i64,
  /// location_type string
  #[serde(rename = "location_type")]
  location_type: String,
  /// quantity integer
  #[serde(rename = "quantity")]
  quantity: i32,
  /// type_id integer
  #[serde(rename = "type_id")]
  type_id: i32
}

impl GetCorporationsCorporationIdAssets200Ok {
  /// 200 ok object
  pub fn new(is_singleton: bool, item_id: i64, location_flag: String, location_id: i64, location_type: String, quantity: i32, type_id: i32) -> GetCorporationsCorporationIdAssets200Ok {
    GetCorporationsCorporationIdAssets200Ok {
      is_blueprint_copy: None,
      is_singleton: is_singleton,
      item_id: item_id,
      location_flag: location_flag,
      location_id: location_id,
      location_type: location_type,
      quantity: quantity,
      type_id: type_id
    }
  }

  pub fn set_is_blueprint_copy(&mut self, is_blueprint_copy: bool) {
    self.is_blueprint_copy = Some(is_blueprint_copy);
  }

  pub fn with_is_blueprint_copy(mut self, is_blueprint_copy: bool) -> GetCorporationsCorporationIdAssets200Ok {
    self.is_blueprint_copy = Some(is_blueprint_copy);
    self
  }

  pub fn is_blueprint_copy(&self) -> Option<&bool> {
    self.is_blueprint_copy.as_ref()
  }

  pub fn reset_is_blueprint_copy(&mut self) {
    self.is_blueprint_copy = None;
  }

  pub fn set_is_singleton(&mut self, is_singleton: bool) {
    self.is_singleton = is_singleton;
  }

  pub fn with_is_singleton(mut self, is_singleton: bool) -> GetCorporationsCorporationIdAssets200Ok {
    self.is_singleton = is_singleton;
    self
  }

  pub fn is_singleton(&self) -> &bool {
    &self.is_singleton
  }


  pub fn set_item_id(&mut self, item_id: i64) {
    self.item_id = item_id;
  }

  pub fn with_item_id(mut self, item_id: i64) -> GetCorporationsCorporationIdAssets200Ok {
    self.item_id = item_id;
    self
  }

  pub fn item_id(&self) -> &i64 {
    &self.item_id
  }


  pub fn set_location_flag(&mut self, location_flag: String) {
    self.location_flag = location_flag;
  }

  pub fn with_location_flag(mut self, location_flag: String) -> GetCorporationsCorporationIdAssets200Ok {
    self.location_flag = location_flag;
    self
  }

  pub fn location_flag(&self) -> &String {
    &self.location_flag
  }


  pub fn set_location_id(&mut self, location_id: i64) {
    self.location_id = location_id;
  }

  pub fn with_location_id(mut self, location_id: i64) -> GetCorporationsCorporationIdAssets200Ok {
    self.location_id = location_id;
    self
  }

  pub fn location_id(&self) -> &i64 {
    &self.location_id
  }


  pub fn set_location_type(&mut self, location_type: String) {
    self.location_type = location_type;
  }

  pub fn with_location_type(mut self, location_type: String) -> GetCorporationsCorporationIdAssets200Ok {
    self.location_type = location_type;
    self
  }

  pub fn location_type(&self) -> &String {
    &self.location_type
  }


  pub fn set_quantity(&mut self, quantity: i32) {
    self.quantity = quantity;
  }

  pub fn with_quantity(mut self, quantity: i32) -> GetCorporationsCorporationIdAssets200Ok {
    self.quantity = quantity;
    self
  }

  pub fn quantity(&self) -> &i32 {
    &self.quantity
  }


  pub fn set_type_id(&mut self, type_id: i32) {
    self.type_id = type_id;
  }

  pub fn with_type_id(mut self, type_id: i32) -> GetCorporationsCorporationIdAssets200Ok {
    self.type_id = type_id;
    self
  }

  pub fn type_id(&self) -> &i32 {
    &self.type_id
  }


}



