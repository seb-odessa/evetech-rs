/* 
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * OpenAPI spec version: 1.17
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// PostFleetsFleetIdMembersInvitation : invitation object

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct PostFleetsFleetIdMembersInvitation {
  /// The character you want to invite
  #[serde(rename = "character_id")]
  character_id: i32,
  /// If a character is invited with the `fleet_commander` role, neither `wing_id` or `squad_id` should be specified. If a character is invited with the `wing_commander` role, only `wing_id` should be specified. If a character is invited with the `squad_commander` role, both `wing_id` and `squad_id` should be specified. If a character is invited with the `squad_member` role, `wing_id` and `squad_id` should either both be specified or not specified at all. If they aren’t specified, the invited character will join any squad with available positions.
  #[serde(rename = "role")]
  role: String,
  /// squad_id integer
  #[serde(rename = "squad_id")]
  squad_id: Option<i64>,
  /// wing_id integer
  #[serde(rename = "wing_id")]
  wing_id: Option<i64>
}

impl PostFleetsFleetIdMembersInvitation {
  /// invitation object
  pub fn new(character_id: i32, role: String) -> PostFleetsFleetIdMembersInvitation {
    PostFleetsFleetIdMembersInvitation {
      character_id: character_id,
      role: role,
      squad_id: None,
      wing_id: None
    }
  }

  pub fn set_character_id(&mut self, character_id: i32) {
    self.character_id = character_id;
  }

  pub fn with_character_id(mut self, character_id: i32) -> PostFleetsFleetIdMembersInvitation {
    self.character_id = character_id;
    self
  }

  pub fn character_id(&self) -> &i32 {
    &self.character_id
  }


  pub fn set_role(&mut self, role: String) {
    self.role = role;
  }

  pub fn with_role(mut self, role: String) -> PostFleetsFleetIdMembersInvitation {
    self.role = role;
    self
  }

  pub fn role(&self) -> &String {
    &self.role
  }


  pub fn set_squad_id(&mut self, squad_id: i64) {
    self.squad_id = Some(squad_id);
  }

  pub fn with_squad_id(mut self, squad_id: i64) -> PostFleetsFleetIdMembersInvitation {
    self.squad_id = Some(squad_id);
    self
  }

  pub fn squad_id(&self) -> Option<&i64> {
    self.squad_id.as_ref()
  }

  pub fn reset_squad_id(&mut self) {
    self.squad_id = None;
  }

  pub fn set_wing_id(&mut self, wing_id: i64) {
    self.wing_id = Some(wing_id);
  }

  pub fn with_wing_id(mut self, wing_id: i64) -> PostFleetsFleetIdMembersInvitation {
    self.wing_id = Some(wing_id);
    self
  }

  pub fn wing_id(&self) -> Option<&i64> {
    self.wing_id.as_ref()
  }

  pub fn reset_wing_id(&mut self) {
    self.wing_id = None;
  }

}



