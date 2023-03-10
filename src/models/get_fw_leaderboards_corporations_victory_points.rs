/* 
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * OpenAPI spec version: 1.17
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// GetFwLeaderboardsCorporationsVictoryPoints : Top 10 rankings of corporations by victory points from yesterday, last week and in total

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetFwLeaderboardsCorporationsVictoryPoints {
  /// Top 10 ranking of corporations active in faction warfare by total victory points. A corporation is considered \"active\" if they have participated in faction warfare in the past 14 days
  #[serde(rename = "active_total")]
  active_total: Vec<::models::GetFwLeaderboardsCorporationsActiveTotalActiveTotal1>,
  /// Top 10 ranking of corporations by victory points in the past week
  #[serde(rename = "last_week")]
  last_week: Vec<::models::GetFwLeaderboardsCorporationsLastWeekLastWeek1>,
  /// Top 10 ranking of corporations by victory points in the past day
  #[serde(rename = "yesterday")]
  yesterday: Vec<::models::GetFwLeaderboardsCorporationsYesterdayYesterday1>
}

impl GetFwLeaderboardsCorporationsVictoryPoints {
  /// Top 10 rankings of corporations by victory points from yesterday, last week and in total
  pub fn new(active_total: Vec<::models::GetFwLeaderboardsCorporationsActiveTotalActiveTotal1>, last_week: Vec<::models::GetFwLeaderboardsCorporationsLastWeekLastWeek1>, yesterday: Vec<::models::GetFwLeaderboardsCorporationsYesterdayYesterday1>) -> GetFwLeaderboardsCorporationsVictoryPoints {
    GetFwLeaderboardsCorporationsVictoryPoints {
      active_total: active_total,
      last_week: last_week,
      yesterday: yesterday
    }
  }

  pub fn set_active_total(&mut self, active_total: Vec<::models::GetFwLeaderboardsCorporationsActiveTotalActiveTotal1>) {
    self.active_total = active_total;
  }

  pub fn with_active_total(mut self, active_total: Vec<::models::GetFwLeaderboardsCorporationsActiveTotalActiveTotal1>) -> GetFwLeaderboardsCorporationsVictoryPoints {
    self.active_total = active_total;
    self
  }

  pub fn active_total(&self) -> &Vec<::models::GetFwLeaderboardsCorporationsActiveTotalActiveTotal1> {
    &self.active_total
  }


  pub fn set_last_week(&mut self, last_week: Vec<::models::GetFwLeaderboardsCorporationsLastWeekLastWeek1>) {
    self.last_week = last_week;
  }

  pub fn with_last_week(mut self, last_week: Vec<::models::GetFwLeaderboardsCorporationsLastWeekLastWeek1>) -> GetFwLeaderboardsCorporationsVictoryPoints {
    self.last_week = last_week;
    self
  }

  pub fn last_week(&self) -> &Vec<::models::GetFwLeaderboardsCorporationsLastWeekLastWeek1> {
    &self.last_week
  }


  pub fn set_yesterday(&mut self, yesterday: Vec<::models::GetFwLeaderboardsCorporationsYesterdayYesterday1>) {
    self.yesterday = yesterday;
  }

  pub fn with_yesterday(mut self, yesterday: Vec<::models::GetFwLeaderboardsCorporationsYesterdayYesterday1>) -> GetFwLeaderboardsCorporationsVictoryPoints {
    self.yesterday = yesterday;
    self
  }

  pub fn yesterday(&self) -> &Vec<::models::GetFwLeaderboardsCorporationsYesterdayYesterday1> {
    &self.yesterday
  }


}



