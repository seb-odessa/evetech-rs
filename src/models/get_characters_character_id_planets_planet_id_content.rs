/* 
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * OpenAPI spec version: 1.17
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// GetCharactersCharacterIdPlanetsPlanetIdContent : content object

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetCharactersCharacterIdPlanetsPlanetIdContent {
  /// amount integer
  #[serde(rename = "amount")]
  amount: i64,
  /// type_id integer
  #[serde(rename = "type_id")]
  type_id: i32
}

impl GetCharactersCharacterIdPlanetsPlanetIdContent {
  /// content object
  pub fn new(amount: i64, type_id: i32) -> GetCharactersCharacterIdPlanetsPlanetIdContent {
    GetCharactersCharacterIdPlanetsPlanetIdContent {
      amount: amount,
      type_id: type_id
    }
  }

  pub fn set_amount(&mut self, amount: i64) {
    self.amount = amount;
  }

  pub fn with_amount(mut self, amount: i64) -> GetCharactersCharacterIdPlanetsPlanetIdContent {
    self.amount = amount;
    self
  }

  pub fn amount(&self) -> &i64 {
    &self.amount
  }


  pub fn set_type_id(&mut self, type_id: i32) {
    self.type_id = type_id;
  }

  pub fn with_type_id(mut self, type_id: i32) -> GetCharactersCharacterIdPlanetsPlanetIdContent {
    self.type_id = type_id;
    self
  }

  pub fn type_id(&self) -> &i32 {
    &self.type_id
  }


}



