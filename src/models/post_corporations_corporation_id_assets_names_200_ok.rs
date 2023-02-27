/* 
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * OpenAPI spec version: 1.17
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// PostCorporationsCorporationIdAssetsNames200Ok : 200 ok object

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct PostCorporationsCorporationIdAssetsNames200Ok {
  /// item_id integer
  #[serde(rename = "item_id")]
  item_id: i64,
  /// name string
  #[serde(rename = "name")]
  name: String
}

impl PostCorporationsCorporationIdAssetsNames200Ok {
  /// 200 ok object
  pub fn new(item_id: i64, name: String) -> PostCorporationsCorporationIdAssetsNames200Ok {
    PostCorporationsCorporationIdAssetsNames200Ok {
      item_id: item_id,
      name: name
    }
  }

  pub fn set_item_id(&mut self, item_id: i64) {
    self.item_id = item_id;
  }

  pub fn with_item_id(mut self, item_id: i64) -> PostCorporationsCorporationIdAssetsNames200Ok {
    self.item_id = item_id;
    self
  }

  pub fn item_id(&self) -> &i64 {
    &self.item_id
  }


  pub fn set_name(&mut self, name: String) {
    self.name = name;
  }

  pub fn with_name(mut self, name: String) -> PostCorporationsCorporationIdAssetsNames200Ok {
    self.name = name;
    self
  }

  pub fn name(&self) -> &String {
    &self.name
  }


}



