/* 
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * OpenAPI spec version: 1.17
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// GetSovereigntyCampaignsParticipant : participant object

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetSovereigntyCampaignsParticipant {
  /// alliance_id integer
  #[serde(rename = "alliance_id")]
  alliance_id: i32,
  /// score number
  #[serde(rename = "score")]
  score: f32
}

impl GetSovereigntyCampaignsParticipant {
  /// participant object
  pub fn new(alliance_id: i32, score: f32) -> GetSovereigntyCampaignsParticipant {
    GetSovereigntyCampaignsParticipant {
      alliance_id: alliance_id,
      score: score
    }
  }

  pub fn set_alliance_id(&mut self, alliance_id: i32) {
    self.alliance_id = alliance_id;
  }

  pub fn with_alliance_id(mut self, alliance_id: i32) -> GetSovereigntyCampaignsParticipant {
    self.alliance_id = alliance_id;
    self
  }

  pub fn alliance_id(&self) -> &i32 {
    &self.alliance_id
  }


  pub fn set_score(&mut self, score: f32) {
    self.score = score;
  }

  pub fn with_score(mut self, score: f32) -> GetSovereigntyCampaignsParticipant {
    self.score = score;
    self
  }

  pub fn score(&self) -> &f32 {
    &self.score
  }


}



