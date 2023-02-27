/* 
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * OpenAPI spec version: 1.17
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// GetFwLeaderboardsCorporationsOk : 200 ok object

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetFwLeaderboardsCorporationsOk {
  #[serde(rename = "kills")]
  kills: ::models::GetFwLeaderboardsCorporationsKills,
  #[serde(rename = "victory_points")]
  victory_points: ::models::GetFwLeaderboardsCorporationsVictoryPoints
}

impl GetFwLeaderboardsCorporationsOk {
  /// 200 ok object
  pub fn new(kills: ::models::GetFwLeaderboardsCorporationsKills, victory_points: ::models::GetFwLeaderboardsCorporationsVictoryPoints) -> GetFwLeaderboardsCorporationsOk {
    GetFwLeaderboardsCorporationsOk {
      kills: kills,
      victory_points: victory_points
    }
  }

  pub fn set_kills(&mut self, kills: ::models::GetFwLeaderboardsCorporationsKills) {
    self.kills = kills;
  }

  pub fn with_kills(mut self, kills: ::models::GetFwLeaderboardsCorporationsKills) -> GetFwLeaderboardsCorporationsOk {
    self.kills = kills;
    self
  }

  pub fn kills(&self) -> &::models::GetFwLeaderboardsCorporationsKills {
    &self.kills
  }


  pub fn set_victory_points(&mut self, victory_points: ::models::GetFwLeaderboardsCorporationsVictoryPoints) {
    self.victory_points = victory_points;
  }

  pub fn with_victory_points(mut self, victory_points: ::models::GetFwLeaderboardsCorporationsVictoryPoints) -> GetFwLeaderboardsCorporationsOk {
    self.victory_points = victory_points;
    self
  }

  pub fn victory_points(&self) -> &::models::GetFwLeaderboardsCorporationsVictoryPoints {
    &self.victory_points
  }


}



