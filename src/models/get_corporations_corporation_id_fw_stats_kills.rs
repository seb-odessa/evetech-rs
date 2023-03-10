/* 
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * OpenAPI spec version: 1.17
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// GetCorporationsCorporationIdFwStatsKills : Summary of kills done by the given corporation against enemy factions

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetCorporationsCorporationIdFwStatsKills {
  /// Last week's total number of kills by members of the given corporation against enemy factions
  #[serde(rename = "last_week")]
  last_week: i32,
  /// Total number of kills by members of the given corporation against enemy factions since the corporation enlisted
  #[serde(rename = "total")]
  total: i32,
  /// Yesterday's total number of kills by members of the given corporation against enemy factions
  #[serde(rename = "yesterday")]
  yesterday: i32
}

impl GetCorporationsCorporationIdFwStatsKills {
  /// Summary of kills done by the given corporation against enemy factions
  pub fn new(last_week: i32, total: i32, yesterday: i32) -> GetCorporationsCorporationIdFwStatsKills {
    GetCorporationsCorporationIdFwStatsKills {
      last_week: last_week,
      total: total,
      yesterday: yesterday
    }
  }

  pub fn set_last_week(&mut self, last_week: i32) {
    self.last_week = last_week;
  }

  pub fn with_last_week(mut self, last_week: i32) -> GetCorporationsCorporationIdFwStatsKills {
    self.last_week = last_week;
    self
  }

  pub fn last_week(&self) -> &i32 {
    &self.last_week
  }


  pub fn set_total(&mut self, total: i32) {
    self.total = total;
  }

  pub fn with_total(mut self, total: i32) -> GetCorporationsCorporationIdFwStatsKills {
    self.total = total;
    self
  }

  pub fn total(&self) -> &i32 {
    &self.total
  }


  pub fn set_yesterday(&mut self, yesterday: i32) {
    self.yesterday = yesterday;
  }

  pub fn with_yesterday(mut self, yesterday: i32) -> GetCorporationsCorporationIdFwStatsKills {
    self.yesterday = yesterday;
    self
  }

  pub fn yesterday(&self) -> &i32 {
    &self.yesterday
  }


}



