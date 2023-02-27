/* 
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * OpenAPI spec version: 1.17
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// GetContractsPublicRegionId200Ok : 200 ok object

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetContractsPublicRegionId200Ok {
  /// Buyout price (for Auctions only)
  #[serde(rename = "buyout")]
  buyout: Option<f64>,
  /// Collateral price (for Couriers only)
  #[serde(rename = "collateral")]
  collateral: Option<f64>,
  /// contract_id integer
  #[serde(rename = "contract_id")]
  contract_id: i32,
  /// Expiration date of the contract
  #[serde(rename = "date_expired")]
  date_expired: String,
  /// Сreation date of the contract
  #[serde(rename = "date_issued")]
  date_issued: String,
  /// Number of days to perform the contract
  #[serde(rename = "days_to_complete")]
  days_to_complete: Option<i32>,
  /// End location ID (for Couriers contract)
  #[serde(rename = "end_location_id")]
  end_location_id: Option<i64>,
  /// true if the contract was issued on behalf of the issuer's corporation
  #[serde(rename = "for_corporation")]
  for_corporation: Option<bool>,
  /// Character's corporation ID for the issuer
  #[serde(rename = "issuer_corporation_id")]
  issuer_corporation_id: i32,
  /// Character ID for the issuer
  #[serde(rename = "issuer_id")]
  issuer_id: i32,
  /// Price of contract (for ItemsExchange and Auctions)
  #[serde(rename = "price")]
  price: Option<f64>,
  /// Remuneration for contract (for Couriers only)
  #[serde(rename = "reward")]
  reward: Option<f64>,
  /// Start location ID (for Couriers contract)
  #[serde(rename = "start_location_id")]
  start_location_id: Option<i64>,
  /// Title of the contract
  #[serde(rename = "title")]
  title: Option<String>,
  /// Type of the contract
  #[serde(rename = "type")]
  _type: String,
  /// Volume of items in the contract
  #[serde(rename = "volume")]
  volume: Option<f64>
}

impl GetContractsPublicRegionId200Ok {
  /// 200 ok object
  pub fn new(contract_id: i32, date_expired: String, date_issued: String, issuer_corporation_id: i32, issuer_id: i32, _type: String) -> GetContractsPublicRegionId200Ok {
    GetContractsPublicRegionId200Ok {
      buyout: None,
      collateral: None,
      contract_id: contract_id,
      date_expired: date_expired,
      date_issued: date_issued,
      days_to_complete: None,
      end_location_id: None,
      for_corporation: None,
      issuer_corporation_id: issuer_corporation_id,
      issuer_id: issuer_id,
      price: None,
      reward: None,
      start_location_id: None,
      title: None,
      _type: _type,
      volume: None
    }
  }

  pub fn set_buyout(&mut self, buyout: f64) {
    self.buyout = Some(buyout);
  }

  pub fn with_buyout(mut self, buyout: f64) -> GetContractsPublicRegionId200Ok {
    self.buyout = Some(buyout);
    self
  }

  pub fn buyout(&self) -> Option<&f64> {
    self.buyout.as_ref()
  }

  pub fn reset_buyout(&mut self) {
    self.buyout = None;
  }

  pub fn set_collateral(&mut self, collateral: f64) {
    self.collateral = Some(collateral);
  }

  pub fn with_collateral(mut self, collateral: f64) -> GetContractsPublicRegionId200Ok {
    self.collateral = Some(collateral);
    self
  }

  pub fn collateral(&self) -> Option<&f64> {
    self.collateral.as_ref()
  }

  pub fn reset_collateral(&mut self) {
    self.collateral = None;
  }

  pub fn set_contract_id(&mut self, contract_id: i32) {
    self.contract_id = contract_id;
  }

  pub fn with_contract_id(mut self, contract_id: i32) -> GetContractsPublicRegionId200Ok {
    self.contract_id = contract_id;
    self
  }

  pub fn contract_id(&self) -> &i32 {
    &self.contract_id
  }


  pub fn set_date_expired(&mut self, date_expired: String) {
    self.date_expired = date_expired;
  }

  pub fn with_date_expired(mut self, date_expired: String) -> GetContractsPublicRegionId200Ok {
    self.date_expired = date_expired;
    self
  }

  pub fn date_expired(&self) -> &String {
    &self.date_expired
  }


  pub fn set_date_issued(&mut self, date_issued: String) {
    self.date_issued = date_issued;
  }

  pub fn with_date_issued(mut self, date_issued: String) -> GetContractsPublicRegionId200Ok {
    self.date_issued = date_issued;
    self
  }

  pub fn date_issued(&self) -> &String {
    &self.date_issued
  }


  pub fn set_days_to_complete(&mut self, days_to_complete: i32) {
    self.days_to_complete = Some(days_to_complete);
  }

  pub fn with_days_to_complete(mut self, days_to_complete: i32) -> GetContractsPublicRegionId200Ok {
    self.days_to_complete = Some(days_to_complete);
    self
  }

  pub fn days_to_complete(&self) -> Option<&i32> {
    self.days_to_complete.as_ref()
  }

  pub fn reset_days_to_complete(&mut self) {
    self.days_to_complete = None;
  }

  pub fn set_end_location_id(&mut self, end_location_id: i64) {
    self.end_location_id = Some(end_location_id);
  }

  pub fn with_end_location_id(mut self, end_location_id: i64) -> GetContractsPublicRegionId200Ok {
    self.end_location_id = Some(end_location_id);
    self
  }

  pub fn end_location_id(&self) -> Option<&i64> {
    self.end_location_id.as_ref()
  }

  pub fn reset_end_location_id(&mut self) {
    self.end_location_id = None;
  }

  pub fn set_for_corporation(&mut self, for_corporation: bool) {
    self.for_corporation = Some(for_corporation);
  }

  pub fn with_for_corporation(mut self, for_corporation: bool) -> GetContractsPublicRegionId200Ok {
    self.for_corporation = Some(for_corporation);
    self
  }

  pub fn for_corporation(&self) -> Option<&bool> {
    self.for_corporation.as_ref()
  }

  pub fn reset_for_corporation(&mut self) {
    self.for_corporation = None;
  }

  pub fn set_issuer_corporation_id(&mut self, issuer_corporation_id: i32) {
    self.issuer_corporation_id = issuer_corporation_id;
  }

  pub fn with_issuer_corporation_id(mut self, issuer_corporation_id: i32) -> GetContractsPublicRegionId200Ok {
    self.issuer_corporation_id = issuer_corporation_id;
    self
  }

  pub fn issuer_corporation_id(&self) -> &i32 {
    &self.issuer_corporation_id
  }


  pub fn set_issuer_id(&mut self, issuer_id: i32) {
    self.issuer_id = issuer_id;
  }

  pub fn with_issuer_id(mut self, issuer_id: i32) -> GetContractsPublicRegionId200Ok {
    self.issuer_id = issuer_id;
    self
  }

  pub fn issuer_id(&self) -> &i32 {
    &self.issuer_id
  }


  pub fn set_price(&mut self, price: f64) {
    self.price = Some(price);
  }

  pub fn with_price(mut self, price: f64) -> GetContractsPublicRegionId200Ok {
    self.price = Some(price);
    self
  }

  pub fn price(&self) -> Option<&f64> {
    self.price.as_ref()
  }

  pub fn reset_price(&mut self) {
    self.price = None;
  }

  pub fn set_reward(&mut self, reward: f64) {
    self.reward = Some(reward);
  }

  pub fn with_reward(mut self, reward: f64) -> GetContractsPublicRegionId200Ok {
    self.reward = Some(reward);
    self
  }

  pub fn reward(&self) -> Option<&f64> {
    self.reward.as_ref()
  }

  pub fn reset_reward(&mut self) {
    self.reward = None;
  }

  pub fn set_start_location_id(&mut self, start_location_id: i64) {
    self.start_location_id = Some(start_location_id);
  }

  pub fn with_start_location_id(mut self, start_location_id: i64) -> GetContractsPublicRegionId200Ok {
    self.start_location_id = Some(start_location_id);
    self
  }

  pub fn start_location_id(&self) -> Option<&i64> {
    self.start_location_id.as_ref()
  }

  pub fn reset_start_location_id(&mut self) {
    self.start_location_id = None;
  }

  pub fn set_title(&mut self, title: String) {
    self.title = Some(title);
  }

  pub fn with_title(mut self, title: String) -> GetContractsPublicRegionId200Ok {
    self.title = Some(title);
    self
  }

  pub fn title(&self) -> Option<&String> {
    self.title.as_ref()
  }

  pub fn reset_title(&mut self) {
    self.title = None;
  }

  pub fn set__type(&mut self, _type: String) {
    self._type = _type;
  }

  pub fn with__type(mut self, _type: String) -> GetContractsPublicRegionId200Ok {
    self._type = _type;
    self
  }

  pub fn _type(&self) -> &String {
    &self._type
  }


  pub fn set_volume(&mut self, volume: f64) {
    self.volume = Some(volume);
  }

  pub fn with_volume(mut self, volume: f64) -> GetContractsPublicRegionId200Ok {
    self.volume = Some(volume);
    self
  }

  pub fn volume(&self) -> Option<&f64> {
    self.volume.as_ref()
  }

  pub fn reset_volume(&mut self) {
    self.volume = None;
  }

}



