/* 
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * OpenAPI spec version: 1.17
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// GetUniverseCategoriesCategoryIdOk : 200 ok object

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetUniverseCategoriesCategoryIdOk {
  /// category_id integer
  #[serde(rename = "category_id")]
  category_id: i32,
  /// groups array
  #[serde(rename = "groups")]
  groups: Vec<i32>,
  /// name string
  #[serde(rename = "name")]
  name: String,
  /// published boolean
  #[serde(rename = "published")]
  published: bool
}

impl GetUniverseCategoriesCategoryIdOk {
  /// 200 ok object
  pub fn new(category_id: i32, groups: Vec<i32>, name: String, published: bool) -> GetUniverseCategoriesCategoryIdOk {
    GetUniverseCategoriesCategoryIdOk {
      category_id: category_id,
      groups: groups,
      name: name,
      published: published
    }
  }

  pub fn set_category_id(&mut self, category_id: i32) {
    self.category_id = category_id;
  }

  pub fn with_category_id(mut self, category_id: i32) -> GetUniverseCategoriesCategoryIdOk {
    self.category_id = category_id;
    self
  }

  pub fn category_id(&self) -> &i32 {
    &self.category_id
  }


  pub fn set_groups(&mut self, groups: Vec<i32>) {
    self.groups = groups;
  }

  pub fn with_groups(mut self, groups: Vec<i32>) -> GetUniverseCategoriesCategoryIdOk {
    self.groups = groups;
    self
  }

  pub fn groups(&self) -> &Vec<i32> {
    &self.groups
  }


  pub fn set_name(&mut self, name: String) {
    self.name = name;
  }

  pub fn with_name(mut self, name: String) -> GetUniverseCategoriesCategoryIdOk {
    self.name = name;
    self
  }

  pub fn name(&self) -> &String {
    &self.name
  }


  pub fn set_published(&mut self, published: bool) {
    self.published = published;
  }

  pub fn with_published(mut self, published: bool) -> GetUniverseCategoriesCategoryIdOk {
    self.published = published;
    self
  }

  pub fn published(&self) -> &bool {
    &self.published
  }


}



