/* 
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * OpenAPI spec version: 1.17
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// PostUniverseNames200Ok : 200 ok object

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct PostUniverseNames200Ok {
  /// category string
  #[serde(rename = "category")]
  category: String,
  /// id integer
  #[serde(rename = "id")]
  id: i32,
  /// name string
  #[serde(rename = "name")]
  name: String
}

impl PostUniverseNames200Ok {
  /// 200 ok object
  pub fn new(category: String, id: i32, name: String) -> PostUniverseNames200Ok {
    PostUniverseNames200Ok {
      category: category,
      id: id,
      name: name
    }
  }

  pub fn set_category(&mut self, category: String) {
    self.category = category;
  }

  pub fn with_category(mut self, category: String) -> PostUniverseNames200Ok {
    self.category = category;
    self
  }

  pub fn category(&self) -> &String {
    &self.category
  }


  pub fn set_id(&mut self, id: i32) {
    self.id = id;
  }

  pub fn with_id(mut self, id: i32) -> PostUniverseNames200Ok {
    self.id = id;
    self
  }

  pub fn id(&self) -> &i32 {
    &self.id
  }


  pub fn set_name(&mut self, name: String) {
    self.name = name;
  }

  pub fn with_name(mut self, name: String) -> PostUniverseNames200Ok {
    self.name = name;
    self
  }

  pub fn name(&self) -> &String {
    &self.name
  }


}



