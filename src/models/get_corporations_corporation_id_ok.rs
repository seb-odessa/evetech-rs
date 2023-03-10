/* 
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * OpenAPI spec version: 1.17
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// GetCorporationsCorporationIdOk : 200 ok object

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetCorporationsCorporationIdOk {
  /// ID of the alliance that corporation is a member of, if any
  #[serde(rename = "alliance_id")]
  alliance_id: Option<i32>,
  /// ceo_id integer
  #[serde(rename = "ceo_id")]
  ceo_id: i32,
  /// creator_id integer
  #[serde(rename = "creator_id")]
  creator_id: i32,
  /// date_founded string
  #[serde(rename = "date_founded")]
  date_founded: Option<String>,
  /// description string
  #[serde(rename = "description")]
  description: Option<String>,
  /// faction_id integer
  #[serde(rename = "faction_id")]
  faction_id: Option<i32>,
  /// home_station_id integer
  #[serde(rename = "home_station_id")]
  home_station_id: Option<i32>,
  /// member_count integer
  #[serde(rename = "member_count")]
  member_count: i32,
  /// the full name of the corporation
  #[serde(rename = "name")]
  name: String,
  /// shares integer
  #[serde(rename = "shares")]
  shares: Option<i64>,
  /// tax_rate number
  #[serde(rename = "tax_rate")]
  tax_rate: f32,
  /// the short name of the corporation
  #[serde(rename = "ticker")]
  ticker: String,
  /// url string
  #[serde(rename = "url")]
  url: Option<String>,
  /// war_eligible boolean
  #[serde(rename = "war_eligible")]
  war_eligible: Option<bool>
}

impl GetCorporationsCorporationIdOk {
  /// 200 ok object
  pub fn new(ceo_id: i32, creator_id: i32, member_count: i32, name: String, tax_rate: f32, ticker: String) -> GetCorporationsCorporationIdOk {
    GetCorporationsCorporationIdOk {
      alliance_id: None,
      ceo_id: ceo_id,
      creator_id: creator_id,
      date_founded: None,
      description: None,
      faction_id: None,
      home_station_id: None,
      member_count: member_count,
      name: name,
      shares: None,
      tax_rate: tax_rate,
      ticker: ticker,
      url: None,
      war_eligible: None
    }
  }

  pub fn set_alliance_id(&mut self, alliance_id: i32) {
    self.alliance_id = Some(alliance_id);
  }

  pub fn with_alliance_id(mut self, alliance_id: i32) -> GetCorporationsCorporationIdOk {
    self.alliance_id = Some(alliance_id);
    self
  }

  pub fn alliance_id(&self) -> Option<&i32> {
    self.alliance_id.as_ref()
  }

  pub fn reset_alliance_id(&mut self) {
    self.alliance_id = None;
  }

  pub fn set_ceo_id(&mut self, ceo_id: i32) {
    self.ceo_id = ceo_id;
  }

  pub fn with_ceo_id(mut self, ceo_id: i32) -> GetCorporationsCorporationIdOk {
    self.ceo_id = ceo_id;
    self
  }

  pub fn ceo_id(&self) -> &i32 {
    &self.ceo_id
  }


  pub fn set_creator_id(&mut self, creator_id: i32) {
    self.creator_id = creator_id;
  }

  pub fn with_creator_id(mut self, creator_id: i32) -> GetCorporationsCorporationIdOk {
    self.creator_id = creator_id;
    self
  }

  pub fn creator_id(&self) -> &i32 {
    &self.creator_id
  }


  pub fn set_date_founded(&mut self, date_founded: String) {
    self.date_founded = Some(date_founded);
  }

  pub fn with_date_founded(mut self, date_founded: String) -> GetCorporationsCorporationIdOk {
    self.date_founded = Some(date_founded);
    self
  }

  pub fn date_founded(&self) -> Option<&String> {
    self.date_founded.as_ref()
  }

  pub fn reset_date_founded(&mut self) {
    self.date_founded = None;
  }

  pub fn set_description(&mut self, description: String) {
    self.description = Some(description);
  }

  pub fn with_description(mut self, description: String) -> GetCorporationsCorporationIdOk {
    self.description = Some(description);
    self
  }

  pub fn description(&self) -> Option<&String> {
    self.description.as_ref()
  }

  pub fn reset_description(&mut self) {
    self.description = None;
  }

  pub fn set_faction_id(&mut self, faction_id: i32) {
    self.faction_id = Some(faction_id);
  }

  pub fn with_faction_id(mut self, faction_id: i32) -> GetCorporationsCorporationIdOk {
    self.faction_id = Some(faction_id);
    self
  }

  pub fn faction_id(&self) -> Option<&i32> {
    self.faction_id.as_ref()
  }

  pub fn reset_faction_id(&mut self) {
    self.faction_id = None;
  }

  pub fn set_home_station_id(&mut self, home_station_id: i32) {
    self.home_station_id = Some(home_station_id);
  }

  pub fn with_home_station_id(mut self, home_station_id: i32) -> GetCorporationsCorporationIdOk {
    self.home_station_id = Some(home_station_id);
    self
  }

  pub fn home_station_id(&self) -> Option<&i32> {
    self.home_station_id.as_ref()
  }

  pub fn reset_home_station_id(&mut self) {
    self.home_station_id = None;
  }

  pub fn set_member_count(&mut self, member_count: i32) {
    self.member_count = member_count;
  }

  pub fn with_member_count(mut self, member_count: i32) -> GetCorporationsCorporationIdOk {
    self.member_count = member_count;
    self
  }

  pub fn member_count(&self) -> &i32 {
    &self.member_count
  }


  pub fn set_name(&mut self, name: String) {
    self.name = name;
  }

  pub fn with_name(mut self, name: String) -> GetCorporationsCorporationIdOk {
    self.name = name;
    self
  }

  pub fn name(&self) -> &String {
    &self.name
  }


  pub fn set_shares(&mut self, shares: i64) {
    self.shares = Some(shares);
  }

  pub fn with_shares(mut self, shares: i64) -> GetCorporationsCorporationIdOk {
    self.shares = Some(shares);
    self
  }

  pub fn shares(&self) -> Option<&i64> {
    self.shares.as_ref()
  }

  pub fn reset_shares(&mut self) {
    self.shares = None;
  }

  pub fn set_tax_rate(&mut self, tax_rate: f32) {
    self.tax_rate = tax_rate;
  }

  pub fn with_tax_rate(mut self, tax_rate: f32) -> GetCorporationsCorporationIdOk {
    self.tax_rate = tax_rate;
    self
  }

  pub fn tax_rate(&self) -> &f32 {
    &self.tax_rate
  }


  pub fn set_ticker(&mut self, ticker: String) {
    self.ticker = ticker;
  }

  pub fn with_ticker(mut self, ticker: String) -> GetCorporationsCorporationIdOk {
    self.ticker = ticker;
    self
  }

  pub fn ticker(&self) -> &String {
    &self.ticker
  }


  pub fn set_url(&mut self, url: String) {
    self.url = Some(url);
  }

  pub fn with_url(mut self, url: String) -> GetCorporationsCorporationIdOk {
    self.url = Some(url);
    self
  }

  pub fn url(&self) -> Option<&String> {
    self.url.as_ref()
  }

  pub fn reset_url(&mut self) {
    self.url = None;
  }

  pub fn set_war_eligible(&mut self, war_eligible: bool) {
    self.war_eligible = Some(war_eligible);
  }

  pub fn with_war_eligible(mut self, war_eligible: bool) -> GetCorporationsCorporationIdOk {
    self.war_eligible = Some(war_eligible);
    self
  }

  pub fn war_eligible(&self) -> Option<&bool> {
    self.war_eligible.as_ref()
  }

  pub fn reset_war_eligible(&mut self) {
    self.war_eligible = None;
  }

}



