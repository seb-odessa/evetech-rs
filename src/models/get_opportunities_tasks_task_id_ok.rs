/* 
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * OpenAPI spec version: 1.17
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// GetOpportunitiesTasksTaskIdOk : 200 ok object

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetOpportunitiesTasksTaskIdOk {
  /// description string
  #[serde(rename = "description")]
  description: String,
  /// name string
  #[serde(rename = "name")]
  name: String,
  /// notification string
  #[serde(rename = "notification")]
  notification: String,
  /// task_id integer
  #[serde(rename = "task_id")]
  task_id: i32
}

impl GetOpportunitiesTasksTaskIdOk {
  /// 200 ok object
  pub fn new(description: String, name: String, notification: String, task_id: i32) -> GetOpportunitiesTasksTaskIdOk {
    GetOpportunitiesTasksTaskIdOk {
      description: description,
      name: name,
      notification: notification,
      task_id: task_id
    }
  }

  pub fn set_description(&mut self, description: String) {
    self.description = description;
  }

  pub fn with_description(mut self, description: String) -> GetOpportunitiesTasksTaskIdOk {
    self.description = description;
    self
  }

  pub fn description(&self) -> &String {
    &self.description
  }


  pub fn set_name(&mut self, name: String) {
    self.name = name;
  }

  pub fn with_name(mut self, name: String) -> GetOpportunitiesTasksTaskIdOk {
    self.name = name;
    self
  }

  pub fn name(&self) -> &String {
    &self.name
  }


  pub fn set_notification(&mut self, notification: String) {
    self.notification = notification;
  }

  pub fn with_notification(mut self, notification: String) -> GetOpportunitiesTasksTaskIdOk {
    self.notification = notification;
    self
  }

  pub fn notification(&self) -> &String {
    &self.notification
  }


  pub fn set_task_id(&mut self, task_id: i32) {
    self.task_id = task_id;
  }

  pub fn with_task_id(mut self, task_id: i32) -> GetOpportunitiesTasksTaskIdOk {
    self.task_id = task_id;
    self
  }

  pub fn task_id(&self) -> &i32 {
    &self.task_id
  }


}



