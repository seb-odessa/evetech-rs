/* 
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * OpenAPI spec version: 1.17
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// GetFleetsFleetIdOk : 200 ok object

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetFleetsFleetIdOk {
  /// Is free-move enabled
  #[serde(rename = "is_free_move")]
  is_free_move: bool,
  /// Does the fleet have an active fleet advertisement
  #[serde(rename = "is_registered")]
  is_registered: bool,
  /// Is EVE Voice enabled
  #[serde(rename = "is_voice_enabled")]
  is_voice_enabled: bool,
  /// Fleet MOTD in CCP flavoured HTML
  #[serde(rename = "motd")]
  motd: String
}

impl GetFleetsFleetIdOk {
  /// 200 ok object
  pub fn new(is_free_move: bool, is_registered: bool, is_voice_enabled: bool, motd: String) -> GetFleetsFleetIdOk {
    GetFleetsFleetIdOk {
      is_free_move: is_free_move,
      is_registered: is_registered,
      is_voice_enabled: is_voice_enabled,
      motd: motd
    }
  }

  pub fn set_is_free_move(&mut self, is_free_move: bool) {
    self.is_free_move = is_free_move;
  }

  pub fn with_is_free_move(mut self, is_free_move: bool) -> GetFleetsFleetIdOk {
    self.is_free_move = is_free_move;
    self
  }

  pub fn is_free_move(&self) -> &bool {
    &self.is_free_move
  }


  pub fn set_is_registered(&mut self, is_registered: bool) {
    self.is_registered = is_registered;
  }

  pub fn with_is_registered(mut self, is_registered: bool) -> GetFleetsFleetIdOk {
    self.is_registered = is_registered;
    self
  }

  pub fn is_registered(&self) -> &bool {
    &self.is_registered
  }


  pub fn set_is_voice_enabled(&mut self, is_voice_enabled: bool) {
    self.is_voice_enabled = is_voice_enabled;
  }

  pub fn with_is_voice_enabled(mut self, is_voice_enabled: bool) -> GetFleetsFleetIdOk {
    self.is_voice_enabled = is_voice_enabled;
    self
  }

  pub fn is_voice_enabled(&self) -> &bool {
    &self.is_voice_enabled
  }


  pub fn set_motd(&mut self, motd: String) {
    self.motd = motd;
  }

  pub fn with_motd(mut self, motd: String) -> GetFleetsFleetIdOk {
    self.motd = motd;
    self
  }

  pub fn motd(&self) -> &String {
    &self.motd
  }


}


