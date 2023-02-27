/* 
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * OpenAPI spec version: 1.17
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// GetCharactersCharacterIdOpportunities200Ok : 200 ok object

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetCharactersCharacterIdOpportunities200Ok {
  /// completed_at string
  #[serde(rename = "completed_at")]
  completed_at: String,
  /// task_id integer
  #[serde(rename = "task_id")]
  task_id: i32
}

impl GetCharactersCharacterIdOpportunities200Ok {
  /// 200 ok object
  pub fn new(completed_at: String, task_id: i32) -> GetCharactersCharacterIdOpportunities200Ok {
    GetCharactersCharacterIdOpportunities200Ok {
      completed_at: completed_at,
      task_id: task_id
    }
  }

  pub fn set_completed_at(&mut self, completed_at: String) {
    self.completed_at = completed_at;
  }

  pub fn with_completed_at(mut self, completed_at: String) -> GetCharactersCharacterIdOpportunities200Ok {
    self.completed_at = completed_at;
    self
  }

  pub fn completed_at(&self) -> &String {
    &self.completed_at
  }


  pub fn set_task_id(&mut self, task_id: i32) {
    self.task_id = task_id;
  }

  pub fn with_task_id(mut self, task_id: i32) -> GetCharactersCharacterIdOpportunities200Ok {
    self.task_id = task_id;
    self
  }

  pub fn task_id(&self) -> &i32 {
    &self.task_id
  }


}



