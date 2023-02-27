/* 
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * OpenAPI spec version: 1.17
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// PostCharactersCharacterIdMailRecipient : recipient object

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct PostCharactersCharacterIdMailRecipient {
  /// recipient_id integer
  #[serde(rename = "recipient_id")]
  recipient_id: i32,
  /// recipient_type string
  #[serde(rename = "recipient_type")]
  recipient_type: String
}

impl PostCharactersCharacterIdMailRecipient {
  /// recipient object
  pub fn new(recipient_id: i32, recipient_type: String) -> PostCharactersCharacterIdMailRecipient {
    PostCharactersCharacterIdMailRecipient {
      recipient_id: recipient_id,
      recipient_type: recipient_type
    }
  }

  pub fn set_recipient_id(&mut self, recipient_id: i32) {
    self.recipient_id = recipient_id;
  }

  pub fn with_recipient_id(mut self, recipient_id: i32) -> PostCharactersCharacterIdMailRecipient {
    self.recipient_id = recipient_id;
    self
  }

  pub fn recipient_id(&self) -> &i32 {
    &self.recipient_id
  }


  pub fn set_recipient_type(&mut self, recipient_type: String) {
    self.recipient_type = recipient_type;
  }

  pub fn with_recipient_type(mut self, recipient_type: String) -> PostCharactersCharacterIdMailRecipient {
    self.recipient_type = recipient_type;
    self
  }

  pub fn recipient_type(&self) -> &String {
    &self.recipient_type
  }


}



