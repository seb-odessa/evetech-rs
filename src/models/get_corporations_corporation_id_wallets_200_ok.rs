/* 
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * OpenAPI spec version: 1.17
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// GetCorporationsCorporationIdWallets200Ok : 200 ok object

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetCorporationsCorporationIdWallets200Ok {
  /// balance number
  #[serde(rename = "balance")]
  balance: f64,
  /// division integer
  #[serde(rename = "division")]
  division: i32
}

impl GetCorporationsCorporationIdWallets200Ok {
  /// 200 ok object
  pub fn new(balance: f64, division: i32) -> GetCorporationsCorporationIdWallets200Ok {
    GetCorporationsCorporationIdWallets200Ok {
      balance: balance,
      division: division
    }
  }

  pub fn set_balance(&mut self, balance: f64) {
    self.balance = balance;
  }

  pub fn with_balance(mut self, balance: f64) -> GetCorporationsCorporationIdWallets200Ok {
    self.balance = balance;
    self
  }

  pub fn balance(&self) -> &f64 {
    &self.balance
  }


  pub fn set_division(&mut self, division: i32) {
    self.division = division;
  }

  pub fn with_division(mut self, division: i32) -> GetCorporationsCorporationIdWallets200Ok {
    self.division = division;
    self
  }

  pub fn division(&self) -> &i32 {
    &self.division
  }


}



