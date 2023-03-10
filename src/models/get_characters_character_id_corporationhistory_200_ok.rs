/* 
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * OpenAPI spec version: 1.17
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// GetCharactersCharacterIdCorporationhistory200Ok : 200 ok object

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetCharactersCharacterIdCorporationhistory200Ok {
  /// corporation_id integer
  #[serde(rename = "corporation_id")]
  corporation_id: i32,
  /// True if the corporation has been deleted
  #[serde(rename = "is_deleted")]
  is_deleted: Option<bool>,
  /// An incrementing ID that can be used to canonically establish order of records in cases where dates may be ambiguous
  #[serde(rename = "record_id")]
  record_id: i32,
  /// start_date string
  #[serde(rename = "start_date")]
  start_date: String
}

impl GetCharactersCharacterIdCorporationhistory200Ok {
  /// 200 ok object
  pub fn new(corporation_id: i32, record_id: i32, start_date: String) -> GetCharactersCharacterIdCorporationhistory200Ok {
    GetCharactersCharacterIdCorporationhistory200Ok {
      corporation_id: corporation_id,
      is_deleted: None,
      record_id: record_id,
      start_date: start_date
    }
  }

  pub fn set_corporation_id(&mut self, corporation_id: i32) {
    self.corporation_id = corporation_id;
  }

  pub fn with_corporation_id(mut self, corporation_id: i32) -> GetCharactersCharacterIdCorporationhistory200Ok {
    self.corporation_id = corporation_id;
    self
  }

  pub fn corporation_id(&self) -> &i32 {
    &self.corporation_id
  }


  pub fn set_is_deleted(&mut self, is_deleted: bool) {
    self.is_deleted = Some(is_deleted);
  }

  pub fn with_is_deleted(mut self, is_deleted: bool) -> GetCharactersCharacterIdCorporationhistory200Ok {
    self.is_deleted = Some(is_deleted);
    self
  }

  pub fn is_deleted(&self) -> Option<&bool> {
    self.is_deleted.as_ref()
  }

  pub fn reset_is_deleted(&mut self) {
    self.is_deleted = None;
  }

  pub fn set_record_id(&mut self, record_id: i32) {
    self.record_id = record_id;
  }

  pub fn with_record_id(mut self, record_id: i32) -> GetCharactersCharacterIdCorporationhistory200Ok {
    self.record_id = record_id;
    self
  }

  pub fn record_id(&self) -> &i32 {
    &self.record_id
  }


  pub fn set_start_date(&mut self, start_date: String) {
    self.start_date = start_date;
  }

  pub fn with_start_date(mut self, start_date: String) -> GetCharactersCharacterIdCorporationhistory200Ok {
    self.start_date = start_date;
    self
  }

  pub fn start_date(&self) -> &String {
    &self.start_date
  }


}



