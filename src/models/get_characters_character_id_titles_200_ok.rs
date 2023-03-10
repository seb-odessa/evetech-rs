/* 
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * OpenAPI spec version: 1.17
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// GetCharactersCharacterIdTitles200Ok : 200 ok object

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetCharactersCharacterIdTitles200Ok {
  /// name string
  #[serde(rename = "name")]
  name: Option<String>,
  /// title_id integer
  #[serde(rename = "title_id")]
  title_id: Option<i32>
}

impl GetCharactersCharacterIdTitles200Ok {
  /// 200 ok object
  pub fn new() -> GetCharactersCharacterIdTitles200Ok {
    GetCharactersCharacterIdTitles200Ok {
      name: None,
      title_id: None
    }
  }

  pub fn set_name(&mut self, name: String) {
    self.name = Some(name);
  }

  pub fn with_name(mut self, name: String) -> GetCharactersCharacterIdTitles200Ok {
    self.name = Some(name);
    self
  }

  pub fn name(&self) -> Option<&String> {
    self.name.as_ref()
  }

  pub fn reset_name(&mut self) {
    self.name = None;
  }

  pub fn set_title_id(&mut self, title_id: i32) {
    self.title_id = Some(title_id);
  }

  pub fn with_title_id(mut self, title_id: i32) -> GetCharactersCharacterIdTitles200Ok {
    self.title_id = Some(title_id);
    self
  }

  pub fn title_id(&self) -> Option<&i32> {
    self.title_id.as_ref()
  }

  pub fn reset_title_id(&mut self) {
    self.title_id = None;
  }

}



