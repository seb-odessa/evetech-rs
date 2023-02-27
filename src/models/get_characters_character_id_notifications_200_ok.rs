/* 
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * OpenAPI spec version: 1.17
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// GetCharactersCharacterIdNotifications200Ok : 200 ok object

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetCharactersCharacterIdNotifications200Ok {
  /// is_read boolean
  #[serde(rename = "is_read")]
  is_read: Option<bool>,
  /// notification_id integer
  #[serde(rename = "notification_id")]
  notification_id: i64,
  /// sender_id integer
  #[serde(rename = "sender_id")]
  sender_id: i32,
  /// sender_type string
  #[serde(rename = "sender_type")]
  sender_type: String,
  /// text string
  #[serde(rename = "text")]
  text: Option<String>,
  /// timestamp string
  #[serde(rename = "timestamp")]
  timestamp: String,
  /// type string
  #[serde(rename = "type")]
  _type: String
}

impl GetCharactersCharacterIdNotifications200Ok {
  /// 200 ok object
  pub fn new(notification_id: i64, sender_id: i32, sender_type: String, timestamp: String, _type: String) -> GetCharactersCharacterIdNotifications200Ok {
    GetCharactersCharacterIdNotifications200Ok {
      is_read: None,
      notification_id: notification_id,
      sender_id: sender_id,
      sender_type: sender_type,
      text: None,
      timestamp: timestamp,
      _type: _type
    }
  }

  pub fn set_is_read(&mut self, is_read: bool) {
    self.is_read = Some(is_read);
  }

  pub fn with_is_read(mut self, is_read: bool) -> GetCharactersCharacterIdNotifications200Ok {
    self.is_read = Some(is_read);
    self
  }

  pub fn is_read(&self) -> Option<&bool> {
    self.is_read.as_ref()
  }

  pub fn reset_is_read(&mut self) {
    self.is_read = None;
  }

  pub fn set_notification_id(&mut self, notification_id: i64) {
    self.notification_id = notification_id;
  }

  pub fn with_notification_id(mut self, notification_id: i64) -> GetCharactersCharacterIdNotifications200Ok {
    self.notification_id = notification_id;
    self
  }

  pub fn notification_id(&self) -> &i64 {
    &self.notification_id
  }


  pub fn set_sender_id(&mut self, sender_id: i32) {
    self.sender_id = sender_id;
  }

  pub fn with_sender_id(mut self, sender_id: i32) -> GetCharactersCharacterIdNotifications200Ok {
    self.sender_id = sender_id;
    self
  }

  pub fn sender_id(&self) -> &i32 {
    &self.sender_id
  }


  pub fn set_sender_type(&mut self, sender_type: String) {
    self.sender_type = sender_type;
  }

  pub fn with_sender_type(mut self, sender_type: String) -> GetCharactersCharacterIdNotifications200Ok {
    self.sender_type = sender_type;
    self
  }

  pub fn sender_type(&self) -> &String {
    &self.sender_type
  }


  pub fn set_text(&mut self, text: String) {
    self.text = Some(text);
  }

  pub fn with_text(mut self, text: String) -> GetCharactersCharacterIdNotifications200Ok {
    self.text = Some(text);
    self
  }

  pub fn text(&self) -> Option<&String> {
    self.text.as_ref()
  }

  pub fn reset_text(&mut self) {
    self.text = None;
  }

  pub fn set_timestamp(&mut self, timestamp: String) {
    self.timestamp = timestamp;
  }

  pub fn with_timestamp(mut self, timestamp: String) -> GetCharactersCharacterIdNotifications200Ok {
    self.timestamp = timestamp;
    self
  }

  pub fn timestamp(&self) -> &String {
    &self.timestamp
  }


  pub fn set__type(&mut self, _type: String) {
    self._type = _type;
  }

  pub fn with__type(mut self, _type: String) -> GetCharactersCharacterIdNotifications200Ok {
    self._type = _type;
    self
  }

  pub fn _type(&self) -> &String {
    &self._type
  }


}


