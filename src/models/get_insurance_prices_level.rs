/* 
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * OpenAPI spec version: 1.17
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// GetInsurancePricesLevel : level object

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetInsurancePricesLevel {
  /// cost number
  #[serde(rename = "cost")]
  cost: f32,
  /// Localized insurance level
  #[serde(rename = "name")]
  name: String,
  /// payout number
  #[serde(rename = "payout")]
  payout: f32
}

impl GetInsurancePricesLevel {
  /// level object
  pub fn new(cost: f32, name: String, payout: f32) -> GetInsurancePricesLevel {
    GetInsurancePricesLevel {
      cost: cost,
      name: name,
      payout: payout
    }
  }

  pub fn set_cost(&mut self, cost: f32) {
    self.cost = cost;
  }

  pub fn with_cost(mut self, cost: f32) -> GetInsurancePricesLevel {
    self.cost = cost;
    self
  }

  pub fn cost(&self) -> &f32 {
    &self.cost
  }


  pub fn set_name(&mut self, name: String) {
    self.name = name;
  }

  pub fn with_name(mut self, name: String) -> GetInsurancePricesLevel {
    self.name = name;
    self
  }

  pub fn name(&self) -> &String {
    &self.name
  }


  pub fn set_payout(&mut self, payout: f32) {
    self.payout = payout;
  }

  pub fn with_payout(mut self, payout: f32) -> GetInsurancePricesLevel {
    self.payout = payout;
    self
  }

  pub fn payout(&self) -> &f32 {
    &self.payout
  }


}


