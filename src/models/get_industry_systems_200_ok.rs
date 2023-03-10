/* 
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * OpenAPI spec version: 1.17
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// GetIndustrySystems200Ok : 200 ok object

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetIndustrySystems200Ok {
  /// cost_indices array
  #[serde(rename = "cost_indices")]
  cost_indices: Vec<::models::GetIndustrySystemsCostIndice>,
  /// solar_system_id integer
  #[serde(rename = "solar_system_id")]
  solar_system_id: i32
}

impl GetIndustrySystems200Ok {
  /// 200 ok object
  pub fn new(cost_indices: Vec<::models::GetIndustrySystemsCostIndice>, solar_system_id: i32) -> GetIndustrySystems200Ok {
    GetIndustrySystems200Ok {
      cost_indices: cost_indices,
      solar_system_id: solar_system_id
    }
  }

  pub fn set_cost_indices(&mut self, cost_indices: Vec<::models::GetIndustrySystemsCostIndice>) {
    self.cost_indices = cost_indices;
  }

  pub fn with_cost_indices(mut self, cost_indices: Vec<::models::GetIndustrySystemsCostIndice>) -> GetIndustrySystems200Ok {
    self.cost_indices = cost_indices;
    self
  }

  pub fn cost_indices(&self) -> &Vec<::models::GetIndustrySystemsCostIndice> {
    &self.cost_indices
  }


  pub fn set_solar_system_id(&mut self, solar_system_id: i32) {
    self.solar_system_id = solar_system_id;
  }

  pub fn with_solar_system_id(mut self, solar_system_id: i32) -> GetIndustrySystems200Ok {
    self.solar_system_id = solar_system_id;
    self
  }

  pub fn solar_system_id(&self) -> &i32 {
    &self.solar_system_id
  }


}



