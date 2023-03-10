/* 
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * OpenAPI spec version: 1.17
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// GetCorporationsCorporationIdContainersLogs200Ok : 200 ok object

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetCorporationsCorporationIdContainersLogs200Ok {
  /// action string
  #[serde(rename = "action")]
  action: String,
  /// ID of the character who performed the action.
  #[serde(rename = "character_id")]
  character_id: i32,
  /// ID of the container
  #[serde(rename = "container_id")]
  container_id: i64,
  /// Type ID of the container
  #[serde(rename = "container_type_id")]
  container_type_id: i32,
  /// location_flag string
  #[serde(rename = "location_flag")]
  location_flag: String,
  /// location_id integer
  #[serde(rename = "location_id")]
  location_id: i64,
  /// Timestamp when this log was created
  #[serde(rename = "logged_at")]
  logged_at: String,
  /// new_config_bitmask integer
  #[serde(rename = "new_config_bitmask")]
  new_config_bitmask: Option<i32>,
  /// old_config_bitmask integer
  #[serde(rename = "old_config_bitmask")]
  old_config_bitmask: Option<i32>,
  /// Type of password set if action is of type SetPassword or EnterPassword
  #[serde(rename = "password_type")]
  password_type: Option<String>,
  /// Quantity of the item being acted upon
  #[serde(rename = "quantity")]
  quantity: Option<i32>,
  /// Type ID of the item being acted upon
  #[serde(rename = "type_id")]
  type_id: Option<i32>
}

impl GetCorporationsCorporationIdContainersLogs200Ok {
  /// 200 ok object
  pub fn new(action: String, character_id: i32, container_id: i64, container_type_id: i32, location_flag: String, location_id: i64, logged_at: String) -> GetCorporationsCorporationIdContainersLogs200Ok {
    GetCorporationsCorporationIdContainersLogs200Ok {
      action: action,
      character_id: character_id,
      container_id: container_id,
      container_type_id: container_type_id,
      location_flag: location_flag,
      location_id: location_id,
      logged_at: logged_at,
      new_config_bitmask: None,
      old_config_bitmask: None,
      password_type: None,
      quantity: None,
      type_id: None
    }
  }

  pub fn set_action(&mut self, action: String) {
    self.action = action;
  }

  pub fn with_action(mut self, action: String) -> GetCorporationsCorporationIdContainersLogs200Ok {
    self.action = action;
    self
  }

  pub fn action(&self) -> &String {
    &self.action
  }


  pub fn set_character_id(&mut self, character_id: i32) {
    self.character_id = character_id;
  }

  pub fn with_character_id(mut self, character_id: i32) -> GetCorporationsCorporationIdContainersLogs200Ok {
    self.character_id = character_id;
    self
  }

  pub fn character_id(&self) -> &i32 {
    &self.character_id
  }


  pub fn set_container_id(&mut self, container_id: i64) {
    self.container_id = container_id;
  }

  pub fn with_container_id(mut self, container_id: i64) -> GetCorporationsCorporationIdContainersLogs200Ok {
    self.container_id = container_id;
    self
  }

  pub fn container_id(&self) -> &i64 {
    &self.container_id
  }


  pub fn set_container_type_id(&mut self, container_type_id: i32) {
    self.container_type_id = container_type_id;
  }

  pub fn with_container_type_id(mut self, container_type_id: i32) -> GetCorporationsCorporationIdContainersLogs200Ok {
    self.container_type_id = container_type_id;
    self
  }

  pub fn container_type_id(&self) -> &i32 {
    &self.container_type_id
  }


  pub fn set_location_flag(&mut self, location_flag: String) {
    self.location_flag = location_flag;
  }

  pub fn with_location_flag(mut self, location_flag: String) -> GetCorporationsCorporationIdContainersLogs200Ok {
    self.location_flag = location_flag;
    self
  }

  pub fn location_flag(&self) -> &String {
    &self.location_flag
  }


  pub fn set_location_id(&mut self, location_id: i64) {
    self.location_id = location_id;
  }

  pub fn with_location_id(mut self, location_id: i64) -> GetCorporationsCorporationIdContainersLogs200Ok {
    self.location_id = location_id;
    self
  }

  pub fn location_id(&self) -> &i64 {
    &self.location_id
  }


  pub fn set_logged_at(&mut self, logged_at: String) {
    self.logged_at = logged_at;
  }

  pub fn with_logged_at(mut self, logged_at: String) -> GetCorporationsCorporationIdContainersLogs200Ok {
    self.logged_at = logged_at;
    self
  }

  pub fn logged_at(&self) -> &String {
    &self.logged_at
  }


  pub fn set_new_config_bitmask(&mut self, new_config_bitmask: i32) {
    self.new_config_bitmask = Some(new_config_bitmask);
  }

  pub fn with_new_config_bitmask(mut self, new_config_bitmask: i32) -> GetCorporationsCorporationIdContainersLogs200Ok {
    self.new_config_bitmask = Some(new_config_bitmask);
    self
  }

  pub fn new_config_bitmask(&self) -> Option<&i32> {
    self.new_config_bitmask.as_ref()
  }

  pub fn reset_new_config_bitmask(&mut self) {
    self.new_config_bitmask = None;
  }

  pub fn set_old_config_bitmask(&mut self, old_config_bitmask: i32) {
    self.old_config_bitmask = Some(old_config_bitmask);
  }

  pub fn with_old_config_bitmask(mut self, old_config_bitmask: i32) -> GetCorporationsCorporationIdContainersLogs200Ok {
    self.old_config_bitmask = Some(old_config_bitmask);
    self
  }

  pub fn old_config_bitmask(&self) -> Option<&i32> {
    self.old_config_bitmask.as_ref()
  }

  pub fn reset_old_config_bitmask(&mut self) {
    self.old_config_bitmask = None;
  }

  pub fn set_password_type(&mut self, password_type: String) {
    self.password_type = Some(password_type);
  }

  pub fn with_password_type(mut self, password_type: String) -> GetCorporationsCorporationIdContainersLogs200Ok {
    self.password_type = Some(password_type);
    self
  }

  pub fn password_type(&self) -> Option<&String> {
    self.password_type.as_ref()
  }

  pub fn reset_password_type(&mut self) {
    self.password_type = None;
  }

  pub fn set_quantity(&mut self, quantity: i32) {
    self.quantity = Some(quantity);
  }

  pub fn with_quantity(mut self, quantity: i32) -> GetCorporationsCorporationIdContainersLogs200Ok {
    self.quantity = Some(quantity);
    self
  }

  pub fn quantity(&self) -> Option<&i32> {
    self.quantity.as_ref()
  }

  pub fn reset_quantity(&mut self) {
    self.quantity = None;
  }

  pub fn set_type_id(&mut self, type_id: i32) {
    self.type_id = Some(type_id);
  }

  pub fn with_type_id(mut self, type_id: i32) -> GetCorporationsCorporationIdContainersLogs200Ok {
    self.type_id = Some(type_id);
    self
  }

  pub fn type_id(&self) -> Option<&i32> {
    self.type_id.as_ref()
  }

  pub fn reset_type_id(&mut self) {
    self.type_id = None;
  }

}



