/* 
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * OpenAPI spec version: 1.17
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// GetFwStatsVictoryPoints : Summary of victory points gained for the given faction

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetFwStatsVictoryPoints {
  /// Last week's victory points gained
  #[serde(rename = "last_week")]
  last_week: i32,
  /// Total victory points gained since faction warfare began
  #[serde(rename = "total")]
  total: i32,
  /// Yesterday's victory points gained
  #[serde(rename = "yesterday")]
  yesterday: i32
}

impl GetFwStatsVictoryPoints {
  /// Summary of victory points gained for the given faction
  pub fn new(last_week: i32, total: i32, yesterday: i32) -> GetFwStatsVictoryPoints {
    GetFwStatsVictoryPoints {
      last_week: last_week,
      total: total,
      yesterday: yesterday
    }
  }

  pub fn set_last_week(&mut self, last_week: i32) {
    self.last_week = last_week;
  }

  pub fn with_last_week(mut self, last_week: i32) -> GetFwStatsVictoryPoints {
    self.last_week = last_week;
    self
  }

  pub fn last_week(&self) -> &i32 {
    &self.last_week
  }


  pub fn set_total(&mut self, total: i32) {
    self.total = total;
  }

  pub fn with_total(mut self, total: i32) -> GetFwStatsVictoryPoints {
    self.total = total;
    self
  }

  pub fn total(&self) -> &i32 {
    &self.total
  }


  pub fn set_yesterday(&mut self, yesterday: i32) {
    self.yesterday = yesterday;
  }

  pub fn with_yesterday(mut self, yesterday: i32) -> GetFwStatsVictoryPoints {
    self.yesterday = yesterday;
    self
  }

  pub fn yesterday(&self) -> &i32 {
    &self.yesterday
  }


}



