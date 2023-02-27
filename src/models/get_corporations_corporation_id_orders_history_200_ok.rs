/* 
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * OpenAPI spec version: 1.17
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// GetCorporationsCorporationIdOrdersHistory200Ok : 200 ok object

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetCorporationsCorporationIdOrdersHistory200Ok {
  /// Number of days the order was valid for (starting from the issued date). An order expires at time issued + duration
  #[serde(rename = "duration")]
  duration: i32,
  /// For buy orders, the amount of ISK in escrow
  #[serde(rename = "escrow")]
  escrow: Option<f64>,
  /// True if the order is a bid (buy) order
  #[serde(rename = "is_buy_order")]
  is_buy_order: Option<bool>,
  /// Date and time when this order was issued
  #[serde(rename = "issued")]
  issued: String,
  /// The character who issued this order
  #[serde(rename = "issued_by")]
  issued_by: Option<i32>,
  /// ID of the location where order was placed
  #[serde(rename = "location_id")]
  location_id: i64,
  /// For buy orders, the minimum quantity that will be accepted in a matching sell order
  #[serde(rename = "min_volume")]
  min_volume: Option<i32>,
  /// Unique order ID
  #[serde(rename = "order_id")]
  order_id: i64,
  /// Cost per unit for this order
  #[serde(rename = "price")]
  price: f64,
  /// Valid order range, numbers are ranges in jumps
  #[serde(rename = "range")]
  range: String,
  /// ID of the region where order was placed
  #[serde(rename = "region_id")]
  region_id: i32,
  /// Current order state
  #[serde(rename = "state")]
  state: String,
  /// The type ID of the item transacted in this order
  #[serde(rename = "type_id")]
  type_id: i32,
  /// Quantity of items still required or offered
  #[serde(rename = "volume_remain")]
  volume_remain: i32,
  /// Quantity of items required or offered at time order was placed
  #[serde(rename = "volume_total")]
  volume_total: i32,
  /// The corporation wallet division used for this order
  #[serde(rename = "wallet_division")]
  wallet_division: i32
}

impl GetCorporationsCorporationIdOrdersHistory200Ok {
  /// 200 ok object
  pub fn new(duration: i32, issued: String, location_id: i64, order_id: i64, price: f64, range: String, region_id: i32, state: String, type_id: i32, volume_remain: i32, volume_total: i32, wallet_division: i32) -> GetCorporationsCorporationIdOrdersHistory200Ok {
    GetCorporationsCorporationIdOrdersHistory200Ok {
      duration: duration,
      escrow: None,
      is_buy_order: None,
      issued: issued,
      issued_by: None,
      location_id: location_id,
      min_volume: None,
      order_id: order_id,
      price: price,
      range: range,
      region_id: region_id,
      state: state,
      type_id: type_id,
      volume_remain: volume_remain,
      volume_total: volume_total,
      wallet_division: wallet_division
    }
  }

  pub fn set_duration(&mut self, duration: i32) {
    self.duration = duration;
  }

  pub fn with_duration(mut self, duration: i32) -> GetCorporationsCorporationIdOrdersHistory200Ok {
    self.duration = duration;
    self
  }

  pub fn duration(&self) -> &i32 {
    &self.duration
  }


  pub fn set_escrow(&mut self, escrow: f64) {
    self.escrow = Some(escrow);
  }

  pub fn with_escrow(mut self, escrow: f64) -> GetCorporationsCorporationIdOrdersHistory200Ok {
    self.escrow = Some(escrow);
    self
  }

  pub fn escrow(&self) -> Option<&f64> {
    self.escrow.as_ref()
  }

  pub fn reset_escrow(&mut self) {
    self.escrow = None;
  }

  pub fn set_is_buy_order(&mut self, is_buy_order: bool) {
    self.is_buy_order = Some(is_buy_order);
  }

  pub fn with_is_buy_order(mut self, is_buy_order: bool) -> GetCorporationsCorporationIdOrdersHistory200Ok {
    self.is_buy_order = Some(is_buy_order);
    self
  }

  pub fn is_buy_order(&self) -> Option<&bool> {
    self.is_buy_order.as_ref()
  }

  pub fn reset_is_buy_order(&mut self) {
    self.is_buy_order = None;
  }

  pub fn set_issued(&mut self, issued: String) {
    self.issued = issued;
  }

  pub fn with_issued(mut self, issued: String) -> GetCorporationsCorporationIdOrdersHistory200Ok {
    self.issued = issued;
    self
  }

  pub fn issued(&self) -> &String {
    &self.issued
  }


  pub fn set_issued_by(&mut self, issued_by: i32) {
    self.issued_by = Some(issued_by);
  }

  pub fn with_issued_by(mut self, issued_by: i32) -> GetCorporationsCorporationIdOrdersHistory200Ok {
    self.issued_by = Some(issued_by);
    self
  }

  pub fn issued_by(&self) -> Option<&i32> {
    self.issued_by.as_ref()
  }

  pub fn reset_issued_by(&mut self) {
    self.issued_by = None;
  }

  pub fn set_location_id(&mut self, location_id: i64) {
    self.location_id = location_id;
  }

  pub fn with_location_id(mut self, location_id: i64) -> GetCorporationsCorporationIdOrdersHistory200Ok {
    self.location_id = location_id;
    self
  }

  pub fn location_id(&self) -> &i64 {
    &self.location_id
  }


  pub fn set_min_volume(&mut self, min_volume: i32) {
    self.min_volume = Some(min_volume);
  }

  pub fn with_min_volume(mut self, min_volume: i32) -> GetCorporationsCorporationIdOrdersHistory200Ok {
    self.min_volume = Some(min_volume);
    self
  }

  pub fn min_volume(&self) -> Option<&i32> {
    self.min_volume.as_ref()
  }

  pub fn reset_min_volume(&mut self) {
    self.min_volume = None;
  }

  pub fn set_order_id(&mut self, order_id: i64) {
    self.order_id = order_id;
  }

  pub fn with_order_id(mut self, order_id: i64) -> GetCorporationsCorporationIdOrdersHistory200Ok {
    self.order_id = order_id;
    self
  }

  pub fn order_id(&self) -> &i64 {
    &self.order_id
  }


  pub fn set_price(&mut self, price: f64) {
    self.price = price;
  }

  pub fn with_price(mut self, price: f64) -> GetCorporationsCorporationIdOrdersHistory200Ok {
    self.price = price;
    self
  }

  pub fn price(&self) -> &f64 {
    &self.price
  }


  pub fn set_range(&mut self, range: String) {
    self.range = range;
  }

  pub fn with_range(mut self, range: String) -> GetCorporationsCorporationIdOrdersHistory200Ok {
    self.range = range;
    self
  }

  pub fn range(&self) -> &String {
    &self.range
  }


  pub fn set_region_id(&mut self, region_id: i32) {
    self.region_id = region_id;
  }

  pub fn with_region_id(mut self, region_id: i32) -> GetCorporationsCorporationIdOrdersHistory200Ok {
    self.region_id = region_id;
    self
  }

  pub fn region_id(&self) -> &i32 {
    &self.region_id
  }


  pub fn set_state(&mut self, state: String) {
    self.state = state;
  }

  pub fn with_state(mut self, state: String) -> GetCorporationsCorporationIdOrdersHistory200Ok {
    self.state = state;
    self
  }

  pub fn state(&self) -> &String {
    &self.state
  }


  pub fn set_type_id(&mut self, type_id: i32) {
    self.type_id = type_id;
  }

  pub fn with_type_id(mut self, type_id: i32) -> GetCorporationsCorporationIdOrdersHistory200Ok {
    self.type_id = type_id;
    self
  }

  pub fn type_id(&self) -> &i32 {
    &self.type_id
  }


  pub fn set_volume_remain(&mut self, volume_remain: i32) {
    self.volume_remain = volume_remain;
  }

  pub fn with_volume_remain(mut self, volume_remain: i32) -> GetCorporationsCorporationIdOrdersHistory200Ok {
    self.volume_remain = volume_remain;
    self
  }

  pub fn volume_remain(&self) -> &i32 {
    &self.volume_remain
  }


  pub fn set_volume_total(&mut self, volume_total: i32) {
    self.volume_total = volume_total;
  }

  pub fn with_volume_total(mut self, volume_total: i32) -> GetCorporationsCorporationIdOrdersHistory200Ok {
    self.volume_total = volume_total;
    self
  }

  pub fn volume_total(&self) -> &i32 {
    &self.volume_total
  }


  pub fn set_wallet_division(&mut self, wallet_division: i32) {
    self.wallet_division = wallet_division;
  }

  pub fn with_wallet_division(mut self, wallet_division: i32) -> GetCorporationsCorporationIdOrdersHistory200Ok {
    self.wallet_division = wallet_division;
    self
  }

  pub fn wallet_division(&self) -> &i32 {
    &self.wallet_division
  }


}



