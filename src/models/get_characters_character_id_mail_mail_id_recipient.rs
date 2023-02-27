/* 
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * OpenAPI spec version: 1.17
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// GetCharactersCharacterIdMailMailIdRecipient : recipient object

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetCharactersCharacterIdMailMailIdRecipient {
  /// recipient_id integer
  #[serde(rename = "recipient_id")]
  recipient_id: i32,
  /// recipient_type string
  #[serde(rename = "recipient_type")]
  recipient_type: String
}

impl GetCharactersCharacterIdMailMailIdRecipient {
  /// recipient object
  pub fn new(recipient_id: i32, recipient_type: String) -> GetCharactersCharacterIdMailMailIdRecipient {
    GetCharactersCharacterIdMailMailIdRecipient {
      recipient_id: recipient_id,
      recipient_type: recipient_type
    }
  }

  pub fn set_recipient_id(&mut self, recipient_id: i32) {
    self.recipient_id = recipient_id;
  }

  pub fn with_recipient_id(mut self, recipient_id: i32) -> GetCharactersCharacterIdMailMailIdRecipient {
    self.recipient_id = recipient_id;
    self
  }

  pub fn recipient_id(&self) -> &i32 {
    &self.recipient_id
  }


  pub fn set_recipient_type(&mut self, recipient_type: String) {
    self.recipient_type = recipient_type;
  }

  pub fn with_recipient_type(mut self, recipient_type: String) -> GetCharactersCharacterIdMailMailIdRecipient {
    self.recipient_type = recipient_type;
    self
  }

  pub fn recipient_type(&self) -> &String {
    &self.recipient_type
  }


}



