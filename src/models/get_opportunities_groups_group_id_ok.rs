/* 
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * OpenAPI spec version: 1.17
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// GetOpportunitiesGroupsGroupIdOk : 200 ok object

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetOpportunitiesGroupsGroupIdOk {
  /// The groups that are connected to this group on the opportunities map
  #[serde(rename = "connected_groups")]
  connected_groups: Vec<i32>,
  /// description string
  #[serde(rename = "description")]
  description: String,
  /// group_id integer
  #[serde(rename = "group_id")]
  group_id: i32,
  /// name string
  #[serde(rename = "name")]
  name: String,
  /// notification string
  #[serde(rename = "notification")]
  notification: String,
  /// Tasks need to complete for this group
  #[serde(rename = "required_tasks")]
  required_tasks: Vec<i32>
}

impl GetOpportunitiesGroupsGroupIdOk {
  /// 200 ok object
  pub fn new(connected_groups: Vec<i32>, description: String, group_id: i32, name: String, notification: String, required_tasks: Vec<i32>) -> GetOpportunitiesGroupsGroupIdOk {
    GetOpportunitiesGroupsGroupIdOk {
      connected_groups: connected_groups,
      description: description,
      group_id: group_id,
      name: name,
      notification: notification,
      required_tasks: required_tasks
    }
  }

  pub fn set_connected_groups(&mut self, connected_groups: Vec<i32>) {
    self.connected_groups = connected_groups;
  }

  pub fn with_connected_groups(mut self, connected_groups: Vec<i32>) -> GetOpportunitiesGroupsGroupIdOk {
    self.connected_groups = connected_groups;
    self
  }

  pub fn connected_groups(&self) -> &Vec<i32> {
    &self.connected_groups
  }


  pub fn set_description(&mut self, description: String) {
    self.description = description;
  }

  pub fn with_description(mut self, description: String) -> GetOpportunitiesGroupsGroupIdOk {
    self.description = description;
    self
  }

  pub fn description(&self) -> &String {
    &self.description
  }


  pub fn set_group_id(&mut self, group_id: i32) {
    self.group_id = group_id;
  }

  pub fn with_group_id(mut self, group_id: i32) -> GetOpportunitiesGroupsGroupIdOk {
    self.group_id = group_id;
    self
  }

  pub fn group_id(&self) -> &i32 {
    &self.group_id
  }


  pub fn set_name(&mut self, name: String) {
    self.name = name;
  }

  pub fn with_name(mut self, name: String) -> GetOpportunitiesGroupsGroupIdOk {
    self.name = name;
    self
  }

  pub fn name(&self) -> &String {
    &self.name
  }


  pub fn set_notification(&mut self, notification: String) {
    self.notification = notification;
  }

  pub fn with_notification(mut self, notification: String) -> GetOpportunitiesGroupsGroupIdOk {
    self.notification = notification;
    self
  }

  pub fn notification(&self) -> &String {
    &self.notification
  }


  pub fn set_required_tasks(&mut self, required_tasks: Vec<i32>) {
    self.required_tasks = required_tasks;
  }

  pub fn with_required_tasks(mut self, required_tasks: Vec<i32>) -> GetOpportunitiesGroupsGroupIdOk {
    self.required_tasks = required_tasks;
    self
  }

  pub fn required_tasks(&self) -> &Vec<i32> {
    &self.required_tasks
  }


}



